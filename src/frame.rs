//! 창 — 원작에서 잰 그대로 그린다 (u = 원작 화소 하나, 모든 선은 u 두께로 채운 사각형, 물리 화소에 스냅).
//! - 클래식 판 (Menu · Message · Plate · Tip · Slate): 1u 딱딱한 그림자(+1,+1) → 1u 먹선 → 질감 채움 → 위 1u 하이라이트 → 오른 1u · 아래 1u 그늘.
//!   모서리 네모 · 왼쪽 하이라이트 없음 · 세로 그라데이션 없음 [M].
//! - TIC 페이지 (Page): 평평한 크림 + 위 1점 선 두 줄 + 종이 두께처럼 안쪽으로 어두워지는 테 + 부드러운 그림자.
//! - HUD (Hud · HudPlate): 남색 가로 그라데이션, 테 없음, 오른쪽 끝에서 옅어져 사라진다.
//! - 양피지 (Parchment): 질감 + 18u에 걸쳐 그을린 가장자리 + 들쭉날쭉한 종이 끝.
//! 내용을 먼저 그리고 그 뒤에 창을 깐다 (자리만 잡아 둔 Shape를 나중에 채운다). 내용을 그리는 동안 표면을 알린다(`skin::within`).
use crate::paint::{fill, hgrad_shape, ring_shape, tex_shape, Tex};
use crate::skin::{self, snap_rect, unit, Skin, Surface};
use crate::{fonts, text};
use egui::{pos2, vec2, Align2, Color32, InnerResponse, Margin, Pos2, Rect, Shape, Stroke, Ui, UiBuilder};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// 명령 · 메뉴 · 도구 창 (클래식 판)
    Menu,
    /// 대사 · 알림 창 (클래식 판, 여백 넓게)
    Message,
    /// 이름표 · 작은 판 (클래식 판, 여백 좁게)
    Plate,
    /// 툴팁 (클래식 판)
    Tip,
    /// 비활성 부모 창 (슬레이트 판 — 자식 창이 열리면 부모가 이 색으로 바뀐다)
    Slate,
    /// 읽는 화면 (TIC 크림 페이지)
    Page,
    /// 3D 위 상태판 (TIC 남색, 오른쪽으로 사라짐)
    Hud,
    /// 3D 위 작은 판 (남색, 사라짐 없이)
    HudPlate,
    /// 연대기 · 역사 (그을린 양피지)
    Parchment,
}
impl Kind {
    pub fn surface(self) -> Surface {
        match self { Kind::Hud | Kind::HudPlate => Surface::Dark, Kind::Parchment => Surface::Parchment, Kind::Page => Surface::Page, _ => Surface::Light }
    }
}

/// 클래식 판 한 장 (리서치 §2.2 표 그대로)
fn classic(v: &mut Vec<Shape>, ctx: &egui::Context, r: Rect, u: f32, tex: Tex, out: Color32, hi: Color32, sh_r: Color32, sh_b: Color32, shadow: Color32) {
    v.push(fill(r.translate(vec2(u, u)), shadow));
    v.push(fill(r, out));
    let i = r.shrink(u);
    v.push(tex_shape(ctx, i, tex, u, Color32::WHITE));
    v.push(fill(Rect::from_min_size(i.min, vec2(i.width(), u)), hi));
    v.push(fill(Rect::from_min_max(pos2(i.max.x - u, i.min.y + u), i.max), sh_r));
    v.push(fill(Rect::from_min_max(pos2(i.min.x, i.max.y - u), pos2(i.max.x - u, i.max.y)), sh_b));
}

fn deckle(r: Rect, u: f32) -> Vec<Pos2> {
    let h = |k: u32| { let mut x = k.wrapping_mul(2654435761); x ^= x >> 15; x = x.wrapping_mul(2246822519); x ^= x >> 13; (x % 1000) as f32 / 1000.0 - 0.5 };
    let step = 4.0 * u; let mut pts = Vec::new(); let mut k = 0u32;
    let mut edge = |a: Pos2, b: Pos2, nrm: egui::Vec2, pts: &mut Vec<Pos2>| { let n = ((b - a).length() / step).max(1.0) as usize; for i in 0..n { let t = i as f32 / n as f32; k += 1; pts.push(a + (b - a) * t + nrm * (h(k) * 2.0 * u)); } };
    edge(r.left_top(), r.right_top(), vec2(0.0, 1.0), &mut pts);
    edge(r.right_top(), r.right_bottom(), vec2(-1.0, 0.0), &mut pts);
    edge(r.right_bottom(), r.left_bottom(), vec2(0.0, -1.0), &mut pts);
    edge(r.left_bottom(), r.left_top(), vec2(1.0, 0.0), &mut pts);
    pts
}

/// 창 하나를 이루는 모양들 (r = 테두리 포함, 그림자 제외)
pub fn shapes(ctx: &egui::Context, r: Rect, kind: Kind, s: &Skin) -> Vec<Shape> {
    let u = unit(ctx); let r = snap_rect(ctx, r); let px = 1.0 / ctx.pixels_per_point();
    let mut v = Vec::new();
    match kind {
        Kind::Menu | Kind::Message | Kind::Plate | Kind::Tip => classic(&mut v, ctx, r, u, Tex::Plate, s.plate_out, s.plate_hi, s.plate_shade_r, s.plate_shade_b, s.plate_shadow),
        Kind::Slate => classic(&mut v, ctx, r, u, Tex::Slate, s.slate_out, Color32::from_rgb(0xa4, 0xac, 0xb2), Color32::from_rgb(0x3b, 0x41, 0x48), Color32::from_rgb(0x53, 0x5a, 0x61), s.plate_shadow),
        Kind::Page => {
            v.push(Shape::Rect(egui::epaint::Shadow { offset: [0, 4], blur: 14, spread: 0, color: Color32::from_black_alpha(110) }.as_shape(r, 2)));
            v.push(Shape::rect_filled(r, 2.0, s.page_edge[0]));
            v.push(Shape::rect_filled(r.shrink(1.0), 1.5, s.page_edge[1]));
            let f = r.shrink(2.0);
            v.push(fill(f, s.page_edge[2]));
            let body = Rect::from_min_max(pos2(f.min.x + 1.0, f.min.y), pos2(f.max.x - 1.0, f.max.y - 1.0));
            v.push(tex_shape(ctx, body, Tex::Page, u, Color32::WHITE));
            v.push(fill(Rect::from_min_size(body.min, vec2(body.width(), 1.0)), s.page_top));
            v.push(fill(Rect::from_min_size(body.min + vec2(0.0, 1.0), vec2(body.width(), 1.0)), s.page_hi));
        }
        Kind::Hud | Kind::HudPlate => {
            let hair = px.max(1.0);
            if kind == Kind::Hud {
                let w = (r.width() * 0.18).clamp(24.0, 64.0);
                let ext = Rect::from_min_max(pos2(r.max.x, r.min.y), pos2(r.max.x + w, r.max.y));
                v.push(hgrad_shape(r, s.hud_left, s.hud_right));
                v.push(hgrad_shape(ext, s.hud_right, Color32::TRANSPARENT));
                v.push(hgrad_shape(Rect::from_min_max(r.min, pos2(ext.max.x, r.min.y + hair)), Color32::from_rgba_unmultiplied(0xf3, 0xee, 0xd0, 80), Color32::TRANSPARENT));
                v.push(hgrad_shape(Rect::from_min_max(pos2(r.min.x, r.max.y - hair), pos2(ext.max.x, r.max.y)), Color32::from_rgba_unmultiplied(0, 0, 0, 120), Color32::TRANSPARENT));
            } else {
                v.push(Shape::Rect(egui::epaint::Shadow { offset: [0, 2], blur: 8, spread: 0, color: Color32::from_black_alpha(60) }.as_shape(r, 0)));
                v.push(hgrad_shape(r, s.hud_left, s.hud_right));
                v.push(fill(Rect::from_min_size(r.min, vec2(r.width(), hair)), Color32::from_rgba_unmultiplied(0xf3, 0xee, 0xd0, 60)));
                v.push(fill(Rect::from_min_max(pos2(r.min.x, r.max.y - hair), r.max), Color32::from_rgba_unmultiplied(0, 0, 0, 120)));
            }
        }
        Kind::Parchment => {
            v.push(Shape::Rect(egui::epaint::Shadow { offset: [3, 4], blur: 12, spread: 0, color: Color32::from_black_alpha(100) }.as_shape(r, 0)));
            let pts = deckle(r, u);
            v.push(Shape::convex_polygon(pts.clone(), s.parch_edge, Stroke::NONE));
            v.push(tex_shape(ctx, r.shrink(u), Tex::Parchment, u * 0.5, Color32::WHITE));
            let e = s.parch_edge; let burn = (18.0 * u).min(r.width().min(r.height()) * 0.3);
            v.push(ring_shape(r.shrink(u), burn, Color32::from_rgba_unmultiplied(e.r(), e.g(), e.b(), 235), Color32::from_rgba_unmultiplied(e.r(), e.g(), e.b(), 0)));
            v.push(Shape::closed_line(pts, Stroke::new(1.0, Color32::from_rgb(0x6a, 0x52, 0x36))));
        }
    }
    v
}

/// 창을 그 자리에 그린다 (내용 없이)
pub fn paint(p: &egui::Painter, r: Rect, kind: Kind) { let s = skin::get(p.ctx()); p.extend(shapes(p.ctx(), r, kind, &s)); }

/// 안쪽 여백 (u 단위 → 점)
pub fn padding(ctx: &egui::Context, kind: Kind) -> Margin {
    let u = unit(ctx); let m = |x: f32, y: f32| Margin::symmetric((x * u).round() as i8, (y * u).round() as i8);
    match kind {
        Kind::Menu | Kind::Slate => m(6.0, 6.0),
        Kind::Message => m(9.0, 7.0),
        Kind::Plate | Kind::HudPlate => m(5.0, 3.0),
        Kind::Tip => m(4.0, 3.0),
        Kind::Hud => m(7.0, 6.0),
        Kind::Page => m(8.0, 7.0),
        Kind::Parchment => m(14.0, 11.0),
    }
}

/// 표면에 맞춰 기본 글자색 (egui 기본 위젯 — 라벨 · 입력칸)
fn ink(ui: &mut Ui, sf: Surface) {
    let pl = skin::get(ui.ctx()).pal(sf);
    let v = ui.visuals_mut();
    v.override_text_color = Some(pl.text);
    for w in [&mut v.widgets.noninteractive, &mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open] { w.fg_stroke.color = pl.text; }
    v.extreme_bg_color = pl.well; v.text_edit_bg_color = Some(pl.well);
    v.selection.bg_fill = pl.select_line.gamma_multiply(0.6);
}

/// 내용이 든 창 (크기는 내용에 맞춘다). 내용을 그리는 동안 이 창의 표면을 알린다
pub fn show<R>(ui: &mut Ui, kind: Kind, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let s = skin::get(ui.ctx());
    let pad = padding(ui.ctx(), kind);
    let slot = ui.painter().add(Shape::Noop);
    let sf = kind.surface();
    let inner = skin::within(sf, || egui::Frame::new().inner_margin(pad).show(ui, |ui| { ink(ui, sf); add(ui) }));
    let r = inner.response.rect;
    ui.painter().set(slot, Shape::Vec(shapes(ui.ctx(), r, kind, &s)));
    inner
}

/// 제목이 붙은 창 — 클래식 판은 윗테 위 꼬리표, 페이지는 머리띠, 양피지는 가운데 제목 + 금줄, HUD는 작은 머리글
pub fn show_titled<R>(ui: &mut Ui, kind: Kind, title: &str, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let u = unit(ui.ctx());
    match kind {
        Kind::Page => show(ui, kind, |ui| { header_band(ui, title); add(ui) }),
        Kind::Parchment => show(ui, kind, |ui| { page_title(ui, title); add(ui) }),
        Kind::Hud | Kind::HudPlate => show(ui, kind, |ui| {
            let pl = skin::pal(ui.ctx());
            let (r, _) = ui.allocate_exact_size(vec2(ui.available_width().max(40.0), 9.0 * u), egui::Sense::hover());
            text::shadowed(ui.painter(), r.left_center(), Align2::LEFT_CENTER, title, fonts::bold(12.5), pl.dim, pl.shadow);
            add(ui)
        }),
        _ => {
            ui.add_space(5.0 * u);
            let res = show(ui, kind, |ui| { ui.add_space(3.0 * u); add(ui) });
            tab_label(&ui.painter().clone(), res.response.rect, title);
            res
        }
    }
}

/// 정해진 rect 안에 창 + 내용
pub fn show_in<R>(ui: &mut Ui, r: Rect, kind: Kind, add: impl FnOnce(&mut Ui) -> R) -> R {
    let s = skin::get(ui.ctx());
    ui.painter().extend(shapes(ui.ctx(), r, kind, &s));
    let pad = padding(ui.ctx(), kind); let sf = kind.surface();
    let inner = Rect::from_min_max(r.min + vec2(pad.left as f32, pad.top as f32), r.max - vec2(pad.right as f32, pad.bottom as f32));
    skin::within(sf, || ui.scope_builder(UiBuilder::new().max_rect(inner), |ui| { ink(ui, sf); add(ui) }).inner)
}

/// 꼬리표 — 판 윗테 왼쪽에 걸친 작은 판 + 크림 글자에 먹 윤곽 (원작 'Menu' · 'Ability' 꼬리표)
pub fn tab_label(p: &egui::Painter, plate: Rect, t: &str) {
    let ctx = p.ctx().clone(); let s = skin::get(&ctx); let u = unit(&ctx);
    let font = fonts::heavy(13.0);
    let g = p.layout_no_wrap(t.to_string(), font.clone(), Color32::WHITE);
    let h = 10.0 * u; let w = g.size().x + 8.0 * u;
    let r = snap_rect(&ctx, Rect::from_min_size(pos2(plate.min.x + 3.0 * u, plate.min.y - h * 0.55), vec2(w, h)));
    let mut v = Vec::new();
    classic(&mut v, &ctx, r, u, Tex::Plate, s.plate_out, s.plate_hi, s.plate_shade_r, s.plate_shade_b, s.plate_shadow);
    p.extend(v);
    text::outlined(p, r.center() + vec2(0.0, -0.5), Align2::CENTER_CENTER, t, font, Color32::from_rgb(0xf2, 0xee, 0xe0), s.plate_out);
}

/// TIC 머리띠 — 페이지 위쪽을 가로지르는 #d6cbb9 띠 + 제목 (창 안쪽 여백을 넘어 테까지 칠한다)
pub fn header_band(ui: &mut Ui, title: &str) {
    let ctx = ui.ctx().clone(); let s = skin::get(&ctx); let u = unit(&ctx); let pad = padding(&ctx, Kind::Page); let pl = s.pal(Surface::Page);
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 14.0 * u), egui::Sense::hover());
    let band = Rect::from_min_max(pos2(r.min.x - pad.left as f32 + 3.0, r.min.y - pad.top as f32 + 4.0), pos2(r.max.x + pad.right as f32 - 3.0, r.max.y));
    let p = ui.painter();
    p.add(fill(band, s.page_header));
    p.add(fill(Rect::from_min_max(pos2(band.min.x, band.max.y - 1.0), band.max), s.page_rule.gamma_multiply(0.9)));
    p.add(fill(Rect::from_min_max(pos2(band.min.x, band.max.y), pos2(band.max.x, band.max.y + 1.0)), s.page_hi));
    p.text(pos2(r.min.x, r.center().y + 1.0), Align2::LEFT_CENTER, title, fonts::bold(15.0), pl.title);
    ui.add_space(3.0 * u);
}

/// 양피지 제목 — 가운데 굵은 제목 + 마름모 끝 금줄
pub fn page_title(ui: &mut Ui, title: &str) {
    let ctx = ui.ctx().clone(); let s = skin::get(&ctx); let u = unit(&ctx); let pl = s.pal(Surface::Parchment);
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 20.0 * u), egui::Sense::hover());
    let p = ui.painter();
    let tr = p.text(pos2(r.center().x, r.min.y + 7.0 * u), Align2::CENTER_CENTER, title, fonts::heavy(18.0), pl.title);
    gold_rule(p, r.center().x, (tr.width() * 0.5 + 24.0 * u).min(r.width() * 0.5 - 2.0 * u), r.min.y + 15.0 * u, s.gold_rule);
}

/// 금줄 — 1u 선 + 양 끝 3u 마름모 + 가운데 ◆◇◆ (리서치 §5.11 GoldRule)
pub fn gold_rule(p: &egui::Painter, cx: f32, half: f32, y: f32, c: Color32) {
    let u = unit(p.ctx()); let y = skin::snap(p.ctx(), y);
    let dia = |p: &egui::Painter, at: Pos2, d: f32, hollow: bool| {
        p.add(Shape::convex_polygon(vec![at + vec2(0.0, -d), at + vec2(d, 0.0), at + vec2(0.0, d), at + vec2(-d, 0.0)], c, Stroke::NONE));
        if hollow { let k = d - u; if k > 0.0 { p.add(Shape::convex_polygon(vec![at + vec2(0.0, -k), at + vec2(k, 0.0), at + vec2(0.0, k), at + vec2(-k, 0.0)], c.gamma_multiply(0.25), Stroke::NONE)); } }
    };
    let gap = 9.0 * u;
    for side in [-1.0f32, 1.0] {
        let (a, b) = (cx + side * gap, cx + side * half);
        p.add(fill(Rect::from_min_max(pos2(a.min(b), y), pos2(a.max(b), y + u * 0.5)), c));
        dia(p, pos2(b, y + u * 0.25), 1.5 * u, false);
        dia(p, pos2(cx + side * 5.0 * u, y + u * 0.25), 1.5 * u, false);
    }
    dia(p, pos2(cx, y + u * 0.25), 2.5 * u, true);
}
