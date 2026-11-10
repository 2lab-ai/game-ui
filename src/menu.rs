//! 명령 메뉴 — 줄마다 (아이콘) · 이름 · 오른쪽 값. 고른 줄은 색이 아니라 손이 가리킨다:
//! 손 끝이 창 왼쪽 테에 닿고, 줄을 옮기면 곧바로 건너뛴다 (원작은 60fps 커서 반응을 최우선으로 뒀다 [S3]).
//! 클래식 판 위에서는 줄 색을 바꾸지 않는다. TIC 페이지 · 양피지 · HUD 위에서는 고른 줄이 왼쪽부터 밝아지고, 줄 사이에 가는 선.
//! 마우스(올리기 · 누르기)와 키보드(↑ ↓ · Enter) 둘 다. 비활성 줄은 먹을 45%로, 누를 수 없다.
use crate::skin::{self, unit, Surface};
use crate::{cursor, fonts, icons::{self, Icon}, text};
use egui::{pos2, vec2, Align2, Key, Rect, Sense, Ui};

#[derive(Clone, Debug)]
pub struct Item { pub label: String, pub icon: Option<Icon>, pub right: Option<String>, pub sub: Option<String>, pub enabled: bool }
impl Item {
    pub fn new(label: impl Into<String>) -> Self { Self { label: label.into(), icon: None, right: None, sub: None, enabled: true } }
    pub fn icon(mut self, i: Icon) -> Self { self.icon = Some(i); self }
    pub fn right(mut self, r: impl Into<String>) -> Self { self.right = Some(r.into()); self }
    pub fn sub(mut self, s: impl Into<String>) -> Self { self.sub = Some(s.into()); self }
    pub fn enabled(mut self, e: bool) -> Self { self.enabled = e; self }
}

pub struct Menu<'a> { id: egui::Id, items: &'a [Item], width: f32, font: f32, keyboard: bool, ghost: bool, hand_gap: Option<f32> }
impl<'a> Menu<'a> {
    pub fn new(id_salt: impl std::hash::Hash + std::fmt::Debug, items: &'a [Item]) -> Self { Self { id: egui::Id::unique(("game-ui-menu", id_salt)), items, width: 220.0, font: 17.0, keyboard: true, ghost: false, hand_gap: None } }
    pub fn width(mut self, w: f32) -> Self { self.width = w; self }
    pub fn font(mut self, f: f32) -> Self { self.font = f; self }
    /// 키보드(↑ ↓ Enter)를 받을지 — 한 화면에 메뉴가 여럿이면 하나만
    pub fn keyboard(mut self, k: bool) -> Self { self.keyboard = k; self }
    /// 자식 창이 열린 부모 메뉴 — 흐린 손만 남기고 입력을 받지 않는다
    pub fn ghost(mut self, g: bool) -> Self { self.ghost = g; self }
    /// 손 끝과 줄 왼쪽 사이 (기본: 창 왼쪽 테에 닿게 5u)
    pub fn hand_gap(mut self, g: f32) -> Self { self.hand_gap = Some(g); self }

    /// 고른 줄을 정한다 (처음 열 때 · 부모 메뉴의 흐린 손 자리)
    pub fn select(ctx: &egui::Context, id_salt: impl std::hash::Hash + std::fmt::Debug, k: usize) { ctx.data_mut(|d| d.insert_temp(egui::Id::unique(("game-ui-menu", id_salt)), k)); }
    /// 지금 고른 줄 (마지막으로 그린 값)
    pub fn current(ctx: &egui::Context, id_salt: impl std::hash::Hash + std::fmt::Debug) -> Option<usize> { ctx.data(|d| d.get_temp(egui::Id::unique(("game-ui-menu", id_salt)))) }

    /// 누른 줄 번호
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let ctx = ui.ctx().clone(); let pl = skin::pal(&ctx); let sf = skin::surface(); let u = unit(&ctx); let t = ctx.input(|i| i.time);
        let mut cur: usize = ctx.data(|d| d.get_temp(self.id)).unwrap_or_else(|| self.items.iter().position(|i| i.enabled).unwrap_or(0));
        let mut picked = None;
        let pitch = (self.font * 1.65).max(14.0 * u);
        let row_h = |it: &Item| if it.sub.is_some() { pitch + self.font * 0.85 } else { pitch };
        let mut rows = Vec::new();
        for (k, it) in self.items.iter().enumerate() {
            let (r, resp) = ui.allocate_exact_size(vec2(self.width, row_h(it)), if it.enabled && !self.ghost { Sense::click() } else { Sense::hover() });
            if !self.ghost && it.enabled && (resp.hovered() && ctx.input(|i| i.pointer.delta().length_sq() > 0.0) || resp.clicked()) { cur = k; }
            if !self.ghost && it.enabled && resp.clicked() { picked = Some(k); }
            if !self.ghost && it.enabled && resp.hovered() { ctx.set_cursor_icon(egui::CursorIcon::PointingHand); }
            rows.push(r);
        }
        if self.keyboard && !self.ghost {
            let n = self.items.len();
            let step = |c: usize, d: i32| -> usize { let mut c = c as i32; for _ in 0..n { c = (c + d).rem_euclid(n as i32); if self.items[c as usize].enabled { break; } } c as usize };
            ctx.input(|i| { if i.key_pressed(Key::ArrowDown) { cur = step(cur, 1); } if i.key_pressed(Key::ArrowUp) { cur = step(cur, -1); } if i.key_pressed(Key::Enter) && self.items.get(cur).map(|x| x.enabled).unwrap_or(false) { picked = Some(cur); } });
        }
        cur = cur.min(self.items.len().saturating_sub(1));
        ctx.data_mut(|d| d.insert_temp(self.id, cur));
        let p = ui.painter();
        let lined = sf != Surface::Light;
        // 줄 구분선 (TIC 목록)
        if lined { for r in rows.iter().skip(1) { p.rect_filled(Rect::from_min_size(pos2(r.min.x, r.min.y - 1.0), vec2(r.width(), 1.0)), 0.0, pl.select_line); p.rect_filled(Rect::from_min_size(pos2(r.min.x, r.min.y), vec2(r.width(), 1.0)), 0.0, egui::Color32::from_white_alpha(if sf == Surface::Dark { 10 } else { 90 })); } }
        // 고른 줄 밝힘 (클래식 판은 없음)
        if lined && !self.ghost { if let Some(r) = rows.get(cur) { crate::paint::hgrad(p, Rect::from_min_max(pos2(r.min.x, r.min.y + 1.0), r.max), pl.select_top, pl.select_bottom); } }
        let x0 = 3.0 * u;
        for (k, (it, r)) in self.items.iter().zip(&rows).enumerate() {
            let col = if !it.enabled { pl.disabled } else if k == cur && sf != Surface::Light { pl.title } else { pl.text };
            let mut x = r.min.x + x0;
            let cy = if it.sub.is_some() { r.min.y + pitch * 0.5 } else { r.center().y };
            if let Some(ic) = it.icon { let s = (self.font * 1.15).round(); icons::paint(p, Rect::from_center_size(pos2(x + s * 0.5, cy), vec2(s, s)), ic, if it.enabled { None } else { Some(pl.disabled) }); x += s + 2.0 * u; }
            text::shadowed(p, pos2(x, cy), Align2::LEFT_CENTER, &it.label, fonts::bold(self.font), col, if it.enabled { pl.shadow } else { egui::Color32::TRANSPARENT });
            if let Some(rt) = &it.right { text::shadowed(p, pos2(r.max.x - 3.0 * u, cy), Align2::RIGHT_CENTER, rt, fonts::num(self.font * 0.85), if it.enabled { pl.accent } else { pl.disabled }, pl.shadow); }
            if let Some(s) = &it.sub { p.text(pos2(x, r.min.y + pitch * 0.5 + self.font * 0.95), Align2::LEFT_CENTER, s, fonts::bold(self.font * 0.7), pl.dim); }
        }
        if let Some(r) = rows.get(cur) {
            let cy = if self.items[cur].sub.is_some() { r.min.y + pitch * 0.5 } else { r.center().y };
            let tip = pos2(r.min.x - self.hand_gap.unwrap_or(5.0 * u), cy);
            if self.ghost { cursor::ghost(p, tip, 10.0 * u); } else { cursor::hand(p, tip, 10.0 * u, t); ctx.request_repaint_after(std::time::Duration::from_millis(250)); }
        }
        picked
    }
}
