//! 메시지 — 대사 창(화자 이름이 윗테에 걸친다 · 계속 표시) · 알림 판(꼬리표 · 아이콘 원 · 「이름」 · 짧은 명령 목록과 손).
//! 원작: 이름은 붉은 갈색 먹(#6e3119), 계속 표시는 1u씩 두 단으로 까딱이는 금갈색 삼각(#856833), 알림은 화면 위 가운데 한 줄 판.
use crate::menu::{Item, Menu};
use crate::skin::{self, unit, Surface};
use crate::{fonts, frame::{self, Kind}, icons::{self, Icon}, text};
use egui::{pos2, vec2, Align2, Color32, Rect, Shape, Stroke, Ui};

/// 계속 표시 — 아래를 가리키는 삼각, 0.8초 주기로 1u 위아래 (계단식)
pub fn continue_arrow(p: &egui::Painter, at: egui::Pos2, t: f64) {
    let s = skin::get(p.ctx()); let u = unit(p.ctx()); let dy = if t.rem_euclid(0.8) < 0.4 { 0.0 } else { u }; let c = at + vec2(0.0, dy);
    p.add(Shape::convex_polygon(vec![c + vec2(-4.0 * u, -3.0 * u), c + vec2(4.0 * u, -3.0 * u), c + vec2(0.0, 3.0 * u)], s.plate_out, Stroke::NONE));
    p.add(Shape::convex_polygon(vec![c + vec2(-3.0 * u, -2.5 * u), c + vec2(3.0 * u, -2.5 * u), c + vec2(0.0, 1.8 * u)], s.continue_mark, Stroke::NONE));
}

/// 아이콘 원 — TIC 명령 아이콘처럼 짙은 원판(#453c33) 위 아이콘. tone은 원판 테에 아주 옅게만
pub fn medallion(p: &egui::Painter, c: egui::Pos2, r: f32, icon: Icon, tone: Color32) {
    let u = unit(p.ctx());
    p.circle_filled(c + vec2(0.0, u), r + u, Color32::from_black_alpha(90));
    p.circle_filled(c, r + u, Color32::from_rgb(0x1a, 0x16, 0x13));
    p.circle_filled(c, r, Color32::from_rgb(0x45, 0x3c, 0x33).lerp_to_gamma(tone, 0.12));
    p.circle_stroke(c, r - u, Stroke::new(1.0, Color32::from_white_alpha(22)));
    icons::paint(p, Rect::from_center_size(c, vec2(r * 1.3, r * 1.3)), icon, None);
}

/// 대사 창 — 클래식 판, 화자 이름이 윗테 왼쪽에 걸친다(크림 글자 + 먹 윤곽), 본문 먹, 오른쪽 아래 계속 표시
pub fn dialogue(p: &egui::Painter, r: Rect, speaker: Option<&str>, body: &str, t: f64) {
    let s = skin::get(p.ctx()); let u = unit(p.ctx()); let pl = s.pal(Surface::Light);
    frame::paint(p, r, Kind::Message);
    if let Some(sp) = speaker { text::outlined(p, pos2(r.min.x + 8.0 * u, r.min.y + u), Align2::LEFT_CENTER, sp, fonts::heavy(18.0), Color32::from_rgb(0xf2, 0xee, 0xe0), s.plate_out); }
    let g = p.layout(body.to_string(), fonts::bold(17.0), Color32::PLACEHOLDER, r.width() - 26.0 * u);
    let at = r.min + vec2(11.0 * u, 10.0 * u);
    p.galley_with_override_text_color(at + vec2(u * 0.5, u * 0.5), g.clone(), pl.shadow);
    p.galley(at, g, pl.text);
    continue_arrow(p, pos2(r.max.x - 9.0 * u, r.max.y - 7.0 * u), t);
}

/// 알림 — 큰 사건: 꼬리표(tag) · 아이콘 원 · 제목(붉은 갈색 먹) · 본문 · 도움말 · 오른쪽 짧은 명령 목록(보러 가기 / 도구 / 닫기).
/// 작은 사건(big = false): 한 줄 판 — 아이콘 · 제목 · 본문 · 보러 가기 칸 · 도구 칸 · 닫기 칸
pub struct Notice<'a> { pub tag: &'a str, pub icon: Icon, pub tone: Color32, pub title: &'a str, pub body: &'a str, pub hint: Option<&'a str>, pub action: Option<(Icon, &'a str)>, pub big: bool }
/// 돌려주는 값 = (보러 가기, 도구, 닫기) 눌림
pub fn notice(ui: &mut Ui, n: &Notice, width: f32) -> (bool, bool, bool) {
    let (mut go, mut act, mut close) = (false, false, false);
    let u = unit(ui.ctx());
    if !n.big {
        // 작은 사건 — 원작 알림 판처럼 한 줄: 아이콘 · 제목 · 본문(한 줄) · 보러 가기 · (도구) · 닫기 칸
        let res = frame::show_titled(ui, Kind::Plate, n.tag, |ui| {
            ui.set_width(width - 10.0 * u);
            let pl = skin::pal(ui.ctx());
            ui.horizontal(|ui| { ui.spacing_mut().item_spacing.x = 3.0 * u;
                let (ir, _) = ui.allocate_exact_size(vec2(10.0 * u, 10.0 * u), egui::Sense::hover()); icons::paint(ui.painter(), ir, n.icon, None);
                ui.label(egui::RichText::new(n.title).font(fonts::heavy(15.0)).color(pl.title));
                let right = if n.action.is_some() { 39.0 * u } else { 26.0 * u };
                let bw = (ui.available_width() - right).max(40.0);
                ui.allocate_ui_with_layout(vec2(bw, 10.0 * u), egui::Layout::left_to_right(egui::Align::Center), |ui| { ui.set_width(bw); ui.add(egui::Label::new(egui::RichText::new(n.body).font(fonts::bold(13.5)).color(pl.text)).truncate()); });
                if crate::controls::icon_button(ui, Icon::Eye, false, 11.0 * u).on_hover_text("보러 가기").clicked() { go = true; }
                if let Some((ic, l)) = n.action { if crate::controls::icon_button(ui, ic, false, 11.0 * u).on_hover_text(l).clicked() { act = true; } }
                if crate::controls::icon_button(ui, Icon::Close, false, 11.0 * u).on_hover_text("닫기").clicked() { close = true; }
            });
        });
        let _ = res;
        return (go, act, close);
    }
    frame::show_titled(ui, Kind::Message, n.tag, |ui| {
        let menu_w = 52.0 * u; let gap = 14.0 * u;
        ui.set_width(width - 18.0 * u);
        let pl = skin::pal(ui.ctx());
        ui.horizontal_top(|ui| {
            let m = if n.big { 13.0 * u } else { 10.0 * u };
            let (mr, _) = ui.allocate_exact_size(vec2(m * 2.0 + 2.0 * u, m * 2.0 + 2.0 * u), egui::Sense::hover());
            medallion(ui.painter(), mr.center(), m, n.icon, n.tone);
            ui.add_space(4.0 * u);
            ui.vertical(|ui| {
                ui.set_width((width - 18.0 * u - m * 2.0 - 8.0 * u - menu_w - gap).max(80.0));
                ui.spacing_mut().item_spacing.y = 2.0 * u;
                let (tr, _) = ui.allocate_exact_size(vec2(ui.available_width(), if n.big { 14.0 * u } else { 11.0 * u }), egui::Sense::hover());
                text::shadowed(ui.painter(), tr.left_center(), Align2::LEFT_CENTER, n.title, if n.big { fonts::heavy(20.0) } else { fonts::heavy(16.0) }, pl.title, pl.shadow);
                ui.add(egui::Label::new(egui::RichText::new(n.body).font(fonts::bold(if n.big { 14.5 } else { 13.5 })).color(pl.text)).wrap());
                if let Some(h) = n.hint { ui.add(egui::Label::new(egui::RichText::new(h).font(fonts::bold(12.5)).color(pl.dim)).wrap()); }
            });
            ui.add_space(gap);
            ui.vertical(|ui| {
                let mut items = vec![Item::new("보러 가기").icon(Icon::Eye)];
                if let Some((ic, l)) = n.action { items.push(Item::new(l).icon(ic)); }
                items.push(Item::new("닫기").icon(Icon::Close));
                let salt = ("notice", n.title.to_string());
                if let Some(k) = Menu::new(salt, &items).width(menu_w).font(14.0).keyboard(false).hand_gap(u).show(ui) {
                    if k == 0 { go = true; } else if k == items.len() - 1 { close = true; } else { act = true; }
                }
            });
        });
    });
    (go, act, close)
}
