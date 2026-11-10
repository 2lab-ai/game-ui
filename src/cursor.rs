//! 손 커서 — FFT의 손처럼 검지를 오른쪽으로 뻗은 흰 장갑(TIC #fffffa · #dedfd6 · 그늘 #605d59) + 강철 소매.
//! 줄을 옮기면 곧바로 건너뛰고(미끄러지지 않는다), 제자리에서 2단으로 까딱인다: 0.25초 1u 뒤로 · 0.25초 제자리 [리서치 §2.4].
use egui::{Color32, Painter, Pos2, Shape, Stroke};

/// tip = 검지 끝. h = 손 높이(점). t = 시간(초)
pub fn hand(p: &Painter, tip: Pos2, h: f32, t: f64) {
    let u = crate::skin::unit(p.ctx());
    let bob = if t.rem_euclid(0.5) < 0.25 { u } else { 0.0 };
    hand_at(p, tip, h, bob, 255);
}
/// 흐린 손 — 자식 창이 열린 부모 창에 남는 손 (원작 ps1_03)
pub fn ghost(p: &Painter, tip: Pos2, h: f32) { hand_at(p, tip, h, 0.0, 120); }

fn hand_at(p: &Painter, tip: Pos2, h: f32, bob: f32, alpha: u8) {
    let p = &p.with_clip_rect(p.ctx().content_rect());
    let s = h / 16.0; let o = tip - egui::vec2(24.0 * s + bob, 7.0 * s);
    let at = |x: f32, y: f32| o + egui::vec2(x * s, y * s);
    let a = |c: Color32| if alpha == 255 { c } else { c.gamma_multiply(alpha as f32 / 255.0) };
    let (white, shade, line, cuff, cuff_d) = (a(Color32::from_rgb(0xff, 0xff, 0xfa)), a(Color32::from_rgb(0xde, 0xdf, 0xd6)), a(Color32::from_rgb(0x28, 0x28, 0x20)), a(Color32::from_rgb(0x78, 0x88, 0x99)), a(Color32::from_rgb(0x40, 0x48, 0x50)));
    let poly = |pts: &[(f32, f32)], fill: Color32, w: f32| { p.add(Shape::convex_polygon(pts.iter().map(|&(x, y)| at(x, y)).collect(), fill, Stroke::new(w * s, line))); };
    // 그림자
    let sh = a(Color32::from_black_alpha(120)); let d = 2.0 * s;
    p.add(Shape::convex_polygon([(0.0, 3.0), (17.0, 3.0), (24.0, 5.5), (24.0, 8.5), (17.0, 13.5), (0.0, 13.5)].iter().map(|&(x, y)| at(x + d, y + d)).collect(), sh, Stroke::NONE));
    // 소매(갑옷 손목)
    poly(&[(0.0, 3.0), (5.0, 3.0), (5.0, 13.5), (0.0, 13.5)], cuff, 1.1);
    p.line_segment([at(1.6, 4.0), at(1.6, 12.5)], Stroke::new(1.0 * s, cuff_d));
    p.line_segment([at(3.4, 4.0), at(3.4, 12.5)], Stroke::new(0.8 * s, a(Color32::from_rgb(0xb1, 0xb9, 0xc1))));
    // 손바닥
    poly(&[(5.0, 4.0), (12.5, 3.2), (15.0, 5.0), (15.0, 12.0), (12.0, 14.0), (5.0, 13.0)], white, 1.1);
    // 검지 (오른쪽으로)
    poly(&[(12.0, 4.2), (22.6, 4.6), (24.0, 6.1), (22.6, 7.6), (12.0, 7.8)], white, 1.1);
    // 접은 손가락 셋
    for (k, y) in [8.0f32, 10.0, 12.0].iter().enumerate() { let x1 = 18.0 - k as f32 * 1.4; poly(&[(12.5, *y - 0.2), (x1, *y), (x1 + 0.9, *y + 1.0), (x1, *y + 2.0), (12.5, *y + 2.1)], if k == 2 { shade } else { white }, 0.9); }
    // 엄지 (위)
    poly(&[(8.0, 3.6), (11.0, 0.8), (13.8, 1.2), (14.2, 2.6), (12.0, 4.4)], white, 1.0);
    // 손등 그늘 · 마디
    p.line_segment([at(6.0, 12.4), at(11.6, 12.8)], Stroke::new(1.2 * s, shade));
    p.line_segment([at(19.5, 4.9), at(19.5, 7.4)], Stroke::new(0.7 * s, shade));
}
