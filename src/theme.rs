//! 테마 (이슈 #22) — 밤하늘 남색 판 + 호박색 강조. 모서리 둥글게, 여백 넉넉히, 글자 크기 단계.
use egui::{Color32, CornerRadius, FontId, Shadow, Stroke, TextStyle};

pub const BG: Color32 = Color32::from_rgb(11, 14, 21);
pub const PANEL: Color32 = Color32::from_rgb(16, 20, 29);
pub const CARD: Color32 = Color32::from_rgb(22, 28, 40);
pub const CARD_HI: Color32 = Color32::from_rgb(32, 40, 56);
pub const LINE: Color32 = Color32::from_rgb(44, 54, 74);
pub const TEXT: Color32 = Color32::from_rgb(226, 230, 238);
pub const DIM: Color32 = Color32::from_rgb(150, 158, 175);
pub const ACCENT: Color32 = Color32::from_rgb(242, 193, 78);
pub const ACCENT_DIM: Color32 = Color32::from_rgb(120, 92, 32);
pub const GOOD: Color32 = Color32::from_rgb(125, 220, 138);
pub const BAD: Color32 = Color32::from_rgb(255, 107, 107);
pub const AMBIENT: Color32 = Color32::from_rgb(110, 205, 160);
pub const HARDCORE: Color32 = Color32::from_rgb(240, 96, 72);

pub fn apply(ctx: &egui::Context, scale: f32) {
    ctx.set_theme(egui::Theme::Dark);
    ctx.all_styles_mut(|s| {
        s.text_styles = [
            (TextStyle::Heading, FontId::new(22.0, egui::FontFamily::Name(crate::fonts::BOLD.into()))),
            (TextStyle::Body, FontId::proportional(14.0)),
            (TextStyle::Button, FontId::proportional(14.0)),
            (TextStyle::Small, FontId::proportional(11.5)),
            (TextStyle::Monospace, FontId::monospace(13.0)),
        ].into();
        s.spacing.item_spacing = egui::vec2(8.0, 6.0); s.spacing.button_padding = egui::vec2(10.0, 5.0); s.spacing.window_margin = egui::Margin::same(14);
        s.spacing.interact_size.y = 24.0;
        let v = &mut s.visuals;
        v.panel_fill = PANEL; v.window_fill = CARD; v.extreme_bg_color = BG; v.faint_bg_color = CARD; v.override_text_color = None;
        v.window_corner_radius = CornerRadius::same(12); v.window_stroke = Stroke::new(1.0, LINE);
        v.window_shadow = Shadow { offset: [0, 8], blur: 28, spread: 0, color: Color32::from_black_alpha(140) };
        v.popup_shadow = Shadow { offset: [0, 4], blur: 16, spread: 0, color: Color32::from_black_alpha(120) };
        v.hyperlink_color = ACCENT; v.selection.bg_fill = ACCENT_DIM; v.selection.stroke = Stroke::new(1.0, ACCENT);
        for (w, bg, stroke) in [(&mut v.widgets.noninteractive, CARD, LINE), (&mut v.widgets.inactive, CARD_HI, LINE), (&mut v.widgets.hovered, Color32::from_rgb(44, 54, 76), ACCENT_DIM), (&mut v.widgets.active, Color32::from_rgb(58, 70, 96), ACCENT), (&mut v.widgets.open, CARD_HI, LINE)] {
            w.bg_fill = bg; w.weak_bg_fill = bg; w.bg_stroke = Stroke::new(1.0, stroke); w.corner_radius = CornerRadius::same(7);
        }
        v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT); v.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT); v.widgets.hovered.fg_stroke = Stroke::new(1.5, Color32::WHITE); v.widgets.active.fg_stroke = Stroke::new(1.5, Color32::WHITE);
    });
    ctx.set_zoom_factor(scale.clamp(0.7, 1.8));
}

/// 카드 틀 (패널 안 묶음)
pub fn card() -> egui::Frame { egui::Frame::new().fill(CARD).stroke(Stroke::new(1.0, LINE)).corner_radius(CornerRadius::same(10)).inner_margin(egui::Margin::same(10)) }
/// 화면 위에 뜨는 판 (메뉴·배너)
pub fn sheet() -> egui::Frame { egui::Frame::new().fill(Color32::from_rgba_unmultiplied(16, 20, 29, 242)).stroke(Stroke::new(1.0, LINE)).corner_radius(CornerRadius::same(14)).inner_margin(egui::Margin::same(22)).shadow(Shadow { offset: [0, 10], blur: 36, spread: 0, color: Color32::from_black_alpha(160) }) }
