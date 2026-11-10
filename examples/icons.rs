//! 아이콘 표 — 62종을 세 크기(16 · 24 · 40)와 두 배경에 (`cargo run -p game-ui --example icons --features snapshot -- out.png`)
use game_ui::{icons, snapshot::{save_png, Snapshot}};
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "icons.png".into());
    let mut s = Snapshot::new().expect("GPU");
    let (w, h, px) = s.render([1240.0, 620.0], 2.0, [0.06, 0.07, 0.1, 1.0], 2, &mut |ui| {
        let p = ui.painter().clone();
        p.rect_filled(egui::Rect::from_min_size(egui::pos2(0.0, 310.0), egui::vec2(1240.0, 310.0)), 0.0, egui::Color32::from_rgb(214, 196, 160));
        for (k, &i) in icons::ALL.iter().enumerate() {
            let (col, row) = ((k % 16) as f32, (k / 16) as f32);
            for (band, size) in [(0.0f32, 24.0f32), (310.0, 24.0)] {
                let x = 20.0 + col * 76.0; let y = band + 20.0 + row * 70.0;
                icons::paint(&p, egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(40.0, 40.0)), i, None);
                icons::paint(&p, egui::Rect::from_min_size(egui::pos2(x + 44.0, y + 4.0), egui::vec2(size, size)), i, None);
                icons::paint(&p, egui::Rect::from_min_size(egui::pos2(x + 44.0, y + 30.0), egui::vec2(16.0, 16.0)), i, None);
                p.text(egui::pos2(x, y + 44.0), egui::Align2::LEFT_TOP, format!("{i:?}"), egui::FontId::proportional(10.0), if band == 0.0 { egui::Color32::GRAY } else { egui::Color32::from_rgb(60, 40, 20) });
            }
        }
    }).expect("render");
    save_png(std::path::Path::new(&out), w, h, &px).expect("png");
    println!("icons {out} {w}x{h}");
}
