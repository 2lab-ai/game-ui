//! 글자 — 판 위의 먹 글씨는 +1u 아래오른쪽에 옅은 먹 그림자(원작 #847c6a), 3D 위 글씨는 판 없이 1점 윤곽(크림 #f7f5ed + 거의 검정 #010c00).
use egui::{Align2, Color32, FontId, Painter, Pos2, Rect};

/// 그림자 글자 — 그림자 오프셋 = 반 u (2점 → 1점, 큰 글자는 1u). 그림자가 투명이면 그리지 않는다
pub fn shadowed(p: &Painter, at: Pos2, align: Align2, text: impl ToString, font: FontId, fill: Color32, shadow: Color32) -> Rect {
    let t = text.to_string();
    if shadow.a() > 0 { let u = crate::skin::unit(p.ctx()); let d = if font.size >= 20.0 { u } else { u * 0.5 }; p.text(at + egui::vec2(d, d), align, &t, font.clone(), shadow); }
    p.text(at, align, t, font, fill)
}
/// 윤곽 글자 (여덟 방향) — 3D 위 이름 · 날짜 · 숫자. 두께 = 1점 (큰 글자는 1.5점)
pub fn outlined(p: &Painter, at: Pos2, align: Align2, text: impl ToString, font: FontId, fill: Color32, line: Color32) -> Rect {
    let t = text.to_string(); let d = if font.size >= 24.0 { 1.5 } else { 1.0 };
    for (dx, dy) in [(-1.0, -1.0), (0.0, -1.0), (1.0, -1.0), (-1.0, 0.0), (1.0, 0.0), (-1.0, 1.0), (0.0, 1.0), (1.0, 1.0)] { p.text(at + egui::vec2(dx * d, dy * d), align, &t, font.clone(), line); }
    p.text(at, align, t, font, fill)
}
/// 3D 위 글자 (HUD 크림 + 먹 윤곽)
pub fn hud(p: &Painter, at: Pos2, align: Align2, text: impl ToString, font: FontId) -> Rect {
    let s = crate::skin::get(p.ctx()); outlined(p, at, align, text, font, s.hud_text, s.hud_outline)
}
/// 지금 표면의 먹으로 (그림자 포함)
pub fn ink(p: &Painter, at: Pos2, align: Align2, text: impl ToString, font: FontId) -> Rect {
    let pl = crate::skin::pl(); shadowed(p, at, align, text, font, pl.text, pl.shadow)
}
/// 자간을 벌린 글자
pub fn spaced(s: &str, gap: usize) -> String { let sep = " ".repeat(gap); s.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(&sep) }
/// 0을 채운 고정폭 숫자 ("Lv.09"처럼) — width 자리
pub fn pad0(v: i64, width: usize) -> String { format!("{:0width$}", v, width = width) }
/// 점 이끎줄 "이름 …… 값" — 이름 끝과 값 앞 사이에 u 크기 점을 3u마다
pub fn leaders(p: &Painter, x0: f32, x1: f32, baseline: f32, c: Color32) {
    let u = crate::skin::unit(p.ctx()); let mut x = x0 + u; while x + u < x1 - u { p.rect_filled(Rect::from_min_size(egui::pos2(x, baseline - u), egui::vec2(u * 0.5, u * 0.5)), 0.0, c); x += u * 1.5; }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pad0_fixed_width() { assert_eq!(super::pad0(9, 2), "09"); assert_eq!(super::pad0(151, 3), "151"); assert_eq!(super::pad0(24, 3), "024"); }
}
