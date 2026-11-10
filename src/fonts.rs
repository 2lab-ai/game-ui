//! 글꼴 — OS 기본 한글 글꼴(라틴 포함)을 읽어 쓴다: macOS Apple SD Gothic Neo · Windows 맑은 고딕 · 리눅스 Noto Sans CJK KR(없으면 나눔고딕).
//! 바이너리에 글꼴을 싣지 않는다 (수 MB). 둘 다 해당 OS에 기본 설치되어 있다.
//! egui 기본 글꼴(라틴·흑백 이모지·아이콘)은 한글 뒤 대체 글꼴로 붙인다 — 도구 아이콘(⛰ 🌋 ☄ …)이 네모로 보이지 않게.
//! 제목·버튼용 굵은 글꼴 `bold`와 로고용 `heavy`도 같은 파일(.ttc의 다른 면, 또는 같은 계열 파일)에서 가져온다.
use std::sync::Arc;

pub const BOLD: &str = "bold";
pub const HEAVY: &str = "heavy";
/// 숫자 · 라틴 글자 — 원작처럼 세리프 + 옛 숫자꼴(Georgia: 기본이 옛 숫자꼴, macOS · Windows 기본 글꼴). 없으면 굵은 한글 글꼴
pub const NUM: &str = "num";

/// (보통, 굵게, 아주 굵게) — 같은 파일이면 면 번호만 다르다
fn candidates() -> Vec<(std::path::PathBuf, u32, Option<(std::path::PathBuf, u32)>, Option<(std::path::PathBuf, u32)>)> {
    let mut v = Vec::new();
    if cfg!(target_os = "macos") {
        let p: std::path::PathBuf = "/System/Library/Fonts/AppleSDGothicNeo.ttc".into();
        v.push((p.clone(), 0, Some((p.clone(), 6)), Some((p, 16))));
        v.push(("/System/Library/Fonts/Supplemental/AppleGothic.ttf".into(), 0, None, None));
    } else if cfg!(target_os = "windows") {
        let win = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".into());
        let bd: std::path::PathBuf = format!("{win}\\Fonts\\malgunbd.ttf").into();
        v.push((format!("{win}\\Fonts\\malgun.ttf").into(), 0, Some((bd.clone(), 0)), Some((bd, 0))));
        v.push((format!("{win}\\Fonts\\gulim.ttc").into(), 0, None, None));
    } else {
        // 리눅스: Noto Sans CJK(.ttc 1번 면 = KR) — Arch/CachyOS · Debian/Ubuntu · Fedora 경로. 없으면 나눔고딕
        for dir in ["/usr/share/fonts/noto-cjk", "/usr/share/fonts/opentype/noto", "/usr/share/fonts/google-noto-sans-cjk-fonts", "/usr/share/fonts/noto-cjk-fonts"] {
            let f = |w: &str| -> std::path::PathBuf { format!("{dir}/NotoSansCJK-{w}.ttc").into() };
            let (bd, bk) = (f("Bold"), f("Black"));
            v.push((f("Regular"), 1, bd.exists().then(|| (bd.clone(), 1)), Some(if bk.exists() { (bk, 1) } else { (bd, 1) })));
        }
        for dir in ["/usr/share/fonts/truetype/nanum", "/usr/share/fonts/nanum", "/usr/share/fonts/TTF"] {
            let f = |w: &str| -> std::path::PathBuf { format!("{dir}/NanumGothic{w}.ttf").into() };
            v.push((f(""), 0, Some((f("Bold"), 0)), Some((f("ExtraBold"), 0))));
        }
    }
    v
}

/// 한글 글꼴을 찾으면 그 경로를 돌려준다 (없으면 None — 한글이 네모로 보인다).
pub fn install(ctx: &egui::Context) -> Option<std::path::PathBuf> {
    let mut defs = egui::FontDefinitions::default();
    let mut found = None;
    for (p, idx, bold, heavy) in candidates() {
        let Ok(bytes) = std::fs::read(&p) else { continue };
        let mut fd = egui::FontData::from_owned(bytes); fd.index = idx;
        defs.font_data.insert("hangul".into(), Arc::new(fd.clone()));
        for fam in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] { defs.families.entry(fam).or_default().insert(0, "hangul".into()); }   // 한글 먼저, egui 기본(라틴·이모지)은 대체
        for (name, spec) in [(BOLD, bold), (HEAVY, heavy)] {
            let data = match spec { Some((bp, bi)) if bp == p => { let mut d = fd.clone(); d.index = bi; Some(d) } Some((bp, bi)) => std::fs::read(&bp).ok().map(|b| { let mut d = egui::FontData::from_owned(b); d.index = bi; d }), None => None };
            let key = format!("hangul-{name}");
            defs.font_data.insert(key.clone(), Arc::new(data.unwrap_or_else(|| fd.clone())));
            let fam = defs.families.entry(egui::FontFamily::Name(name.into())).or_default(); fam.push(key); fam.push("hangul".into());
            let rest: Vec<String> = defs.families.get(&egui::FontFamily::Proportional).cloned().unwrap_or_default(); for r in rest { let f = defs.families.get_mut(&egui::FontFamily::Name(name.into())).unwrap(); if !f.contains(&r) { f.push(r); } }
        }
        found = Some(p); break;
    }
    // 숫자 글꼴 (Georgia Bold) → 굵은 한글 → 나머지
    let georgia: Vec<std::path::PathBuf> = if cfg!(target_os = "macos") { vec!["/System/Library/Fonts/Supplemental/Georgia Bold.ttf".into(), "/Library/Fonts/Georgia Bold.ttf".into()] }
        else if cfg!(target_os = "windows") { let win = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".into()); vec![format!("{win}\\Fonts\\georgiab.ttf").into()] }
        // 리눅스엔 Georgia가 없다 — 굵은 세리프(Noto Serif · DejaVu Serif · Liberation Serif)로 대신한다 (옛 숫자꼴은 아니다)
        else { ["/usr/share/fonts/noto/NotoSerif-Bold.ttf", "/usr/share/fonts/truetype/noto/NotoSerif-Bold.ttf", "/usr/share/fonts/google-noto/NotoSerif-Bold.ttf",
                "/usr/share/fonts/TTF/DejaVuSerif-Bold.ttf", "/usr/share/fonts/truetype/dejavu/DejaVuSerif-Bold.ttf", "/usr/share/fonts/dejavu-serif-fonts/DejaVuSerif-Bold.ttf",
                "/usr/share/fonts/liberation/LiberationSerif-Bold.ttf", "/usr/share/fonts/truetype/liberation/LiberationSerif-Bold.ttf"].iter().map(|p| p.into()).collect() };
    let mut num_fam: Vec<String> = Vec::new();
    if let Some(b) = georgia.iter().find_map(|p| std::fs::read(p).ok()) { defs.font_data.insert("num-serif".into(), Arc::new(egui::FontData::from_owned(b))); num_fam.push("num-serif".into()); }
    for f in defs.families.get(&egui::FontFamily::Name(BOLD.into())).cloned().unwrap_or_default() { if !num_fam.contains(&f) { num_fam.push(f); } }
    defs.families.insert(egui::FontFamily::Name(NUM.into()), num_fam);
    if found.is_none() {
        log::warn!("한글 글꼴을 찾지 못했다 — 한글이 네모로 보일 수 있다");
        for name in [BOLD, HEAVY, NUM] { let fam: Vec<String> = defs.families.get(&egui::FontFamily::Proportional).cloned().unwrap_or_default(); defs.families.insert(egui::FontFamily::Name(name.into()), fam); }
    }
    ctx.set_fonts(defs);
    found
}

pub fn bold(size: f32) -> egui::FontId { egui::FontId::new(size, egui::FontFamily::Name(BOLD.into())) }
pub fn heavy(size: f32) -> egui::FontId { egui::FontId::new(size, egui::FontFamily::Name(HEAVY.into())) }
/// 숫자 · 라틴 세리프 (옛 숫자꼴). 한글은 굵은 한글 글꼴로 이어진다
pub fn num(size: f32) -> egui::FontId { egui::FontId::new(size, egui::FontFamily::Name(NUM.into())) }
