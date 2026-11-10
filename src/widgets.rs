//! 위젯 (deskarium에서 분리한 첫 판) — 메뉴 줄 · 큰 버튼 · 막 · 알약 · 도구 줄 · 탭 · 섹션 제목 · 타일 · 막대.
use crate::fonts::bold;
use crate::theme::{ACCENT, CARD, CARD_HI, DIM, LINE, TEXT};
use egui::{pos2, vec2, Align2, Color32, CornerRadius, FontId, Margin, Pos2, Rect, Response, RichText, Sense, Stroke, StrokeKind, Ui};

/// 메뉴 줄 하나 — 굵은 글씨, 마우스를 올리면 왼쪽 호박색 막대와 함께 살짝 밀린다
pub fn menu_item(ui: &mut Ui, label: &str, sub: Option<&str>, enabled: bool) -> Response {
    let h = if sub.is_some() { 60.0 } else { 46.0 };
    let (rect, resp) = ui.allocate_exact_size(vec2(360.0, h), if enabled { Sense::click() } else { Sense::hover() });
    let hov = ui.ctx().animate_bool_with_time(resp.id, resp.hovered() && enabled, 0.12);
    let p = ui.painter();
    if hov > 0.0 { p.rect_filled(rect, 9.0, Color32::from_white_alpha((16.0 * hov) as u8)); p.rect_filled(Rect::from_min_size(rect.min + vec2(0.0, 8.0), vec2(4.0, h - 16.0)), 2.0, ACCENT.gamma_multiply(hov)); }
    let x = rect.min.x + 20.0 + 10.0 * hov;
    let col = if enabled { TEXT.lerp_to_gamma(Color32::WHITE, hov) } else { DIM.gamma_multiply(0.5) };
    p.text(pos2(x, rect.min.y + 9.0), Align2::LEFT_TOP, label, bold(25.0), col);
    if let Some(s) = sub { p.text(pos2(x, rect.min.y + 39.0), Align2::LEFT_TOP, s, FontId::proportional(13.0), if hov > 0.5 { ACCENT } else { DIM }); }
    if enabled && resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
    resp
}

/// 큰 호박색 버튼
pub fn primary(ui: &mut Ui, label: &str, w: f32) -> Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(w, 46.0), Sense::click());
    let hov = ui.ctx().animate_bool_with_time(resp.id, resp.hovered(), 0.1);
    ui.painter().rect_filled(rect, 10.0, ACCENT.lerp_to_gamma(Color32::from_rgb(255, 214, 120), hov));
    ui.painter().text(rect.center(), Align2::CENTER_CENTER, label, bold(18.0), Color32::from_rgb(28, 22, 8));
    if resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
    resp
}

pub fn ghost(ui: &mut Ui, label: &str, w: f32) -> Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(w, 46.0), Sense::click());
    let hov = ui.ctx().animate_bool_with_time(resp.id, resp.hovered(), 0.1);
    ui.painter().rect(rect, 10.0, CARD.lerp_to_gamma(CARD_HI, hov), Stroke::new(1.0, LINE.lerp_to_gamma(ACCENT, hov * 0.6)), egui::StrokeKind::Inside);
    ui.painter().text(rect.center(), Align2::CENTER_CENTER, label, bold(16.0), TEXT);
    if resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
    resp
}

/// 왼쪽은 어둡고 오른쪽으로 갈수록 세계가 비치는 막 + 아래 그늘 (타이틀) — 꼭짓점 색 보간이라 띠가 생기지 않는다
pub fn veil(ui: &Ui, rect: Rect, strength: f32) {
    let band = |m: &mut egui::Mesh, a: (Pos2, Pos2), b: (Pos2, Pos2), ca: Color32, cb: Color32| {
        let i = m.vertices.len() as u32; m.colored_vertex(a.0, ca); m.colored_vertex(a.1, ca); m.colored_vertex(b.0, cb); m.colored_vertex(b.1, cb);
        m.add_triangle(i, i + 1, i + 2); m.add_triangle(i + 1, i + 3, i + 2);
    };
    let mut m = egui::Mesh::default(); let n = 40; let w = rect.width() * 0.7;
    let col = |t: f32| Color32::from_rgba_unmultiplied(8, 10, 16, ((1.0 - t).clamp(0.0, 1.0).powf(1.25) * 238.0 * strength) as u8);
    for k in 0..n { let (t0, t1) = (k as f32 / n as f32, (k + 1) as f32 / n as f32); let (x0, x1) = (rect.min.x + w * t0, rect.min.x + w * t1); band(&mut m, (pos2(x0, rect.min.y), pos2(x0, rect.max.y)), (pos2(x1, rect.min.y), pos2(x1, rect.max.y)), col(t0), col(t1)); }
    let h = 200.0; let n = 12;
    let col2 = |t: f32| Color32::from_rgba_unmultiplied(6, 8, 12, (t.powf(1.6) * 170.0 * strength) as u8);
    for k in 0..n { let (t0, t1) = (k as f32 / n as f32, (k + 1) as f32 / n as f32); let (y0, y1) = (rect.max.y - h * (1.0 - t0), rect.max.y - h * (1.0 - t1)); band(&mut m, (pos2(rect.min.x, y0), pos2(rect.max.x, y0)), (pos2(rect.min.x, y1), pos2(rect.max.x, y1)), col2(t0), col2(t1)); }
    ui.painter().add(m);
}

/// 둥근 알약 (상태 표시). click이면 누를 수 있다
pub fn pill(ui: &mut Ui, text: &str, col: Color32, fill: f32, click: bool) -> Response {
    let g = ui.painter().layout_no_wrap(text.to_string(), bold(13.0), col);
    let (r, resp) = ui.allocate_exact_size(g.size() + vec2(18.0, 8.0), if click { Sense::click() } else { Sense::hover() });
    let hov = if click { ui.ctx().animate_bool_with_time(resp.id, resp.hovered(), 0.1) } else { 0.0 };
    ui.painter().rect(r, CornerRadius::same(255), col.gamma_multiply(fill + 0.08 * hov), Stroke::new(1.0, col.gamma_multiply(0.55 + 0.3 * hov)), StrokeKind::Inside);
    ui.painter().galley(r.min + vec2(9.0, 4.0), g, col);
    if click && resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
    resp
}

pub fn tool_btn(ui: &mut Ui, sel: bool, icon: &str, label: &str) -> Response {
    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 27.0), Sense::click());
    let hov = ui.ctx().animate_bool_with_time(resp.id, resp.hovered(), 0.1);
    let p = ui.painter();
    if sel { p.rect_filled(r, 7.0, ACCENT); } else if hov > 0.0 { p.rect_filled(r, 7.0, CARD_HI.gamma_multiply(hov)); }
    let col = if sel { Color32::from_rgb(28, 22, 8) } else { TEXT };
    p.text(pos2(r.min.x + 16.0, r.center().y), Align2::CENTER_CENTER, icon, FontId::proportional(14.0), col);
    p.text(pos2(r.min.x + 32.0, r.center().y), Align2::LEFT_CENTER, label, if sel { bold(13.5) } else { FontId::proportional(13.5) }, col);
    if resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
    resp
}

pub fn tab_btn(ui: &mut Ui, sel: bool, label: &str, w: f32) -> Response {
    let (r, resp) = ui.allocate_exact_size(vec2(w, 32.0), Sense::click());
    let hov = ui.ctx().animate_bool_with_time(resp.id, resp.hovered() || sel, 0.12);
    let p = ui.painter();
    if hov > 0.0 { p.rect_filled(r, 8.0, CARD.gamma_multiply(hov)); }
    p.text(r.center(), Align2::CENTER_CENTER, label, bold(13.5), if sel { Color32::WHITE } else { DIM.lerp_to_gamma(TEXT, hov) });
    if sel { p.rect_filled(Rect::from_min_size(pos2(r.center().x - 14.0, r.max.y - 3.0), vec2(28.0, 3.0)), 2.0, ACCENT); }
    if resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
    resp
}

pub fn section(ui: &mut Ui, t: &str) { ui.add_space(8.0); ui.label(RichText::new(t).font(bold(14.0)).color(ACCENT)); ui.add_space(2.0); }

pub fn tile(ui: &mut Ui, w: f32, label: &str, val: String, col: Color32) {
    egui::Frame::new().fill(CARD_HI).corner_radius(CornerRadius::same(8)).inner_margin(Margin::symmetric(10, 6)).show(ui, |ui| {
        ui.set_width(w - 20.0);
        ui.label(RichText::new(label).small().color(DIM));
        ui.label(RichText::new(val).font(bold(18.0)).color(col));
    });
}

pub fn bar(ui: &mut Ui, k: f32, col: Color32, text: String) {
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 18.0), Sense::hover()); let p = ui.painter();
    p.rect_filled(r, 6.0, CARD_HI); p.rect_filled(Rect::from_min_size(r.min, vec2(r.width() * k.clamp(0.0, 1.0), r.height())), 6.0, col.gamma_multiply(0.75));
    p.text(r.min + vec2(8.0, r.height() / 2.0), Align2::LEFT_CENTER, text, FontId::proportional(12.0), Color32::WHITE);
}
