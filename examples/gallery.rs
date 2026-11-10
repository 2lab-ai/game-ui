//! 갤러리 — 모든 컴포넌트를 한 장에 그려 PNG로 (`cargo run -p game-ui --example gallery --features snapshot -- out.png [배경.png] [ppp]`)
use egui::{pos2, vec2, Align2, Color32, Rect, UiBuilder};
use game_ui::skin::Surface;
use game_ui::{controls, fonts, frame::{self, Kind}, gauge::{self, Tone}, icons::{self, Icon}, menu::{Item, Menu}, message, paint, skin, snapshot::{save_png, Snapshot}, text};

fn at<R>(ui: &mut egui::Ui, x: f32, y: f32, w: f32, h: f32, f: impl FnOnce(&mut egui::Ui) -> R) -> R { ui.scope_builder(UiBuilder::new().max_rect(Rect::from_min_size(pos2(x, y), vec2(w, h))), |ui| { ui.spacing_mut().item_spacing = vec2(0.0, 0.0); f(ui) }).inner }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = args.get(1).cloned().unwrap_or_else(|| "gallery.png".into());
    let bg = args.get(2).filter(|s| !s.is_empty()).and_then(|p| image_rgba(p));
    let ppp: f32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1.0);
    let mut s = Snapshot::new().expect("GPU");
    let (mut tab, mut speed, mut chk, mut sl) = (2usize, 1u8, true, 1.0f32);
    let mut bg_tex: Option<egui::TextureHandle> = None;
    let (w, h, px) = s.render([1280.0, 800.0], ppp, [0.0, 0.0, 0.0, 1.0], 3, &mut |ui| {
        let ctx = ui.ctx().clone(); let sk = skin::get(&ctx); let p = ui.painter().clone(); let t = ctx.input(|i| i.time); let u = skin::unit(&ctx);
        let full = Rect::from_min_size(pos2(0.0, 0.0), vec2(1280.0, 800.0));
        if let Some((bw, bh, ref data)) = bg { let tx = bg_tex.get_or_insert_with(|| ctx.load_texture("bg", egui::ColorImage::from_rgba_unmultiplied([bw, bh], data), egui::TextureOptions::LINEAR)); p.image(tx.id(), full, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE); }
        else { paint::vgrad_stops(&p, full, &[(0.0, Color32::from_rgb(92, 132, 186)), (0.45, Color32::from_rgb(150, 182, 206)), (0.46, Color32::from_rgb(110, 150, 76)), (1.0, Color32::from_rgb(62, 96, 50))]); }

        // 3D 위 글자 (판 없이) — 날짜 · 속도
        text::hud(&p, pos2(24.0, 26.0), Align2::LEFT_CENTER, "1년 10일 13:00", fonts::num(24.0));
        text::hud(&p, pos2(26.0, 52.0), Align2::LEFT_CENTER, "하드코어 · 흥망성쇠", fonts::bold(13.0));
        at(ui, 24.0, 66.0, 260.0, 40.0, |ui| skin::within(Surface::Dark, || controls::choice(ui, &mut speed, &[(0u8, "Ⅱ"), (1, "1×"), (4, "4×"), (16, "16×"), (64, "64×")], 40.0)));

        // 부모 메뉴(슬레이트 + 흐린 손) → 자식 명령 메뉴(클래식 + 손)
        Menu::select(&ctx, "parent", 3);
        let parent = [Item::new("땅"), Item::new("자연"), Item::new("생명"), Item::new("재앙"), Item::new("손")];
        at(ui, 40.0, 140.0, 200.0, 240.0, |ui| { frame::show_titled(ui, Kind::Slate, "신의 손", |ui| { Menu::new("parent", &parent).width(84.0).font(16.0).keyboard(false).ghost(true).show(ui); }); });
        let items = [Item::new("운석").icon(Icon::Meteor).right("3"), Item::new("번개").icon(Icon::Lightning), Item::new("역병").icon(Icon::Skull), Item::new("지진").icon(Icon::Quake), Item::new("핵").icon(Icon::Nuke).enabled(false)];
        at(ui, 150.0, 200.0, 220.0, 260.0, |ui| { frame::show_titled(ui, Kind::Menu, "재앙", |ui| { Menu::new("cmd", &items).width(128.0).show(ui); }); });

        // 클래식 상태판 (원작 이름판 + 게이지)
        at(ui, 40.0, 470.0, 330.0, 200.0, |ui| { frame::show(ui, Kind::Menu, |ui| {
            ui.set_width(290.0); let pl = skin::pal(ui.ctx());
            ui.horizontal(|ui| { let (mr, _) = ui.allocate_exact_size(vec2(44.0, 44.0), egui::Sense::hover()); message::medallion(ui.painter(), mr.center(), 19.0, Icon::House, sk.ambient);
                ui.add_space(8.0);
                ui.vertical(|ui| { let (r, _) = ui.allocate_exact_size(vec2(230.0, 44.0), egui::Sense::hover()); text::shadowed(ui.painter(), r.left_top() + vec2(0.0, 12.0), Align2::LEFT_CENTER, "노을내", fonts::heavy(20.0), pl.title, pl.shadow); text::shadowed(ui.painter(), r.left_top() + vec2(0.0, 34.0), Align2::LEFT_CENTER, "인간 · 독립 마을", fonts::bold(13.0), pl.dim, Color32::TRANSPARENT); text::shadowed(ui.painter(), r.right_top() + vec2(0.0, 12.0), Align2::RIGHT_CENTER, "Lv.09", fonts::num(16.0), pl.text, pl.shadow); }); });
            ui.add_space(4.0);
            gauge::gauge(ui, "인구", 151.0, 151.0, Tone::Hp, 290.0); gauge::gauge(ui, "식량", 24.0, 60.0, Tone::Mp, 290.0); gauge::percent(ui, "연구", 0.42, Tone::Ct, 290.0, Some("37/77".into()));
        }); });

        // TIC HUD 판 (남색, 오른쪽으로 사라짐) + 관 게이지
        at(ui, 40.0, 680.0, 420.0, 110.0, |ui| { frame::show(ui, Kind::Hud, |ui| {
            ui.set_width(300.0); let pl = skin::pal(ui.ctx());
            let (r, _) = ui.allocate_exact_size(vec2(300.0, 24.0), egui::Sense::hover()); text::shadowed(ui.painter(), r.left_center(), Align2::LEFT_CENTER, "노을내", fonts::heavy(19.0), pl.title, pl.shadow); text::shadowed(ui.painter(), r.right_center(), Align2::RIGHT_CENTER, "12 / 40", fonts::num(16.0), pl.text, pl.shadow);
            gauge::percent(ui, "행복", 0.42, Tone::Hp, 300.0, None); gauge::percent(ui, "충성", 0.8, Tone::Ct, 300.0, None);
        }); });

        // TIC 탭 + 페이지 목록 (읽는 화면)
        at(ui, 420.0, 130.0, 400.0, 360.0, |ui| {
            controls::tabs(ui, &mut tab, &["내 마을", "세계", "연대기", "역사"], 380.0);
            ui.add_space(2.0 * u);
            frame::show_titled(ui, Kind::Page, "연대기 · 1년", |ui| { ui.set_width(380.0 - 16.0 * u);
                let rows = [Item::new("「바람 들개 대군」이 노을내를 덮쳤다").icon(Icon::Horse).sub("1년 10일 13:00"), Item::new("검은 역병이 계명언덕에서 번진다").icon(Icon::Skull).sub("1년 9일 06:00"), Item::new("하나그라 왕국이 대륙을 묶었다").icon(Icon::Crown).sub("1년 4일 18:00")];
                Menu::new("chron", &rows).width(380.0 - 16.0 * u).font(14.0).keyboard(false).hand_gap(2.0 * u).show(ui);
            });
        });

        // 설정 페이지 — 칸 줄 · 확인 · ‹값› · 마름모 단추
        at(ui, 860.0, 40.0, 400.0, 360.0, |ui| { frame::show_titled(ui, Kind::Page, "설정", |ui| { ui.set_width(370.0); ui.spacing_mut().item_spacing.y = 3.0 * u;
            controls::heading(ui, "속도");
            controls::choice(ui, &mut speed, &[(0u8, "멈춤"), (1, "1×"), (4, "4×"), (16, "16×"), (64, "64×")], 70.0);
            controls::heading(ui, "화면");
            controls::check(ui, &mut chk, "큰 사건 알림");
            controls::slider(ui, &mut sl, 0.75..=1.6, 0.05, 370.0, &|v| format!("{:.0}%", v * 100.0));
            ui.add_space(2.0 * u);
            ui.horizontal(|ui| { ui.spacing_mut().item_spacing.x = 6.0 * u; ui.add_space(6.0 * u); controls::button(ui, "뒤로", None, false, 140.0); controls::button(ui, "세계 만들기", Some(Icon::Play), true, 190.0); });
        }); });

        // 알림 (클래식 판 + 꼬리표 + 짧은 명령)
        at(ui, 420.0, 535.0, 620.0, 160.0, |ui| { message::notice(ui, &message::Notice { tag: "재앙", icon: Icon::Horse, tone: sk.hardcore, title: "「바람 들개 대군」 60기", body: "남쪽 초원에서 기마병 60기가 나타났다. 지나는 곳마다 아무것도 남기지 않는다.", hint: Some("운석 · 번개로 대군을 깰 수 있다 — 내 마을에 닿기 전에"), action: Some((Icon::Meteor, "운석 들기")), big: true }, 600.0); });

        // 작은 알림 (한 줄 판)
        at(ui, 420.0, 470.0, 620.0, 40.0, |ui| { message::notice(ui, &message::Notice { tag: "자연", icon: Icon::Wheat, tone: sk.good, title: "풍년", body: "시에니아에 대풍작이 들었다.", hint: None, action: None, big: false }, 600.0); });
        // 대사 창
        message::dialogue(&p, Rect::from_min_size(pos2(500.0, 690.0), vec2(560.0, 90.0)), Some("초원의 전령"), "남쪽 초원에서 기마병 60기가 나타났다.\n지나는 곳마다 아무것도 남기지 않는다.", t);

        // 양피지 (연대기 쪽)
        at(ui, 860.0, 325.0, 400.0, 260.0, |ui| { frame::show_titled(ui, Kind::Parchment, "흥망의 기록", |ui| { ui.set_width(330.0); let pl = skin::pal(ui.ctx());
            for (ic, d, l) in [(Icon::Horse, "1년 10일", "「바람 들개 대군」이 노을내를 덮쳤다"), (Icon::Skull, "1년 9일", "검은 역병이 계명언덕에서 번졌다"), (Icon::Crown, "1년 4일", "하나그라 왕국이 대륙을 하나로 묶었다")] {
                ui.horizontal(|ui| { let (r, _) = ui.allocate_exact_size(vec2(22.0, 22.0), egui::Sense::hover()); icons::paint(ui.painter(), r, ic, None); ui.add_space(6.0); ui.label(egui::RichText::new(d).font(fonts::num(12.5)).color(pl.dim)); ui.add_space(6.0); ui.label(egui::RichText::new(l).font(fonts::bold(13.5)).color(pl.text)); });
                ui.add_space(4.0);
            }
        }); });

        // 단축키 판 (오른쪽 아래)
        let mut x = 1080.0; for (k, l) in [("Esc", "메뉴"), ("Space", "멈춤")] { let r = controls::key_hint(&p, pos2(x, 780.0), k, l); x = r.max.x + 14.0; }
        // 아이콘 줄
        for (k, ic) in icons::ALL.iter().enumerate().take(14) { icons::paint(&p, Rect::from_min_size(pos2(420.0 + k as f32 * 30.0, 92.0), vec2(24.0, 24.0)), *ic, None); }
    }).expect("render");
    save_png(std::path::Path::new(&out), w, h, &px).expect("png");
    println!("gallery {out} {w}x{h} adapter={}", s.adapter);
}

fn image_rgba(path: &str) -> Option<(usize, usize, Vec<u8>)> {
    let f = std::fs::File::open(path).ok()?; let d = png::Decoder::new(std::io::BufReader::new(f)); let mut r = d.read_info().ok()?;
    let mut buf = vec![0; r.output_buffer_size()?]; let info = r.next_frame(&mut buf).ok()?;
    let (w, h) = (info.width as usize, info.height as usize);
    let data = match info.color_type { png::ColorType::Rgba => buf[..w * h * 4].to_vec(), png::ColorType::Rgb => buf[..w * h * 3].chunks(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(), _ => return None };
    Some((w, h, data))
}
