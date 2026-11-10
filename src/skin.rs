//! 스킨 — FFT(1997 · 2025 이발리스 연대기)에서 잰 색과 치수 (출처: docs/research/fft-ui-research.md, [M] = 픽셀 측정값).
//! 표면 넷:
//! - `Light` 클래식 판: 카키 질감 #a59d83 + 1u 먹선 + 위쪽만 하이라이트 + 오른·아래 그늘 + 딱딱한 1u 그림자. 명령 · 도구 · 알림.
//! - `Page` TIC 크림 페이지: 평평한 #e8dfd0 + 머리띠 #d6cbb9 + 줄 사이 가는 선. 길게 읽는 화면(목록 · 설정 · 새 게임).
//! - `Dark` TIC 전투 HUD: 남색 #132555 → #1a2b59, 오른쪽으로 옅어진다. 3D 위 상태판.
//! - `Parchment` 연대기 종이: 가운데 #daccae → 가장자리 #a28563로 그을림.
//! 장식 원칙: 강조색 없음 · 둥근 카드 없음 · 빛남 없음. 고른 것은 색이 아니라 손 커서가 가리킨다.
//! u = 원작 화소 하나 = 2점(물리 화소에 맞춰 반올림, `unit`).
use egui::Color32;
use std::cell::RefCell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Surface { Light, Page, Dark, Parchment }

/// 한 표면 위의 글자 · 강조 색
#[derive(Clone, Copy, Debug)]
pub struct Pal {
    /// 본문 먹
    pub text: Color32,
    /// 보조 글 (날짜 · 설명)
    pub dim: Color32,
    /// 제목 · 이름 먹 (원작의 붉은 갈색 화자 이름)
    pub title: Color32,
    /// 글자 그림자 (+1u, +1u). 투명이면 그리지 않는다
    pub shadow: Color32,
    pub disabled: Color32,
    /// TIC 줄 밝힘 (왼 → 오른으로 사라짐). 클래식 판은 투명 = 줄 색 안 바꿈
    pub select_top: Color32,
    pub select_bottom: Color32,
    /// 줄 구분선
    pub select_line: Color32,
    /// 숫자 · 오른쪽 값
    pub accent: Color32,
    /// 우묵한 칸 (확인 상자 · 값 칸)
    pub well: Color32,
    pub well_edge: Color32,
    /// 뒤집힌 칸 (고른 탭 칸): 바탕 · 글자
    pub inv_fill: Color32,
    pub inv_text: Color32,
}

#[derive(Clone, Debug)]
pub struct Skin {
    // 클래식 판 [M]
    pub plate_fill: Color32, pub plate_out: Color32, pub plate_hi: Color32, pub plate_shade_r: Color32, pub plate_shade_b: Color32, pub plate_shadow: Color32,
    // 슬레이트(비활성 부모 창) [M]
    pub slate_fill: Color32, pub slate_out: Color32, pub slate_ink: Color32,
    // TIC 페이지 [M]
    pub page_fill: Color32, pub page_header: Color32, pub page_hi: Color32, pub page_top: Color32, pub page_rule: Color32, pub page_gutter: Color32, pub page_edge: [Color32; 3],
    // HUD 남색 [M]
    pub hud_left: Color32, pub hud_right: Color32, pub hud_text: Color32, pub hud_dim: Color32, pub hud_outline: Color32,
    // 양피지 [M]
    pub parch_center: Color32, pub parch_edge: Color32, pub parch_dialog: Color32, pub parch_ink: Color32, pub parch_ink_dim: Color32,
    // 장식
    pub gold: Color32, pub gold_rule: Color32, pub continue_mark: Color32, pub ink_name: Color32,
    // TIC 마름모 단추 [M]
    pub btn_idle: Color32, pub btn_idle_border: Color32, pub btn_sel_top: Color32, pub btn_sel_mid: Color32, pub btn_sel_bottom: Color32, pub btn_sel_glow: Color32, pub btn_sel_border: Color32, pub btn_text: Color32, pub btn_text_sel: Color32,
    // TIC 탭 줄 (어두운 띠)
    pub tab_bar: Color32, pub tab_div: Color32, pub tab_text: Color32, pub tab_sel_top: Color32, pub tab_sel_bottom: Color32,
    // 게이지: 클래식 (왼, 오른) · TIC 관 (위, 속, 아래)
    pub hp: (Color32, Color32), pub mp: (Color32, Color32), pub ct: (Color32, Color32),
    pub hp_tube: [Color32; 3], pub mp_tube: [Color32; 3], pub ct_tube: [Color32; 3],
    pub gauge_out: Color32, pub gauge_empty: Color32,
    // 공통 이름 (앱 호환)
    pub outline: Color32, pub shadow: Color32,
    pub pal_light: Pal, pub pal_page: Pal, pub pal_dark: Pal, pub pal_parch: Pal,
    // 상태색 — 먹 쪽으로 낮춘 값 (네온 금지)
    pub good: Color32, pub bad: Color32, pub warn: Color32, pub plague: Color32, pub ambient: Color32, pub hardcore: Color32,
    pub dark_bottom: Color32,
}

const fn c(r: u8, g: u8, b: u8) -> Color32 { Color32::from_rgb(r, g, b) }
const fn hex(v: u32) -> Color32 { Color32::from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8) }

impl Default for Skin { fn default() -> Self { Self::tactics() } }

impl Skin {
    pub fn tactics() -> Self {
        let ink = hex(0x423b30); let ink_shadow = hex(0x847c6a); let ink_name = hex(0x6e3119);
        let pal_light = Pal {
            text: ink, dim: hex(0x5f5746), title: ink_name, shadow: ink_shadow, disabled: Color32::from_rgba_unmultiplied(0x42, 0x3b, 0x30, 110),
            select_top: Color32::TRANSPARENT, select_bottom: Color32::TRANSPARENT, select_line: hex(0x525341), accent: hex(0x3a2a1a),
            well: hex(0x8d856d), well_edge: hex(0x2a2119), inv_fill: hex(0x2a2119), inv_text: hex(0xd6cfad),
        };
        let pal_page = Pal {
            text: hex(0x3d3020), dim: hex(0x7d6f58), title: hex(0x685032), shadow: Color32::TRANSPARENT, disabled: Color32::from_rgba_unmultiplied(0x3d, 0x30, 0x20, 100),
            select_top: hex(0xf2ecd7), select_bottom: Color32::from_rgba_unmultiplied(0xf2, 0xec, 0xd7, 0), select_line: hex(0xd8ceab), accent: hex(0x3d3020),
            well: hex(0xd8cdb7), well_edge: hex(0x928670), inv_fill: hex(0x3a3526), inv_text: hex(0xf3eed0),
        };
        let pal_dark = Pal {
            text: hex(0xf3eed0), dim: hex(0x8f8e7f), title: hex(0xf3eed0), shadow: Color32::from_black_alpha(150), disabled: Color32::from_rgba_unmultiplied(0x8f, 0x8e, 0x7f, 120),
            select_top: Color32::from_rgba_unmultiplied(0xf3, 0xee, 0xd0, 40), select_bottom: Color32::from_rgba_unmultiplied(0xf3, 0xee, 0xd0, 0), select_line: Color32::from_rgba_unmultiplied(0xf3, 0xee, 0xd0, 40), accent: hex(0xf3eed0),
            well: Color32::from_rgba_unmultiplied(4, 10, 30, 170), well_edge: Color32::from_rgba_unmultiplied(0xf3, 0xee, 0xd0, 50), inv_fill: hex(0xbfb59c), inv_text: hex(0x1a1613),
        };
        let pal_parch = Pal {
            text: hex(0x3d3020), dim: hex(0x7a6448), title: hex(0x685032), shadow: Color32::TRANSPARENT, disabled: Color32::from_rgba_unmultiplied(0x3d, 0x30, 0x20, 100),
            select_top: Color32::from_rgba_unmultiplied(0xf2, 0xe6, 0xc4, 150), select_bottom: Color32::from_rgba_unmultiplied(0xf2, 0xe6, 0xc4, 0), select_line: hex(0xbb9d66), accent: hex(0x3d3020),
            well: hex(0xc8b48e), well_edge: hex(0x856833), inv_fill: hex(0x3d3020), inv_text: hex(0xe8dcc0),
        };
        Self {
            plate_fill: hex(0xa59d83), plate_out: hex(0x2a2119), plate_hi: hex(0xd6cfad), plate_shade_r: hex(0x525341), plate_shade_b: hex(0x746953), plate_shadow: hex(0x201711),
            slate_fill: hex(0x6b737b), slate_out: hex(0x293139), slate_ink: hex(0x2f373e),
            page_fill: hex(0xe8dfd0), page_header: hex(0xd6cbb9), page_hi: hex(0xfaf1e0), page_top: hex(0xcac1b2), page_rule: hex(0xd8ceab), page_gutter: hex(0xd8cdb7),
            page_edge: [hex(0x3a3526), hex(0x928670), hex(0xded6c6)],
            hud_left: Color32::from_rgba_unmultiplied(0x13, 0x25, 0x55, 238), hud_right: Color32::from_rgba_unmultiplied(0x1a, 0x2b, 0x59, 205), hud_text: hex(0xf7f5ed), hud_dim: hex(0x8f8e7f), hud_outline: hex(0x010c00),
            parch_center: hex(0xdaccae), parch_edge: hex(0xa28563), parch_dialog: hex(0xd6c49d), parch_ink: hex(0x3d3020), parch_ink_dim: hex(0x7a6448),
            gold: hex(0xd7b172), gold_rule: hex(0xbb9d66), continue_mark: hex(0x856833), ink_name,
            btn_idle: hex(0x161513), btn_idle_border: hex(0x342f2b), btn_sel_top: hex(0x2d2615), btn_sel_mid: hex(0x372e1a), btn_sel_bottom: hex(0x826a44), btn_sel_glow: hex(0xd7b172), btn_sel_border: hex(0xaa956a), btn_text: hex(0xcfc4a8), btn_text_sel: hex(0xfaf1e0),
            tab_bar: hex(0x1a1613), tab_div: hex(0x2e2b26), tab_text: hex(0x8a8270), tab_sel_top: hex(0xbfb59c), tab_sel_bottom: hex(0x9d8f6e),
            hp: (hex(0x496a7b), hex(0xa8b871)), mp: (hex(0x85423a), hex(0xe19c51)), ct: (hex(0x546b41), hex(0xacac3c)),
            hp_tube: [hex(0x91cb14), hex(0x6d9221), hex(0x88b90f)], mp_tube: [hex(0xd6845e), hex(0xb86253), hex(0xc27b5d)], ct_tube: [hex(0xca9718), hex(0x9b7811), hex(0xc19913)],
            gauge_out: hex(0x1d251a), gauge_empty: Color32::from_rgba_unmultiplied(0x1a, 0x1f, 0x17, 204),
            outline: hex(0x2a2119), shadow: hex(0x201711),
            pal_light, pal_page, pal_dark, pal_parch,
            good: hex(0x5e8a3a), bad: hex(0xa8321e), warn: hex(0xa86a1c), plague: hex(0x6e3f8a), ambient: hex(0x5e8a3a), hardcore: hex(0x8c2a1c),
            dark_bottom: c(0x10, 0x1c, 0x40),
        }
    }
    pub fn pal(&self, s: Surface) -> Pal { match s { Surface::Light => self.pal_light, Surface::Page => self.pal_page, Surface::Dark => self.pal_dark, Surface::Parchment => self.pal_parch } }
}

/// 컨텍스트에 스킨을 둔다 / 꺼낸다 (없으면 기본)
pub fn set(ctx: &egui::Context, s: Skin) { let a = std::sync::Arc::new(s); CUR.with(|c| *c.borrow_mut() = Some(a.clone())); ctx.data_mut(|d| d.insert_temp(egui::Id::unique("game-ui-skin"), a)); }
pub fn get(ctx: &egui::Context) -> std::sync::Arc<Skin> {
    if let Some(s) = ctx.data(|d| d.get_temp::<std::sync::Arc<Skin>>(egui::Id::unique("game-ui-skin"))) { return s; }
    let a = std::sync::Arc::new(Skin::default()); ctx.data_mut(|d| d.insert_temp(egui::Id::unique("game-ui-skin"), a.clone())); a
}

/// u — 원작 화소 하나. 2점을 물리 화소 정수배로 반올림 (100% 2px · 150% 3px · 200% 4px)
pub fn unit(ctx: &egui::Context) -> f32 { unit_for(ctx.pixels_per_point()) }
/// 화면 배율(ppp)에서의 u (점)
pub fn unit_for(ppp: f32) -> f32 { (2.0 * ppp).round().max(1.0) / ppp }
/// 물리 화소에 맞춘다
pub fn snap(ctx: &egui::Context, v: f32) -> f32 { let ppp = ctx.pixels_per_point(); (v * ppp).round() / ppp }
pub fn snap_rect(ctx: &egui::Context, r: egui::Rect) -> egui::Rect { egui::Rect::from_min_max(egui::pos2(snap(ctx, r.min.x), snap(ctx, r.min.y)), egui::pos2(snap(ctx, r.max.x), snap(ctx, r.max.y))) }

thread_local! {
    static SURF: RefCell<Vec<Surface>> = const { RefCell::new(Vec::new()) };
    static CUR: RefCell<Option<std::sync::Arc<Skin>>> = const { RefCell::new(None) };
}
/// 지금 표면의 글자색 (컨텍스트 없이 — 마지막으로 `set`한 스킨, 없으면 기본)
pub fn pl() -> Pal { let s = surface(); CUR.with(|c| c.borrow_mut().get_or_insert_with(|| std::sync::Arc::new(Skin::default())).pal(s)) }
/// 지금 그리는 창의 표면 (창 밖이면 HUD — 3D 위 글자)
pub fn surface() -> Surface { SURF.with(|s| s.borrow().last().copied().unwrap_or(Surface::Dark)) }
/// 표면 안에서 f를 돈다
pub fn within<R>(s: Surface, f: impl FnOnce() -> R) -> R { SURF.with(|v| v.borrow_mut().push(s)); let r = f(); SURF.with(|v| { v.borrow_mut().pop(); }); r }
/// 지금 표면의 글자색
pub fn pal(ctx: &egui::Context) -> Pal { get(ctx).pal(surface()) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_is_whole_physical_pixels() {
        for (ppp, px) in [(1.0f32, 2.0f32), (1.5, 3.0), (2.0, 4.0), (1.25, 3.0), (1.75, 4.0)] {
            let u = unit_for(ppp);
            assert!(((u * ppp) - px).abs() < 1e-4, "ppp {ppp}: u {u} → {} px (want {px})", u * ppp);
        }
    }
    #[test]
    fn measured_classic_plate_tokens() {
        // 리서치 §2.1 A [M] — 바뀌면 원작과 멀어진다
        let s = Skin::tactics();
        assert_eq!(s.plate_fill, Color32::from_rgb(0xa5, 0x9d, 0x83));
        assert_eq!(s.plate_out, Color32::from_rgb(0x2a, 0x21, 0x19));
        assert_eq!(s.plate_hi, Color32::from_rgb(0xd6, 0xcf, 0xad));
        assert_eq!(s.plate_shade_r, Color32::from_rgb(0x52, 0x53, 0x41));
        assert_eq!(s.plate_shade_b, Color32::from_rgb(0x74, 0x69, 0x53));
        assert_eq!(s.pal_light.text, Color32::from_rgb(0x42, 0x3b, 0x30));
        assert_eq!(s.pal_light.shadow, Color32::from_rgb(0x84, 0x7c, 0x6a));
    }
    #[test]
    fn surface_stack_nests() {
        assert_eq!(surface(), Surface::Dark);
        within(Surface::Light, || { assert_eq!(surface(), Surface::Light); within(Surface::Parchment, || assert_eq!(surface(), Surface::Parchment)); assert_eq!(surface(), Surface::Light); });
        assert_eq!(surface(), Surface::Dark);
    }
}
