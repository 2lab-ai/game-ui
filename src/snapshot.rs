//! 창 없이 그린다 — egui 한 장면을 오프스크린 텍스처에 그려 RGBA로 읽어 온다 (갤러리 · 시각 회귀 시험).
pub struct Snapshot { device: wgpu::Device, queue: wgpu::Queue, renderer: egui_wgpu::Renderer, pub ctx: egui::Context, pub adapter: String }

const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

impl Snapshot {
    pub fn new() -> Result<Self, String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, force_fallback_adapter: false, compatible_surface: None, apply_limit_buckets: false })).map_err(|e| format!("어댑터 없음: {e}"))?;
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor { label: Some("game-ui snapshot"), required_limits: wgpu::Limits::default().using_resolution(adapter.limits()), ..Default::default() })).map_err(|e| format!("장치 없음: {e}"))?;
        let info = adapter.get_info();
        let renderer = egui_wgpu::Renderer::new(&device, FORMAT, egui_wgpu::RendererOptions::default());
        let ctx = egui::Context::default(); crate::fonts::install(&ctx);
        Ok(Self { device, queue, renderer, ctx, adapter: format!("{} · {:?}", info.name, info.backend) })
    }

    /// size(논리 점) × ppp 픽셀로 그린다. 애니메이션이 자리 잡도록 frames번 돌리고(0.5초 간격) 마지막 장면을 읽는다.
    pub fn render(&mut self, size: [f32; 2], ppp: f32, clear: [f64; 4], frames: usize, ui: &mut dyn FnMut(&mut egui::Ui)) -> Result<(u32, u32, Vec<u8>), String> {
        let (w, h) = ((size[0] * ppp).round() as u32, (size[1] * ppp).round() as u32);
        let tex = self.device.create_texture(&wgpu::TextureDescriptor { label: Some("snap"), size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: FORMAT, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
        let view = tex.create_view(&Default::default());
        for i in 0..frames.max(1) {
            let mut raw = egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(size[0], size[1]))), time: Some(i as f64 * 0.5), predicted_dt: 0.5, ..Default::default() };
            raw.viewports.entry(egui::ViewportId::ROOT).or_default().native_pixels_per_point = Some(ppp);
            let out = self.ctx.run_ui(raw, |u| ui(u));
            let jobs = self.ctx.tessellate(out.shapes, out.pixels_per_point);
            let sd = egui_wgpu::ScreenDescriptor { size_in_pixels: [w, h], pixels_per_point: out.pixels_per_point };
            for (id, ds) in &out.textures_delta.set { for d in ds { self.renderer.update_texture(&self.device, &self.queue, *id, d); } }
            let mut enc = self.device.create_command_encoder(&Default::default());
            let extra = self.renderer.update_buffers(&self.device, &self.queue, &mut enc, &jobs, &sd);
            {
                let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("snap"), color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &view, depth_slice: None, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: clear[0], g: clear[1], b: clear[2], a: clear[3] }), store: wgpu::StoreOp::Store } })], depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None, multiview_mask: None }).forget_lifetime();
                self.renderer.render(&mut pass, &jobs, &sd);
            }
            self.queue.submit(extra.into_iter().chain(std::iter::once(enc.finish())));
            for id in &out.textures_delta.free { self.renderer.free_texture(id); }
        }
        let bpr = (w * 4).div_ceil(256) * 256;
        let buf = self.device.create_buffer(&wgpu::BufferDescriptor { label: Some("readback"), size: (bpr * h) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let mut enc = self.device.create_command_encoder(&Default::default());
        enc.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All }, wgpu::TexelCopyBufferInfo { buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(h) } }, wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
        self.queue.submit([enc.finish()]);
        let slice = buf.slice(..); slice.map_async(wgpu::MapMode::Read, |_| {});
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| format!("poll: {e}"))?;
        let data = slice.get_mapped_range().map_err(|e| format!("map: {e:?}"))?;
        let mut px = Vec::with_capacity((w * h * 4) as usize);
        for row in 0..h { let s = (row * bpr) as usize; px.extend_from_slice(&data[s..s + (w * 4) as usize]); }
        drop(data); buf.unmap();
        for p in px.chunks_exact_mut(4) { p[3] = 255; }
        Ok((w, h, px))
    }
}

pub fn save_png(path: &std::path::Path, w: u32, h: u32, px: &[u8]) -> Result<(), String> {
    let f = std::fs::File::create(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(f), w, h); enc.set_color(png::ColorType::Rgba); enc.set_depth(png::BitDepth::Eight);
    enc.write_header().and_then(|mut wr| wr.write_image_data(px)).map_err(|e| format!("png: {e}"))
}
