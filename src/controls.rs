//! 조작 — 원작 언어로만: 마름모 단추(TIC 확인 창) · 판 안의 칸 줄(클래식 탭 줄, 고른 칸은 뒤집힌다) · 어두운 탭 띠(TIC) ·
//! 판에 새긴 칸(도구) · ‹ 값 › 조절(원작 설정은 미끄럼 막대가 없다) · 먹 머리말 · 금줄 · 단축키 판.
//! 둥근 알약 · 강조색 테두리 · 빛나는 선택 없음. 고른 것은 손이 가리키거나 칸이 뒤집힌다.
use crate::skin::{self, snap_rect, unit, Surface};
use crate::{cursor, fonts, frame, icons::{self, Icon}, paint::{fill, Tex}, text};
use egui::{pos2, vec2, Align2, Color32, Mesh, Pos2, Rect, Response, Sense, Shape, Stroke, Ui};

/// 마름모 단추 모양 (끝이 45°로 뾰족한 긴 육각형). sel = 금빛
pub fn lozenge(p: &egui::Painter, r: Rect, sel: bool) {
    let s = skin::get(p.ctx()); let u = unit(p.ctx()); let r = snap_rect(p.ctx(), r);
    let hex = |r: Rect| -> Vec<Pos2> { let h = r.height() / 2.0; vec![pos2(r.min.x + h, r.min.y), pos2(r.max.x - h, r.min.y), pos2(r.max.x, r.center().y), pos2(r.max.x - h, r.max.y), pos2(r.min.x + h, r.max.y), pos2(r.min.x, r.center().y)] };
    p.add(Shape::convex_polygon(hex(r.translate(vec2(0.0, u))), Color32::from_black_alpha(120), Stroke::NONE));
    p.add(Shape::convex_polygon(hex(r), if sel { s.btn_sel_border } else { s.btn_idle_border }, Stroke::NONE));
    let i = r.shrink(u);
    if sel {
        let pts = hex(i); let cy = i.center().y; let col = |y: f32| { let t = (y - i.min.y) / i.height(); if t < 0.5 { s.btn_sel_top.lerp_to_gamma(s.btn_sel_mid, t * 2.0) } else { s.btn_sel_mid.lerp_to_gamma(s.btn_sel_bottom, (t - 0.5) * 2.0) } };
        let mut m = Mesh::default(); m.colored_vertex(pos2(i.center().x, cy), col(cy)); for q in &pts { m.colored_vertex(*q, col(q.y)); }
        for k in 0..6u32 { m.add_triangle(0, 1 + k, 1 + (k + 1) % 6); } p.add(Shape::mesh(m));
        p.add(fill(Rect::from_min_max(pos2(i.min.x + i.height() / 2.0, i.max.y - 2.0 * u), pos2(i.max.x - i.height() / 2.0, i.max.y - u)), s.btn_sel_glow.gamma_multiply(0.45)));
    } else {
        p.add(Shape::convex_polygon(hex(i), s.btn_idle, Stroke::NONE));
    }
    let k = i.shrink(u);
    p.add(Shape::closed_line(hex(k), Stroke::new(1.0, if sel { s.btn_sel_glow.gamma_multiply(0.35) } else { Color32::from_white_alpha(14) })));
}

/// 단추 (마름모). primary = 늘 금빛, 아니면 마우스를 올렸을 때 금빛 + 왼쪽에 손
pub fn button(ui: &mut Ui, label: &str, icon: Option<Icon>, primary: bool, width: f32) -> Response {
    let ctx = ui.ctx().clone(); let s = skin::get(&ctx); let u = unit(&ctx); let t = ctx.input(|i| i.time);
    let (r, resp) = ui.allocate_exact_size(vec2(width, 17.0 * u), Sense::click());
    let sel = primary || resp.hovered() || resp.has_focus();
    let p = ui.painter();
    lozenge(p, r, sel);
    let col = if sel { s.btn_text_sel } else { s.btn_text };
    let g = p.layout_no_wrap(label.to_string(), fonts::bold(15.0), col);
    let iw = if icon.is_some() { 11.0 * u } else { 0.0 }; let x = r.center().x - (g.size().x + iw) / 2.0;
    if let Some(ic) = icon { icons::paint(p, Rect::from_center_size(pos2(x + 4.5 * u, r.center().y), vec2(9.0 * u, 9.0 * u)), ic, None); }
    text::shadowed(p, pos2(x + iw, r.center().y), Align2::LEFT_CENTER, label, fonts::bold(15.0), col, Color32::from_black_alpha(160));
    if resp.hovered() { cursor::hand(p, pos2(r.min.x - u, r.center().y), 10.0 * u, t); ctx.set_cursor_icon(egui::CursorIcon::PointingHand); ctx.request_repaint_after(std::time::Duration::from_millis(250)); }
    resp
}

/// 판에 새긴 칸 하나 (rect) — 클래식: 위·왼 그늘 + 아래·오른 하이라이트로 파인 칸. on = 뒤집힘(먹 바탕)
pub fn cell(p: &egui::Painter, r: Rect, on: bool, hover: bool) {
    let s = skin::get(p.ctx()); let u = unit(p.ctx()); let r = snap_rect(p.ctx(), r);
    if skin::surface() == Surface::Dark {
        if on { p.add(crate::paint::vgrad_shape(r, s.tab_sel_top, s.tab_sel_bottom)); }
        else { p.rect_filled(r, 0.0, if hover { Color32::from_rgba_unmultiplied(0xf3, 0xee, 0xd0, 30) } else { Color32::from_rgba_unmultiplied(0, 6, 24, 120) }); }
        p.rect_stroke(r, 0.0, Stroke::new(1.0, Color32::from_rgba_unmultiplied(0xf3, 0xee, 0xd0, if on { 90 } else { 34 })), egui::StrokeKind::Inside);
        return;
    }
    if on {
        p.add(fill(r, s.plate_out));
        p.add(fill(Rect::from_min_max(pos2(r.min.x + u, r.max.y - u), pos2(r.max.x, r.max.y)), s.plate_hi.gamma_multiply(0.55)));
        p.add(fill(Rect::from_min_max(pos2(r.max.x - u, r.min.y + u), r.max), s.plate_hi.gamma_multiply(0.55)));
        return;
    }
    p.add(crate::paint::tex_shape(p.ctx(), r, Tex::Plate, u, if hover { Color32::from_rgb(255, 255, 248) } else { Color32::from_rgb(232, 230, 222) }));
    p.add(fill(Rect::from_min_size(r.min, vec2(r.width(), u)), s.plate_shade_r));
    p.add(fill(Rect::from_min_size(r.min, vec2(u, r.height())), s.plate_shade_b));
    p.add(fill(Rect::from_min_max(pos2(r.min.x + u, r.max.y - u), r.max), s.plate_hi));
    p.add(fill(Rect::from_min_max(pos2(r.max.x - u, r.min.y + u), r.max), s.plate_hi));
}

/// 작은 아이콘 칸 (정사각). on = 고른 것(뒤집힌 칸)
pub fn icon_button(ui: &mut Ui, icon: Icon, on: bool, size: f32) -> Response {
    let (r, resp) = ui.allocate_exact_size(vec2(size, size), Sense::click());
    let p = ui.painter();
    cell(p, r, on, resp.hovered());
    icons::paint(p, r.shrink(size * 0.16), icon, None);
    if resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
    resp
}

/// TIC 탭 띠 — 어두운 띠 위 칸들, 고른 칸은 크림금 그라데이션에 먹 글자
pub fn tabs(ui: &mut Ui, sel: &mut usize, labels: &[&str], width: f32) -> bool {
    let ctx = ui.ctx().clone(); let s = skin::get(&ctx); let u = unit(&ctx); let n = labels.len().max(1); let mut changed = false;
    let (r, _) = ui.allocate_exact_size(vec2(width, 15.0 * u), Sense::hover());
    let r = snap_rect(&ctx, r);
    let p = ui.painter();
    p.add(Shape::Rect(egui::epaint::Shadow { offset: [0, 2], blur: 8, spread: 0, color: Color32::from_black_alpha(90) }.as_shape(r, 0)));
    p.add(fill(r, s.tab_bar));
    let w = r.width() / n as f32;
    for (k, l) in labels.iter().enumerate() {
        let on = k == *sel;
        let tr = snap_rect(&ctx, Rect::from_min_size(pos2(r.min.x + k as f32 * w, r.min.y), vec2(w, r.height())));
        let resp = ui.interact(tr, ui.scope_id().with(("tab", k)), Sense::click());
        if on { p.add(crate::paint::vgrad_shape(tr.shrink(u), s.tab_sel_top, s.tab_sel_bottom)); p.add(fill(Rect::from_min_size(tr.min + vec2(u, u), vec2(tr.width() - 2.0 * u, 1.0)), Color32::from_white_alpha(110))); }
        if k > 0 { p.add(fill(Rect::from_min_size(pos2(tr.min.x, r.min.y + 2.0 * u), vec2(1.0, r.height() - 4.0 * u)), s.tab_div)); }
        let col = if on { s.tab_bar } else if resp.hovered() { s.tab_sel_top } else { s.tab_text };
        p.text(tr.center(), Align2::CENTER_CENTER, l, fonts::bold(14.0), col);
        if resp.clicked() && !on { *sel = k; changed = true; }
        if resp.hovered() { ctx.set_cursor_icon(egui::CursorIcon::PointingHand); }
    }
    p.add(fill(Rect::from_min_size(pos2(r.min.x, r.max.y - 1.0), vec2(r.width(), 1.0)), s.tab_div));
    changed
}

/// 칸 줄 — 클래식: 판 하나 안에 칸을 나누고 고른 칸은 뒤집힌다(먹 바탕 · 밝은 글자). HUD 위면 TIC 띠
pub fn choice<T: PartialEq + Copy>(ui: &mut Ui, val: &mut T, opts: &[(T, &str)], cell_w: f32) -> bool {
    let ctx = ui.ctx().clone(); let s = skin::get(&ctx); let u = unit(&ctx); let dark = matches!(skin::surface(), Surface::Dark | Surface::Page); let pl = skin::pal(&ctx); let mut ch = false;
    let n = opts.len().max(1);
    let (r, _) = ui.allocate_exact_size(vec2(cell_w * n as f32, 13.0 * u), Sense::hover());
    let r = snap_rect(&ctx, r);
    let p = ui.painter();
    if dark { p.add(fill(r, s.tab_bar.gamma_multiply(0.9))); p.rect_stroke(r, 0.0, Stroke::new(1.0, s.tab_div), egui::StrokeKind::Inside); }
    else { frame::paint(p, r, frame::Kind::Plate); }
    let inner = if dark { r.shrink(1.0) } else { r.shrink(u) };
    let w = inner.width() / n as f32;
    for (k, (v, l)) in opts.iter().enumerate() {
        let cr = snap_rect(&ctx, Rect::from_min_size(pos2(inner.min.x + k as f32 * w, inner.min.y), vec2(w, inner.height())));
        let resp = ui.interact(cr, ui.scope_id().with(("choice", k)), Sense::click());
        let on = *val == *v;
        if on {
            if dark { p.add(crate::paint::vgrad_shape(cr, s.tab_sel_top, s.tab_sel_bottom)); }
            else { p.add(fill(cr, s.plate_out)); p.add(fill(Rect::from_min_max(pos2(cr.min.x, cr.max.y - u), cr.max), s.plate_shade_r)); }
        } else if resp.hovered() { p.add(fill(cr.shrink(u * 0.5), if dark { Color32::from_white_alpha(18) } else { Color32::from_white_alpha(40) })); }
        if k > 0 && !on && *val != opts[k - 1].0 { p.add(fill(Rect::from_min_size(pos2(cr.min.x, cr.min.y + 2.0 * u), vec2(u * 0.5, cr.height() - 4.0 * u)), if dark { s.tab_div } else { s.plate_shade_r })); }
        let (col, sh) = if dark { (if on { s.tab_bar } else { s.tab_sel_top }, Color32::TRANSPARENT) } else if on { (s.plate_hi, Color32::TRANSPARENT) } else { (pl.text, pl.shadow) };
        text::shadowed(p, cr.center(), Align2::CENTER_CENTER, l, fonts::num(13.5), col, sh);
        if resp.clicked() && !on { *val = *v; ch = true; }
        if resp.hovered() { ctx.set_cursor_icon(egui::CursorIcon::PointingHand); }
    }
    ch
}

/// 확인 상자 — 판에 판 작은 칸 + 먹 체크
pub fn check(ui: &mut Ui, on: &mut bool, label: &str) -> bool {
    let ctx = ui.ctx().clone(); let u = unit(&ctx); let pl = skin::pal(&ctx);
    let g = ui.painter().layout_no_wrap(label.to_string(), fonts::bold(14.0), pl.text);
    let (r, resp) = ui.allocate_exact_size(vec2(14.0 * u + g.size().x, 12.0 * u), Sense::click());
    let b = Rect::from_min_size(pos2(r.min.x, r.center().y - 4.5 * u), vec2(9.0 * u, 9.0 * u)); let p = ui.painter();
    cell(p, b, false, resp.hovered());
    if *on { icons::paint(p, b.shrink(u), Icon::Check, Some(pl.text)); }
    text::shadowed(p, pos2(b.max.x + 4.0 * u, r.center().y), Align2::LEFT_CENTER, label, fonts::bold(14.0), pl.text, pl.shadow);
    if resp.clicked() { *on = !*on; }
    if resp.hovered() { ctx.set_cursor_icon(egui::CursorIcon::PointingHand); }
    resp.clicked()
}

/// ‹ 값 › 조절 — 왼·오른 화살 칸 + 눈금 칸들(값까지 먹으로 채움) + 값 글자. 눈금 위를 누르거나 끌어도 된다
pub fn slider(ui: &mut Ui, v: &mut f32, range: std::ops::RangeInclusive<f32>, step: f32, width: f32, fmt: &dyn Fn(f32) -> String) -> bool {
    let ctx = ui.ctx().clone(); let s = skin::get(&ctx); let u = unit(&ctx); let pl = skin::pal(&ctx);
    let (lo, hi) = (*range.start(), *range.end()); let mut ch = false;
    let (r, _) = ui.allocate_exact_size(vec2(width, 12.0 * u), Sense::hover());
    let a = 9.0 * u; let val_w = 34.0 * u;
    let left = Rect::from_min_size(pos2(r.min.x, r.center().y - a / 2.0), vec2(a, a));
    let right = Rect::from_min_size(pos2(r.max.x - val_w - a - 2.0 * u, r.center().y - a / 2.0), vec2(a, a));
    let track = Rect::from_min_max(pos2(left.max.x + 2.0 * u, r.center().y - 3.0 * u), pos2(right.min.x - 2.0 * u, r.center().y + 3.0 * u));
    let mut set = |nv: f32, v: &mut f32| { let nv = ((nv / step).round() * step).clamp(lo, hi); if (nv - *v).abs() > 1e-6 { *v = nv; ch = true; } };
    for (rr, d, id) in [(left, -1.0f32, "l"), (right, 1.0, "r")] {
        let resp = ui.interact(rr, ui.scope_id().with(("sl", id)), Sense::click());
        let p = ui.painter(); cell(p, rr, false, resp.hovered());
        let c = rr.center(); let k = 2.5 * u;
        let tri = if d < 0.0 { vec![c + vec2(-k, 0.0), c + vec2(k * 0.7, -k), c + vec2(k * 0.7, k)] } else { vec![c + vec2(k, 0.0), c + vec2(-k * 0.7, -k), c + vec2(-k * 0.7, k)] };
        p.add(Shape::convex_polygon(tri, pl.text, Stroke::NONE));
        if resp.clicked() { set(*v + d * step, v); }
        if resp.hovered() { ctx.set_cursor_icon(egui::CursorIcon::PointingHand); }
    }
    let tr = ui.interact(track, ui.scope_id().with("sl-track"), Sense::click_and_drag());
    if let Some(pp) = tr.interact_pointer_pos() { let k = ((pp.x - track.min.x) / track.width()).clamp(0.0, 1.0); set(lo + (hi - lo) * k, v); }
    if tr.hovered() { ctx.set_cursor_icon(egui::CursorIcon::PointingHand); }
    let p = ui.painter();
    let n = (((hi - lo) / step).round() as usize).clamp(4, 16);
    let k = ((*v - lo) / (hi - lo)).clamp(0.0, 1.0);
    let filled = (k * n as f32).round() as usize;
    let gw = track.width() / n as f32;
    for i in 0..n {
        let pr = snap_rect(&ctx, Rect::from_min_size(pos2(track.min.x + i as f32 * gw + u * 0.5, track.min.y), vec2(gw - u, track.height())));
        if i < filled { p.add(fill(pr, pl.text)); p.add(fill(Rect::from_min_size(pr.min, vec2(pr.width(), u * 0.5)), s.plate_hi.gamma_multiply(0.4))); }
        else { p.add(fill(pr, pl.well)); p.add(fill(Rect::from_min_size(pr.min, vec2(pr.width(), u * 0.5)), pl.well_edge.gamma_multiply(0.5))); }
    }
    text::shadowed(p, pos2(r.max.x, r.center().y), Align2::RIGHT_CENTER, fmt(*v), fonts::num(14.0), pl.text, pl.shadow);
    ch
}

/// 머리말 — 제목 먹 + 가는 1u 선
pub fn heading(ui: &mut Ui, t: &str) {
    let ctx = ui.ctx().clone(); let pl = skin::pal(&ctx); let u = unit(&ctx);
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 13.0 * u), Sense::hover()); let p = ui.painter();
    let tr = text::shadowed(p, pos2(r.min.x, r.center().y), Align2::LEFT_CENTER, t, fonts::heavy(14.5), pl.title, pl.shadow);
    rule(p, tr.max.x + 4.0 * u, r.max.x, r.center().y + u, pl.dim);
}

/// 구분선 — 1u 선, 시작에 작은 마름모
pub fn rule(p: &egui::Painter, x0: f32, x1: f32, y: f32, c: Color32) {
    if x1 - x0 < 8.0 { return; }
    let u = unit(p.ctx()); let y = skin::snap(p.ctx(), y);
    p.add(fill(Rect::from_min_max(pos2(x0 + 2.0 * u, y), pos2(x1, y + u * 0.5)), c.gamma_multiply(0.7)));
    p.add(Shape::convex_polygon(vec![pos2(x0, y - 1.5 * u), pos2(x0 + 1.5 * u, y + u * 0.25), pos2(x0, y + 2.0 * u), pos2(x0 - 1.5 * u, y + u * 0.25)], c, Stroke::NONE));
}

/// 가운데 금줄 (제목 아래)
pub fn rule_center(p: &egui::Painter, cx: f32, half: f32, y: f32, c: Color32) { frame::gold_rule(p, cx, half, y, c); }

/// 단축키 판 — 작은 클래식 판에 키 글자 + 오른쪽에 뜻 (HUD 글자). 돌려주는 값 = 차지한 rect
pub fn key_hint(p: &egui::Painter, at: Pos2, key: &str, label: &str) -> Rect {
    let ctx = p.ctx().clone(); let u = unit(&ctx); let s = skin::get(&ctx);
    let g = p.layout_no_wrap(key.to_string(), fonts::num(12.0), s.plate_out);
    let kr = snap_rect(&ctx, Rect::from_min_size(pos2(at.x, at.y - 5.0 * u), vec2((g.size().x + 4.0 * u).max(10.0 * u), 10.0 * u)));
    frame::paint(p, kr, frame::Kind::Plate);
    p.text(kr.center() + vec2(-u * 0.5, 0.0), Align2::CENTER_CENTER, key, fonts::num(12.0), s.plate_out);
    let lr = text::hud(p, pos2(kr.max.x + 3.0 * u, at.y), Align2::LEFT_CENTER, label, fonts::bold(13.0));
    kr.union(lr)
}

/// 스탯 줄 — (아이콘) 이름 …… 값. 값은 오른쪽 끝에 옛 숫자꼴, 사이는 점 이끎줄 (원작 상태창 "Move……4")
pub fn stat_row(ui: &mut Ui, icon: Option<Icon>, label: &str, value: &str, col: Option<Color32>, width: f32) -> Response {
    let ctx = ui.ctx().clone(); let u = unit(&ctx); let pl = skin::pal(&ctx);
    let (r, resp) = ui.allocate_exact_size(vec2(width, 11.0 * u), Sense::hover());
    let p = ui.painter(); let cy = r.center().y; let mut x = r.min.x;
    if let Some(ic) = icon { icons::paint(p, Rect::from_center_size(pos2(x + 4.0 * u, cy), vec2(8.0 * u, 8.0 * u)), ic, None); x += 10.0 * u; }
    let lr = text::shadowed(p, pos2(x, cy), Align2::LEFT_CENTER, label, fonts::bold(13.0), pl.text, pl.shadow);
    let vr = text::shadowed(p, pos2(r.max.x, cy), Align2::RIGHT_CENTER, value, fonts::num(14.0), col.unwrap_or(pl.text), pl.shadow);
    text::leaders(p, lr.max.x + u, vr.min.x - u, cy + 5.0, pl.dim.gamma_multiply(0.8));
    resp
}
