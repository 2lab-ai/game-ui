//! 테마 — egui 기본 위젯(입력칸 · 스크롤 막대 · 툴팁 · 기본 창)도 클래식 판 언어로: 카키 판 · 먹 글자 · 네모 모서리 · 딱딱한 그림자.
//! 게임 화면의 창은 `frame`이 그리고, 여기는 그 밖에 남는 egui 기본 모양만 맞춘다.
use egui::{Color32, CornerRadius, FontId, Shadow, Stroke, TextStyle};

pub const BG: Color32 = Color32::from_rgb(0x19, 0x19, 0x19);
pub const PANEL: Color32 = Color32::from_rgb(0xa5, 0x9d, 0x83);
pub const CARD: Color32 = Color32::from_rgb(0xa5, 0x9d, 0x83);
pub const CARD_HI: Color32 = Color32::from_rgb(0xb4, 0xac, 0x93);
pub const LINE: Color32 = Color32::from_rgb(0x2a, 0x21, 0x19);
pub const TEXT: Color32 = Color32::from_rgb(0x42, 0x3b, 0x30);
pub const DIM: Color32 = Color32::from_rgb(0x5f, 0x57, 0x46);
/// 강조색은 없다 — 이름 먹(붉은 갈색)
pub const ACCENT: Color32 = Color32::from_rgb(0x6e, 0x31, 0x19);
pub const ACCENT_DIM: Color32 = Color32::from_rgb(0x84, 0x7c, 0x6a);
pub const GOOD: Color32 = Color32::from_rgb(0x5e, 0x8a, 0x3a);
pub const BAD: Color32 = Color32::from_rgb(0xa8, 0x32, 0x1e);
pub const AMBIENT: Color32 = Color32::from_rgb(0x5e, 0x8a, 0x3a);
pub const HARDCORE: Color32 = Color32::from_rgb(0x8c, 0x2a, 0x1c);

pub fn apply(ctx: &egui::Context, scale: f32) {
    ctx.set_theme(egui::Theme::Light);
    ctx.all_styles_mut(|s| {
        s.text_styles = [
            (TextStyle::Heading, FontId::new(20.0, egui::FontFamily::Name(crate::fonts::HEAVY.into()))),
            (TextStyle::Body, FontId::new(14.0, egui::FontFamily::Name(crate::fonts::BOLD.into()))),
            (TextStyle::Button, FontId::new(14.0, egui::FontFamily::Name(crate::fonts::BOLD.into()))),
            (TextStyle::Small, FontId::new(12.0, egui::FontFamily::Name(crate::fonts::BOLD.into()))),
            (TextStyle::Monospace, FontId::monospace(13.0)),
        ].into();
        s.spacing.item_spacing = egui::vec2(8.0, 6.0); s.spacing.button_padding = egui::vec2(10.0, 5.0); s.spacing.window_margin = egui::Margin::same(12);
        s.spacing.interact_size.y = 24.0; s.spacing.scroll.bar_width = 6.0; s.spacing.scroll.floating = false;
        let v = &mut s.visuals;
        v.dark_mode = false;
        v.panel_fill = PANEL; v.window_fill = CARD; v.extreme_bg_color = Color32::from_rgb(0x8d, 0x85, 0x6d); v.faint_bg_color = CARD_HI; v.code_bg_color = CARD_HI;
        v.text_edit_bg_color = Some(Color32::from_rgb(0x8d, 0x85, 0x6d));
        v.override_text_color = None;
        v.window_corner_radius = CornerRadius::ZERO; v.menu_corner_radius = CornerRadius::ZERO; v.window_stroke = Stroke::new(2.0, LINE);
        v.window_shadow = Shadow { offset: [2, 2], blur: 0, spread: 0, color: Color32::from_rgb(0x20, 0x17, 0x11) };
        v.popup_shadow = Shadow { offset: [2, 2], blur: 0, spread: 0, color: Color32::from_rgb(0x20, 0x17, 0x11) };
        v.hyperlink_color = ACCENT; v.selection.bg_fill = Color32::from_rgb(0x84, 0x7c, 0x6a); v.selection.stroke = Stroke::new(1.0, TEXT);
        v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(0x74, 0x69, 0x53));
        for (w, bg, stroke) in [(&mut v.widgets.noninteractive, CARD, Color32::from_rgb(0x74, 0x69, 0x53)), (&mut v.widgets.inactive, Color32::from_rgb(0x9a, 0x92, 0x7b), LINE), (&mut v.widgets.hovered, CARD_HI, LINE), (&mut v.widgets.active, Color32::from_rgb(0xd6, 0xcf, 0xad), LINE), (&mut v.widgets.open, CARD_HI, LINE)] {
            w.bg_fill = bg; w.weak_bg_fill = bg; w.bg_stroke = Stroke::new(1.0, stroke); w.corner_radius = CornerRadius::ZERO; w.fg_stroke = Stroke::new(1.0, TEXT); w.expansion = 0.0;
        }
        v.widgets.hovered.fg_stroke = Stroke::new(1.5, Color32::from_rgb(0x2a, 0x21, 0x19)); v.widgets.active.fg_stroke = Stroke::new(1.5, Color32::from_rgb(0x2a, 0x21, 0x19));
    });
    ctx.set_zoom_factor(scale.clamp(0.7, 1.8));
}

/// 카드 틀 (egui Frame — 클래식 판 근사, 질감 없이)
pub fn card() -> egui::Frame { egui::Frame::new().fill(CARD).stroke(Stroke::new(2.0, LINE)).corner_radius(CornerRadius::ZERO).inner_margin(egui::Margin::same(10)).shadow(Shadow { offset: [2, 2], blur: 0, spread: 0, color: Color32::from_rgb(0x20, 0x17, 0x11) }) }
/// 화면 위에 뜨는 판
pub fn sheet() -> egui::Frame { card().inner_margin(egui::Margin::same(18)) }
