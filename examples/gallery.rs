//! 갤러리 — 모든 위젯을 한 장에 그려 PNG로 (`cargo run -p game-ui --example gallery --features snapshot -- out.png`)
use game_ui::{snapshot::{save_png, Snapshot}, theme, widgets};

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "gallery.png".into());
    let mut s = Snapshot::new().expect("GPU");
    theme::apply(&s.ctx, 1.0);
    let (w, h, px) = s.render([1280.0, 800.0], 1.0, [0.05, 0.06, 0.09, 1.0], 4, &mut |ui| {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("game-ui gallery");
            widgets::menu_item(ui, "이어하기", Some("1년 3일 08:00 · 하드코어"), true);
            widgets::menu_item(ui, "새 세계", None, true);
            ui.horizontal(|ui| { widgets::ghost(ui, "뒤로", 140.0); widgets::primary(ui, "세계 만들기  ▶", 260.0); });
            ui.horizontal(|ui| { widgets::pill(ui, "🔥 하드코어", theme::HARDCORE, 0.16, false); widgets::pill(ui, "★ 노을내", theme::ACCENT, 0.16, true); });
            widgets::bar(ui, 0.6, egui::Color32::from_rgb(111, 184, 255), "연구 「왕권」 37/77".into());
            ui.horizontal(|ui| { widgets::tile(ui, 120.0, "인구", "12".into(), egui::Color32::WHITE); widgets::tile(ui, 120.0, "식량", "362".into(), egui::Color32::WHITE); });
        });
    }).expect("render");
    save_png(std::path::Path::new(&out), w, h, &px).expect("png");
    println!("gallery {out} {w}x{h} adapter={}", s.adapter);
}
