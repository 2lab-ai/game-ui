//! 그리기 도구 — 그라데이션 · 판 질감(원작처럼 u 한 칸마다 다른 잡음, NEAREST로 또렷하게) · 값 잡음 결 · 베벨 · 점선 · u 사각형.
use egui::{Color32, ColorImage, Context, Mesh, Painter, Pos2, Rect, Shape, Stroke, TextureHandle, TextureOptions};

/// 세로 그라데이션 사각형 (위 → 아래)
pub fn vgrad(p: &Painter, r: Rect, top: Color32, bottom: Color32) {
    let mut m = Mesh::default();
    m.colored_vertex(r.left_top(), top); m.colored_vertex(r.right_top(), top); m.colored_vertex(r.left_bottom(), bottom); m.colored_vertex(r.right_bottom(), bottom);
    m.add_triangle(0, 1, 2); m.add_triangle(1, 3, 2); p.add(Shape::mesh(m));
}
/// 가로 그라데이션 (왼 → 오른)
pub fn hgrad(p: &Painter, r: Rect, left: Color32, right: Color32) {
    let mut m = Mesh::default();
    m.colored_vertex(r.left_top(), left); m.colored_vertex(r.right_top(), right); m.colored_vertex(r.left_bottom(), left); m.colored_vertex(r.right_bottom(), right);
    m.add_triangle(0, 1, 2); m.add_triangle(1, 3, 2); p.add(Shape::mesh(m));
}
/// 여러 단계 세로 그라데이션 — stops = (0..1 위치, 색)
pub fn vgrad_stops(p: &Painter, r: Rect, stops: &[(f32, Color32)]) {
    for w in stops.windows(2) { let (a, b) = (w[0], w[1]); vgrad(p, Rect::from_min_max(Pos2::new(r.min.x, r.min.y + r.height() * a.0), Pos2::new(r.max.x, r.min.y + r.height() * b.0)), a.1, b.1); }
}

/// 질감 종류 — 같은 종류는 한 번만 만든다
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Grain { Parchment, Stone, Cloth }

fn hash(x: i32, y: i32, seed: u32) -> f32 { let mut h = (x as u32).wrapping_mul(374761393) ^ (y as u32).wrapping_mul(668265263) ^ seed.wrapping_mul(2246822519); h = (h ^ (h >> 13)).wrapping_mul(1274126177); ((h ^ (h >> 16)) & 0xffff) as f32 / 65535.0 }
fn vnoise(x: f32, y: f32, seed: u32, period: i32) -> f32 {
    let (xi, yi) = (x.floor() as i32, y.floor() as i32); let (fx, fy) = (x - xi as f32, y - yi as f32);
    let w = |a: i32, b: i32| hash(a.rem_euclid(period), b.rem_euclid(period), seed);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let a = w(xi, yi) + (w(xi + 1, yi) - w(xi, yi)) * sx; let b = w(xi, yi + 1) + (w(xi + 1, yi + 1) - w(xi, yi + 1)) * sx; a + (b - a) * sy
}
/// 이음매 없이 이어 붙는(타일) 회색 잡음 — 0..255, 밝기 변조용 (색은 칠할 때 곱한다)
fn grain_image(kind: Grain, n: usize) -> ColorImage {
    let mut px = vec![Color32::WHITE; n * n];
    for y in 0..n { for x in 0..n {
        let (u, v) = (x as f32, y as f32);
        let v_ = match kind {
            // 양피지: 큰 얼룩 + 잔 섬유 결 (가로로 늘인 잡음)
            Grain::Parchment => { let big = vnoise(u / 32.0, v / 32.0, 7, (n / 32) as i32) * 0.55 + vnoise(u / 12.0, v / 12.0, 11, (n / 12).max(1) as i32) * 0.3; let fiber = vnoise(u / 6.0, v / 1.6, 23, (n / 6) as i32) * 0.15; big + fiber }
            // 돌: 반점 + 미세 입자
            Grain::Stone => { let a = vnoise(u / 16.0, v / 16.0, 3, (n / 16) as i32) * 0.5 + vnoise(u / 5.0, v / 5.0, 5, (n / 5).max(1) as i32) * 0.3 + hash(x as i32, y as i32, 9) * 0.2; a }
            // 천: 가로세로 결
            Grain::Cloth => { let a = ((u * 1.6).sin() * 0.5 + 0.5) * 0.25 + ((v * 1.6).sin() * 0.5 + 0.5) * 0.25 + hash(x as i32, y as i32, 13) * 0.5; a }
        };
        let l = (205.0 + (v_ - 0.5) * 100.0).clamp(0.0, 255.0) as u8;
        px[y * n + x] = Color32::from_rgb(l, l, l);
    } }
    ColorImage::new([n, n], px)
}
/// 질감 텍스처 (컨텍스트에 캐시)
pub fn grain(ctx: &Context, kind: Grain) -> TextureHandle {
    let id = egui::Id::unique(("game-ui-grain", kind as u8));
    if let Some(t) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) { return t; }
    let t = ctx.load_texture(format!("grain-{kind:?}"), grain_image(kind, 128), TextureOptions { magnification: egui::TextureFilter::Linear, minification: egui::TextureFilter::Linear, wrap_mode: egui::TextureWrapMode::Repeat, mipmap_mode: None });
    ctx.data_mut(|d| d.insert_temp(id, t.clone())); t
}
/// 질감을 rect에 tint로 곱해 깐다 (scale = 질감 한 칸이 몇 점인지)
pub fn grain_rect(p: &Painter, r: Rect, kind: Grain, tint: Color32, scale: f32) {
    let t = grain(p.ctx(), kind);
    let uv = Rect::from_min_max(Pos2::new(r.min.x / (128.0 * scale), r.min.y / (128.0 * scale)), Pos2::new(r.max.x / (128.0 * scale), r.max.y / (128.0 * scale)));
    p.image(t.id(), r, uv, tint);
}

/// 베벨: 위·왼은 밝게, 아래·오른은 어둡게 (inset이면 반대)
pub fn bevel(p: &Painter, r: Rect, w: f32, light: Color32, dark: Color32, inset: bool) {
    let (a, b) = if inset { (dark, light) } else { (light, dark) };
    let h = w / 2.0;
    p.line_segment([Pos2::new(r.min.x, r.min.y + h), Pos2::new(r.max.x - w, r.min.y + h)], Stroke::new(w, a));
    p.line_segment([Pos2::new(r.min.x + h, r.min.y), Pos2::new(r.min.x + h, r.max.y - w)], Stroke::new(w, a));
    p.line_segment([Pos2::new(r.min.x + w, r.max.y - h), Pos2::new(r.max.x, r.max.y - h)], Stroke::new(w, b));
    p.line_segment([Pos2::new(r.max.x - h, r.min.y + w), Pos2::new(r.max.x - h, r.max.y)], Stroke::new(w, b));
}
/// 사각 테두리 한 줄 (선 안쪽으로)
pub fn rule(p: &Painter, r: Rect, w: f32, c: Color32) { p.rect_stroke(r.shrink(w / 2.0), 0.0, Stroke::new(w, c), egui::StrokeKind::Middle); }
/// 점선 (가로)
pub fn dotted_h(p: &Painter, y: f32, x0: f32, x1: f32, step: f32, c: Color32) { let mut x = x0; while x < x1 { p.rect_filled(Rect::from_min_size(Pos2::new(x, y), egui::vec2(step * 0.5, 1.0)), 0.0, c); x += step; } }

/// 판 질감 종류 (리서치 §6.2 표의 값)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tex { Plate, Slate, Page, Dialog, Parchment }

/// 64×64 이음매 없는 판 질감 — 칸마다 ±px_amp 잡음 + 16칸 주기 얼룩 + 비스듬한 섬유 (리서치 §6.2 bake_plate)
fn bake_plate(base: [u8; 3], seed: u32, px_amp: i32, blot_amp: f32, fiber_pct: f32, cell: usize) -> ColorImage {
    const N: usize = 64;
    let mut r = seed.max(1);
    let mut rnd = || { r ^= r << 13; r ^= r >> 17; r ^= r << 5; r };
    let cells = N / cell;
    let mut g = vec![vec![0f32; cells + 1]; cells + 1];
    for row in g.iter_mut().take(cells) { for v in row.iter_mut().take(cells) { *v = (rnd() % 1000) as f32 / 500.0 - 1.0; } }
    for i in 0..=cells { g[cells][i] = g[0][i % cells]; g[i][cells] = g[i % cells][0]; }
    let mut img = vec![Color32::BLACK; N * N];
    let s = |t: f32| t * t * (3.0 - 2.0 * t);
    for y in 0..N { for x in 0..N {
        let (fx, fy) = (x as f32 / cell as f32, y as f32 / cell as f32);
        let (ix, iy) = (fx as usize, fy as usize); let (tx, ty) = (fx.fract(), fy.fract());
        let a = g[iy][ix] + (g[iy][ix + 1] - g[iy][ix]) * s(tx);
        let b = g[iy + 1][ix] + (g[iy + 1][ix + 1] - g[iy + 1][ix]) * s(tx);
        let blot = (a + (b - a) * s(ty)) * blot_amp;
        let px = if px_amp > 0 { (rnd() % (2 * px_amp as u32 + 1)) as i32 - px_amp } else { 0 };
        let d = px as f32 + blot;
        let c = |v: u8| (v as f32 + d).clamp(0.0, 255.0) as u8;
        img[y * N + x] = Color32::from_rgb(c(base[0]), c(base[1]), c(base[2]));
    } }
    let n_fib = (N * N) as f32 * fiber_pct / 6.0;
    for _ in 0..n_fib as usize {
        let (mut x, mut y) = ((rnd() as usize) % N, (rnd() as usize) % N);
        for k in 0..(3 + rnd() % 6) {
            let i = (y % N) * N + (x % N); let p = img[i];
            img[i] = Color32::from_rgb(p.r().saturating_sub(6), p.g().saturating_sub(6), p.b().saturating_sub(5));
            x += 2; if k % 2 == 1 { y += 1; }
        }
    }
    ColorImage::new([N, N], img)
}

/// 판 질감 텍스처 (컨텍스트에 캐시, NEAREST · 반복)
pub fn tex(ctx: &Context, kind: Tex) -> TextureHandle {
    let id = egui::Id::unique(("game-ui-tex", kind as u8));
    if let Some(t) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) { return t; }
    let img = match kind {
        Tex::Plate => bake_plate([0xa5, 0x9d, 0x83], 7, 7, 3.0, 0.03, 16),
        Tex::Slate => bake_plate([0x6b, 0x73, 0x7b], 11, 5, 2.0, 0.02, 16),
        Tex::Page => bake_plate([0xe8, 0xdf, 0xd0], 13, 1, 0.5, 0.0, 16),
        Tex::Dialog => bake_plate([0xd6, 0xc4, 0x9d], 17, 3, 2.0, 0.04, 16),
        Tex::Parchment => bake_plate([0xda, 0xcc, 0xae], 19, 2, 6.0, 0.02, 32),
    };
    let t = ctx.load_texture(format!("tex-{kind:?}"), img, TextureOptions { magnification: egui::TextureFilter::Nearest, minification: egui::TextureFilter::Nearest, wrap_mode: egui::TextureWrapMode::Repeat, mipmap_mode: None });
    ctx.data_mut(|d| d.insert_temp(id, t.clone())); t
}
/// 질감을 rect에 깐다 — 한 칸 = texel 점, 판 왼쪽 위에 고정 (창이 움직여도 질감이 미끄러지지 않는다)
pub fn tex_shape(ctx: &Context, r: Rect, kind: Tex, texel: f32, tint: Color32) -> Shape {
    let t = tex(ctx, kind);
    let uv = Rect::from_min_max(Pos2::ZERO, Pos2::new(r.width() / (64.0 * texel), r.height() / (64.0 * texel)));
    let mut m = Mesh::with_texture(t.id()); m.add_rect_with_uv(r, uv, tint); Shape::mesh(m)
}
/// 가로 그라데이션 (모양으로)
pub fn hgrad_shape(r: Rect, left: Color32, right: Color32) -> Shape {
    let mut m = Mesh::default();
    m.colored_vertex(r.left_top(), left); m.colored_vertex(r.right_top(), right); m.colored_vertex(r.left_bottom(), left); m.colored_vertex(r.right_bottom(), right);
    m.add_triangle(0, 1, 2); m.add_triangle(1, 3, 2); Shape::mesh(m)
}
/// 세로 그라데이션 (모양으로)
pub fn vgrad_shape(r: Rect, top: Color32, bottom: Color32) -> Shape {
    let mut m = Mesh::default();
    m.colored_vertex(r.left_top(), top); m.colored_vertex(r.right_top(), top); m.colored_vertex(r.left_bottom(), bottom); m.colored_vertex(r.right_bottom(), bottom);
    m.add_triangle(0, 1, 2); m.add_triangle(1, 3, 2); Shape::mesh(m)
}
/// 테 그라데이션 — 바깥 테두리는 outer 색, w만큼 안쪽은 inner 색 (그을린 가장자리 · 종이 두께)
pub fn ring_shape(r: Rect, w: f32, outer: Color32, inner: Color32) -> Shape {
    let i = r.shrink(w);
    let mut m = Mesh::default();
    for p in [r.left_top(), r.right_top(), r.right_bottom(), r.left_bottom()] { m.colored_vertex(p, outer); }
    for p in [i.left_top(), i.right_top(), i.right_bottom(), i.left_bottom()] { m.colored_vertex(p, inner); }
    for k in 0..4u32 { let n = (k + 1) % 4; m.add_triangle(k, n, 4 + k); m.add_triangle(n, 4 + n, 4 + k); }
    Shape::mesh(m)
}
/// 채운 사각형 (선 대신 — 획은 반 화소에서 번진다)
pub fn fill(r: Rect, c: Color32) -> Shape { Shape::rect_filled(r, 0.0, c) }

/// 막 — 왼쪽은 어둡고 오른쪽으로 갈수록 세계가 비치는 막 + 아래 그늘 (타이틀 뒤). 꼭짓점 색 보간이라 띠가 생기지 않는다
pub fn veil(p: &Painter, rect: Rect, strength: f32) {
    let band = |m: &mut Mesh, a: (Pos2, Pos2), b: (Pos2, Pos2), ca: Color32, cb: Color32| {
        let i = m.vertices.len() as u32; m.colored_vertex(a.0, ca); m.colored_vertex(a.1, ca); m.colored_vertex(b.0, cb); m.colored_vertex(b.1, cb);
        m.add_triangle(i, i + 1, i + 2); m.add_triangle(i + 1, i + 3, i + 2);
    };
    let mut m = Mesh::default(); let n = 40; let w = rect.width() * 0.7;
    let col = |t: f32| Color32::from_rgba_unmultiplied(10, 8, 6, ((1.0 - t).clamp(0.0, 1.0).powf(1.25) * 230.0 * strength) as u8);
    for k in 0..n { let (t0, t1) = (k as f32 / n as f32, (k + 1) as f32 / n as f32); let (x0, x1) = (rect.min.x + w * t0, rect.min.x + w * t1); band(&mut m, (Pos2::new(x0, rect.min.y), Pos2::new(x0, rect.max.y)), (Pos2::new(x1, rect.min.y), Pos2::new(x1, rect.max.y)), col(t0), col(t1)); }
    let h = 200.0; let n = 12;
    let col2 = |t: f32| Color32::from_rgba_unmultiplied(8, 6, 4, (t.powf(1.6) * 170.0 * strength) as u8);
    for k in 0..n { let (t0, t1) = (k as f32 / n as f32, (k + 1) as f32 / n as f32); let (y0, y1) = (rect.max.y - h * (1.0 - t0), rect.max.y - h * (1.0 - t1)); band(&mut m, (Pos2::new(rect.min.x, y0), Pos2::new(rect.max.x, y0)), (Pos2::new(rect.min.x, y1), Pos2::new(rect.max.x, y1)), col2(t0), col2(t1)); }
    p.add(m);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plate_texture_matches_measured_mean_and_noise() {
        // 리서치 §6.2: 클래식 판 평균 #a59d83 근처, 표준편차 3~5
        let img = bake_plate([0xa5, 0x9d, 0x83], 7, 7, 3.0, 0.03, 16);
        let n = img.pixels.len() as f32;
        let mean = img.pixels.iter().map(|p| p.r() as f32).sum::<f32>() / n;
        let sd = (img.pixels.iter().map(|p| (p.r() as f32 - mean).powi(2)).sum::<f32>() / n).sqrt();
        assert!((mean - 165.0).abs() < 3.0, "mean {mean}");
        assert!((2.0..=6.0).contains(&sd), "sd {sd}");
    }
}
