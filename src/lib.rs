//! game-ui — egui 위의 게임 UI 키트.
//! 글꼴(OS 한글 글꼴 · 굵게 · 아주 굵게) · 테마 · 위젯. `snapshot` 기능 = 창 없이 그려 PNG로 남긴다.
pub mod fonts;
pub mod theme;
pub mod widgets;
#[cfg(feature = "snapshot")]
pub mod snapshot;
