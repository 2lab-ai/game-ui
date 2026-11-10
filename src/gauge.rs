//! 게이지 — 판 위에서는 클래식(왼끝을 비스듬히 자른 평행사변형 · 가로 그라데이션 · 1u 먹선 · 1u 그림자),
//! HUD 위에서는 TIC 관(위 밝은 선 · 짙은 속 · 아래 조금 밝게, 1점 거의 검은 테). 값은 막대 오른쪽 끝 위에 옛 숫자꼴로.
use crate::skin::{self, unit, Surface};
use crate::{fonts, text};
use egui::{pos2, vec2, Align2, Color32, Mesh, Pos2, Rect, Sense, Shape, Stroke, Ui};

#[derive(Clone, Copy, Debug)]
pub enum Tone { Hp, Mp, Ct, Custom(Color32, Color32) }

fn lerp(a: Color32, b: Color32, t: f32) -> Color32 { a.lerp_to_gamma(b, t.clamp(0.0, 1.0)) }

/// 막대만 그린다 (rect 안). 표면이 HUD면 TIC 관, 아니면 클래식 평행사변형
pub fn bar(p: &egui::Painter, r: Rect, k: f32, tone: Tone) {
    let sk = skin::get(p.ctx()); let u = unit(p.ctx()); let k = k.clamp(0.0, 1.0); let r = skin::snap_rect(p.ctx(), r);
    if skin::surface() == Surface::Dark {
        let [t, c, b] = match tone { Tone::Hp => sk.hp_tube, Tone::Mp => sk.mp_tube, Tone::Ct => sk.ct_tube, Tone::Custom(hi, lo) => [hi, lo, lerp(lo, hi, 0.4)] };
        p.rect_filled(r, 2.0, Color32::from_rgb(0x08, 0x0a, 0x10));
        let i = r.shrink(1.0);
        p.rect_filled(i, 1.5, Color32::from_rgba_unmultiplied(0x20, 0x26, 0x3a, 230));
        if k > 0.0 {
            let f = Rect::from_min_size(i.min, vec2((i.width() * k).max(2.0), i.height()));
            let h = f.height();
            let band = |y0: f32, y1: f32, a: Color32, b: Color32| { let mut m = Mesh::default(); let rr = Rect::from_min_max(pos2(f.min.x, f.min.y + h * y0), pos2(f.max.x, f.min.y + h * y1)); m.colored_vertex(rr.left_top(), a); m.colored_vertex(rr.right_top(), a); m.colored_vertex(rr.left_bottom(), b); m.colored_vertex(rr.right_bottom(), b); m.add_triangle(0, 1, 2); m.add_triangle(1, 3, 2); p.add(Shape::mesh(m)); };
            band(0.0, 0.3, t, c); band(0.3, 0.75, c, c); band(0.75, 1.0, c, b);
            p.rect_filled(Rect::from_min_size(f.min + vec2(1.0, 1.0), vec2((f.width() - 2.0).max(0.0), 1.0)), 0.0, Color32::from_white_alpha(70));
        }
        return;
    }
    // 클래식
    let (left, right) = match tone { Tone::Hp => sk.hp, Tone::Mp => sk.mp, Tone::Ct => sk.ct, Tone::Custom(hi, lo) => (lo, hi) };
    let sl = (r.height() * 0.6).min(3.0 * u);
    let para = |r: Rect| -> Vec<Pos2> { vec![pos2(r.min.x + sl, r.min.y), pos2(r.max.x, r.min.y), pos2(r.max.x, r.max.y), pos2(r.min.x, r.max.y)] };
    p.add(Shape::convex_polygon(para(r.translate(vec2(0.0, u))), sk.plate_shadow, Stroke::NONE));
    p.add(Shape::convex_polygon(para(r), sk.gauge_out, Stroke::NONE));
    let i = Rect::from_min_max(pos2(r.min.x + u, r.min.y + u), pos2(r.max.x - u, r.max.y - u));
    p.add(Shape::convex_polygon(para(i), sk.gauge_empty, Stroke::NONE));
    if k > 0.0 {
        let xk = i.min.x + (i.width() * k).max(sl + u);
        let col = |x: f32| lerp(left, right, (x - i.min.x) / i.width());
        let mut m = Mesh::default();
        m.colored_vertex(pos2(i.min.x + sl, i.min.y), col(i.min.x + sl)); m.colored_vertex(pos2(xk, i.min.y), col(xk));
        m.colored_vertex(pos2(xk, i.max.y), col(xk)); m.colored_vertex(pos2(i.min.x, i.max.y), col(i.min.x));
        m.add_triangle(0, 1, 2); m.add_triangle(0, 2, 3); p.add(Shape::mesh(m));
    }
}

/// 값 글자 — "151" 크게, "/151" 작게 오른쪽 아래로 엇갈려 (원작 상태창)
fn fraction(p: &egui::Painter, right: Pos2, cur: &str, max: Option<&str>, col: Color32, sh: Color32) {
    let u = unit(p.ctx());
    let mut x = right.x;
    if let Some(m) = max { let r = text::shadowed(p, pos2(x, right.y + 2.0 * u), Align2::RIGHT_CENTER, format!("/{m}"), fonts::num(11.5), col, sh); x = r.min.x - 1.0; }
    text::shadowed(p, pos2(x, right.y), Align2::RIGHT_CENTER, cur, fonts::num(15.0), col, sh);
}

/// 이름 + 값 + 막대 한 줄 (width 점)
pub fn gauge(ui: &mut Ui, name: &str, cur: f32, max: f32, tone: Tone, width: f32) {
    let pl = skin::pal(ui.ctx()); let u = unit(ui.ctx());
    let (r, _) = ui.allocate_exact_size(vec2(width, 17.0 * u), Sense::hover()); let p = ui.painter();
    text::shadowed(p, pos2(r.min.x, r.min.y + 5.0 * u), Align2::LEFT_CENTER, name, fonts::bold(13.0), pl.text, pl.shadow);
    let w = (cur.max(max) as i64).to_string().len().max(3);
    fraction(p, pos2(r.max.x, r.min.y + 4.5 * u), &text::pad0(cur.round() as i64, w), Some(&text::pad0(max.round() as i64, w)), pl.text, pl.shadow);
    bar(p, Rect::from_min_size(pos2(r.min.x, r.min.y + 11.0 * u), vec2(width, 5.0 * u)), if max > 0.0 { cur / max } else { 0.0 }, tone);
}

/// 퍼센트 게이지 (행복 · 충성 · 연구처럼 숫자가 %). right = 오른쪽 값 글(없으면 %)
pub fn percent(ui: &mut Ui, name: &str, k: f32, tone: Tone, width: f32, right: Option<String>) {
    let pl = skin::pal(ui.ctx()); let u = unit(ui.ctx());
    let (r, _) = ui.allocate_exact_size(vec2(width, 17.0 * u), Sense::hover()); let p = ui.painter();
    text::shadowed(p, pos2(r.min.x, r.min.y + 5.0 * u), Align2::LEFT_CENTER, name, fonts::bold(13.0), pl.text, pl.shadow);
    let rt = right.unwrap_or_else(|| format!("{:.0}%", k * 100.0));
    match rt.split_once('/') {
        Some((a, b)) if !a.contains(' ') && !b.contains(' ') => fraction(p, pos2(r.max.x, r.min.y + 4.5 * u), a, Some(b), pl.text, pl.shadow),
        _ => { text::shadowed(p, pos2(r.max.x, r.min.y + 5.0 * u), Align2::RIGHT_CENTER, rt, fonts::num(14.0), pl.text, pl.shadow); }
    }
    bar(p, Rect::from_min_size(pos2(r.min.x, r.min.y + 11.0 * u), vec2(width, 5.0 * u)), k, tone);
}
