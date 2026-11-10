//! game-ui — egui 위의 게임 UI 키트. 파이널 판타지 택틱스(1997 PS1 · 2025 이발리스 연대기)의 화면 언어를 잰 값 그대로 옮겼다
//! (리서치: deskarium `docs/research/fft-ui-research.md`).
//! - `skin` 색 · 치수 (u = 원작 화소 하나 = 2점) · `frame` 창 (클래식 판 · 슬레이트 · TIC 페이지 · HUD · 양피지) · 꼬리표 · 머리띠 · 금줄
//! - `menu` 손 커서 명령 메뉴 · `controls` 마름모 단추 · 칸 줄 · 탭 띠 · 새긴 칸 · ‹값› 조절 · 스탯 줄 · 단축키 판
//! - `gauge` 클래식 평행사변형 / TIC 관 · `message` 알림 · 대사 · `icons` 벡터 아이콘 60종 · `cursor` 손 · `text` 먹 그림자 · 윤곽 · 옛 숫자꼴
//! - `fonts` OS 한글 글꼴 + Georgia(옛 숫자꼴) · `theme` egui 기본 위젯 · `paint` 판 질감 · 그라데이션
//! `snapshot` 기능 = 창 없이 그려 PNG로 남긴다 (갤러리 · 시각 회귀).
pub mod controls;
pub mod cursor;
pub mod fonts;
pub mod frame;
pub mod gauge;
pub mod icons;
pub mod menu;
pub mod message;
pub mod paint;
pub mod skin;
pub mod text;
pub mod theme;
#[cfg(feature = "snapshot")]
pub mod snapshot;
