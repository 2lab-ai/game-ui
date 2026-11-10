//! 벡터 아이콘 — 이모지 대신 직접 그린다. 16×16 격자에 볼록 다각형 · 선 · 원으로 짓고, 한 픽셀 아래 오른쪽에 짙은 그림자를 깔아
//! 어떤 배경 위에서도 읽힌다 (FFT 아이템 아이콘처럼 실루엣 + 어두운 윤곽).
use egui::{epaint::PathStroke, Color32, Painter, Pos2, Rect, Shape, Stroke};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    // 상태 · 위협
    Sword, Horse, Skull, Crown, Flame, Leaf, Shield, Flag, Star, Heart, Eye, Hourglass, Scroll, Map,
    // 자원 · 마을
    House, People, Wheat, Bread, Coin, Flask, Gear, Pickaxe, Anvil, Temple, Smile,
    // 날씨 · 하늘
    Sun, Moon, Rain, Drop, Lava, Lightning, Meteor, Snow,
    // 신의 도구
    Raise, Lower, Block, Shovel, Person, Sheep, Fish, Sparkle, Quake, Bomb, Nuke, Magnifier, Ghost,
    // 조작
    Pause, Play, Fast, Menu, Close, Check, Cog, Save, Dice, ArrowRight, ArrowLeft, ArrowUp, ArrowDown, Hammer,
}

pub const ALL: [Icon; 60] = [
    Icon::Sword, Icon::Horse, Icon::Skull, Icon::Crown, Icon::Flame, Icon::Leaf, Icon::Shield, Icon::Flag, Icon::Star, Icon::Heart, Icon::Eye, Icon::Hourglass, Icon::Scroll, Icon::Map,
    Icon::House, Icon::People, Icon::Wheat, Icon::Bread, Icon::Coin, Icon::Flask, Icon::Gear, Icon::Pickaxe, Icon::Anvil, Icon::Temple, Icon::Smile,
    Icon::Sun, Icon::Moon, Icon::Rain, Icon::Drop, Icon::Lava, Icon::Lightning, Icon::Meteor, Icon::Snow,
    Icon::Raise, Icon::Lower, Icon::Block, Icon::Shovel, Icon::Person, Icon::Sheep, Icon::Fish, Icon::Sparkle, Icon::Quake, Icon::Bomb, Icon::Nuke, Icon::Magnifier, Icon::Ghost,
    Icon::Pause, Icon::Play, Icon::Fast, Icon::Menu, Icon::Close, Icon::Check, Icon::Cog, Icon::Save, Icon::Dice, Icon::ArrowRight, Icon::ArrowLeft, Icon::ArrowUp, Icon::ArrowDown, Icon::Hammer,
];

/// 아이콘 고유색 (tint를 주지 않을 때)
pub fn natural(i: Icon) -> Color32 {
    use Icon::*;
    let c = |r, g, b| Color32::from_rgb(r, g, b);
    match i {
        Sword | Shield | Pickaxe | Anvil | Hammer | Gear | Cog | Magnifier => c(206, 212, 220),
        Horse => c(176, 120, 72), Skull => c(232, 226, 206), Crown | Coin | Star | Sun => c(246, 200, 74),
        Flame | Lava | Bomb => c(244, 120, 52), Leaf | Wheat => c(132, 196, 92), Flag => c(214, 70, 58), Heart => c(230, 84, 96),
        Eye | Ghost | Moon | Snow => c(214, 226, 240), Hourglass | Scroll | Map | Bread => c(226, 196, 140),
        House | Temple | Block => c(212, 170, 112), People | Person | Smile => c(240, 214, 176), Flask => c(132, 200, 236),
        Rain | Drop | Fish => c(104, 170, 240), Lightning => c(250, 230, 96), Meteor | Quake | Lower | Shovel | Raise => c(196, 160, 112),
        Sheep => c(240, 236, 226), Sparkle => c(150, 240, 170), Nuke => c(220, 240, 90), Save | Dice => c(226, 214, 190),
        _ => c(232, 228, 216),
    }
}

/// 그린다. `tint`가 있으면 그 색 하나로 (도구 막대에서 선택된 줄 등)
pub fn paint(p: &Painter, rect: Rect, icon: Icon, tint: Option<Color32>) {
    let col = tint.unwrap_or_else(|| natural(icon));
    let shadow = Color32::from_black_alpha(170);
    let s = rect.width().min(rect.height()) / 16.0;
    let o = rect.center() - egui::vec2(8.0 * s, 8.0 * s);
    let off = egui::vec2(s.max(1.0), s.max(1.0)) * 0.9;
    // 그림자(오른쪽 아래) → 어두운 윤곽(네 방향으로 조금씩) → 색 — 어떤 배경(밤하늘 · 양피지) 위에서도 실루엣이 읽힌다
    let ol = (0.55 * s).max(0.7); let line = Color32::from_rgba_unmultiplied(20, 14, 8, 230);
    let passes: [(egui::Vec2, Option<Color32>); 6] = [(off + egui::vec2(ol, ol), Some(shadow)), (egui::vec2(-ol, 0.0), Some(line)), (egui::vec2(ol, 0.0), Some(line)), (egui::vec2(0.0, -ol), Some(line)), (egui::vec2(0.0, ol), Some(line)), (egui::Vec2::ZERO, None)];
    for (d, mono) in passes {
        let at = |x: f32, y: f32| -> Pos2 { o + egui::vec2(x * s, y * s) + d };
        let mut g = Pen { p, at: &at, s, c: mono.unwrap_or(col), dark: mono.unwrap_or_else(|| darker(col)), mono };
        draw(&mut g, icon);
    }
}

fn darker(c: Color32) -> Color32 { Color32::from_rgb((c.r() as f32 * 0.45) as u8, (c.g() as f32 * 0.45) as u8, (c.b() as f32 * 0.45) as u8) }

struct Pen<'a> { p: &'a Painter, at: &'a dyn Fn(f32, f32) -> Pos2, s: f32, c: Color32, dark: Color32, mono: Option<Color32> }
impl Pen<'_> {
    fn m(&self, c: Color32) -> Color32 { self.mono.unwrap_or(c) }
    fn poly(&mut self, pts: &[(f32, f32)]) { self.p.add(Shape::convex_polygon(pts.iter().map(|&(x, y)| (self.at)(x, y)).collect(), self.c, Stroke::NONE)); }
    fn poly_c(&mut self, pts: &[(f32, f32)], c: Color32) { let c = self.m(c); self.p.add(Shape::convex_polygon(pts.iter().map(|&(x, y)| (self.at)(x, y)).collect(), c, Stroke::NONE)); }
    fn rect(&mut self, x: f32, y: f32, w: f32, h: f32) { self.poly(&[(x, y), (x + w, y), (x + w, y + h), (x, y + h)]); }
    fn rect_c(&mut self, x: f32, y: f32, w: f32, h: f32, c: Color32) { self.poly_c(&[(x, y), (x + w, y), (x + w, y + h), (x, y + h)], c); }
    fn circle(&mut self, x: f32, y: f32, r: f32) { self.p.circle_filled((self.at)(x, y), r * self.s, self.c); }
    fn circle_c(&mut self, x: f32, y: f32, r: f32, c: Color32) { let c = self.m(c); self.p.circle_filled((self.at)(x, y), r * self.s, c); }
    fn ring(&mut self, x: f32, y: f32, r: f32, w: f32) { self.p.circle_stroke((self.at)(x, y), r * self.s, Stroke::new(w * self.s, self.c)); }
    fn line(&mut self, pts: &[(f32, f32)], w: f32) { self.p.add(Shape::line(pts.iter().map(|&(x, y)| (self.at)(x, y)).collect(), PathStroke::new(w * self.s, self.c))); }
    fn line_c(&mut self, pts: &[(f32, f32)], w: f32, c: Color32) { let c = self.m(c); self.p.add(Shape::line(pts.iter().map(|&(x, y)| (self.at)(x, y)).collect(), PathStroke::new(w * self.s, c))); }
    fn arc(&mut self, x: f32, y: f32, r: f32, a0: f32, a1: f32, w: f32) { let n = 14; let pts: Vec<(f32, f32)> = (0..=n).map(|i| { let a = a0 + (a1 - a0) * i as f32 / n as f32; (x + r * a.cos(), y + r * a.sin()) }).collect(); self.line(&pts, w); }
    fn star(&mut self, x: f32, y: f32, r: f32, n: usize) { let r2 = r * 0.45; for i in 0..n { let a = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::TAU / n as f32; let b = a + std::f32::consts::PI / n as f32; let c = a - std::f32::consts::PI / n as f32; self.poly(&[(x, y), (x + r2 * c.cos(), y + r2 * c.sin()), (x + r * a.cos(), y + r * a.sin()), (x + r2 * b.cos(), y + r2 * b.sin())]); } }
}

fn draw(g: &mut Pen, icon: Icon) {
    use Icon::*;
    let dk = g.dark;
    match icon {
        Sword => { g.poly(&[(12.5, 1.5), (14.5, 1.5), (14.5, 3.5), (6.0, 12.0), (4.0, 10.0)]); g.line(&[(3.0, 8.5), (7.5, 13.0)], 1.6); g.line(&[(4.8, 11.2), (2.0, 14.0)], 1.8); g.circle(1.8, 14.2, 1.0); g.line_c(&[(13.2, 2.8), (6.0, 10.0)], 0.6, dk); }
        Horse => { g.poly(&[(3.0, 7.0), (10.5, 6.0), (11.5, 10.0), (3.5, 10.5)]); g.poly(&[(9.5, 6.5), (11.5, 2.5), (14.5, 4.0), (14.0, 6.0), (11.5, 8.5)]); g.poly(&[(11.0, 3.0), (11.5, 1.2), (12.4, 2.8)]); for x in [3.6, 5.6, 8.6, 10.3] { g.rect(x, 9.5, 1.3, 5.0); } g.line(&[(3.2, 7.5), (1.0, 11.0)], 1.3); g.line_c(&[(11.0, 3.0), (9.6, 6.6)], 1.0, dk); }
        Skull => { g.circle(8.0, 6.8, 5.6); g.rect(4.8, 9.0, 6.4, 4.2); g.circle_c(5.8, 7.2, 1.6, dk); g.circle_c(10.2, 7.2, 1.6, dk); g.poly_c(&[(8.0, 9.0), (8.9, 10.6), (7.1, 10.6)], dk); for x in [6.2, 8.0, 9.8] { g.line_c(&[(x, 11.6), (x, 13.2)], 0.7, dk); } }
        Crown => { g.poly(&[(2.0, 5.0), (5.0, 9.0), (2.5, 12.0)]); g.poly(&[(14.0, 5.0), (13.5, 12.0), (11.0, 9.0)]); g.poly(&[(8.0, 3.0), (10.5, 9.0), (5.5, 9.0)]); g.rect(2.5, 9.0, 11.0, 3.0); g.rect(2.5, 12.6, 11.0, 1.6); for (x, y) in [(2.0, 4.6), (8.0, 2.6), (14.0, 4.6)] { g.circle(x, y, 1.1); } g.circle_c(8.0, 10.5, 0.9, dk); }
        Flame => { g.poly(&[(8.0, 1.0), (12.5, 7.0), (12.8, 11.0), (8.0, 15.0), (3.2, 11.0), (3.8, 6.5)]); g.poly(&[(11.5, 4.0), (13.0, 6.5), (12.0, 8.0)]); g.poly_c(&[(8.0, 6.5), (10.4, 10.5), (8.0, 14.0), (5.6, 10.5)], Color32::from_rgb(255, 214, 96)); }
        Leaf => { g.poly(&[(2.0, 14.0), (3.0, 7.0), (7.0, 3.0), (14.0, 2.0), (13.0, 9.0), (9.0, 13.0)]); g.line_c(&[(2.5, 13.5), (11.5, 4.5)], 0.9, dk); g.line_c(&[(6.0, 10.0), (5.6, 6.6)], 0.6, dk); g.line_c(&[(8.4, 7.6), (11.0, 8.0)], 0.6, dk); }
        Shield => { g.poly(&[(2.5, 2.5), (13.5, 2.5), (13.5, 8.0), (8.0, 14.8), (2.5, 8.0)]); g.poly_c(&[(8.0, 3.6), (12.4, 3.6), (12.4, 7.8), (8.0, 13.2)], dk); }
        Flag => { g.rect(2.5, 1.5, 1.4, 13.5); g.poly(&[(3.9, 2.0), (14.0, 2.0), (11.0, 5.0), (14.0, 8.0), (3.9, 8.0)]); }
        Star => g.star(8.0, 8.4, 7.2, 5),
        Heart => { g.circle(5.3, 6.0, 3.3); g.circle(10.7, 6.0, 3.3); g.poly(&[(2.1, 7.0), (13.9, 7.0), (8.0, 14.0)]); }
        Eye => { g.poly(&[(1.0, 8.0), (5.0, 4.0), (11.0, 4.0), (15.0, 8.0), (11.0, 12.0), (5.0, 12.0)]); g.circle_c(8.0, 8.0, 3.0, dk); g.circle(8.6, 7.4, 1.0); }
        Hourglass => { g.rect(3.0, 1.5, 10.0, 1.6); g.rect(3.0, 12.9, 10.0, 1.6); g.poly(&[(4.0, 3.0), (12.0, 3.0), (8.6, 8.0), (7.4, 8.0)]); g.poly(&[(7.4, 8.0), (8.6, 8.0), (12.0, 13.0), (4.0, 13.0)]); g.poly_c(&[(6.4, 10.6), (9.6, 10.6), (11.0, 12.6), (5.0, 12.6)], dk); }
        Scroll => { g.rect(3.5, 3.0, 9.0, 10.0); g.circle(3.5, 4.0, 1.8); g.circle(12.5, 12.0, 1.8); for y in [5.6, 7.6, 9.6] { g.line_c(&[(5.5, y), (11.0, y)], 0.8, dk); } }
        Map => { g.poly(&[(1.5, 3.5), (5.5, 2.0), (5.5, 13.0), (1.5, 14.5)]); g.poly(&[(5.5, 2.0), (10.5, 3.5), (10.5, 14.5), (5.5, 13.0)]); g.poly(&[(10.5, 3.5), (14.5, 2.0), (14.5, 13.0), (10.5, 14.5)]); g.line_c(&[(3.0, 10.0), (7.0, 6.5), (9.0, 9.5), (13.0, 5.5)], 0.8, dk); }
        House => { g.poly(&[(8.0, 1.5), (15.0, 8.0), (1.0, 8.0)]); g.rect(2.8, 7.5, 10.4, 7.0); g.rect_c(6.8, 10.0, 2.6, 4.5, dk); g.rect(11.0, 2.5, 1.6, 3.5); }
        People => { g.circle(5.0, 4.5, 2.4); g.poly(&[(1.5, 14.0), (2.0, 8.5), (8.0, 8.5), (8.5, 14.0)]); g.circle(11.0, 5.5, 2.2); g.poly(&[(8.0, 14.0), (8.6, 9.5), (14.0, 9.5), (14.5, 14.0)]); }
        Wheat => { g.line(&[(8.0, 15.0), (8.0, 4.0)], 1.0); for (dy, sx) in [(4.0, 1.0), (6.5, 1.0), (9.0, 1.0), (4.0, -1.0), (6.5, -1.0), (9.0, -1.0)] { let x = 8.0 + sx * 2.0; g.poly(&[(8.0, dy + 2.2), (x, dy - 0.8), (x + sx * 1.2, dy + 0.6)]); } g.poly(&[(7.2, 3.5), (8.0, 1.0), (8.8, 3.5)]); }
        Bread => { g.poly(&[(2.0, 9.0), (3.5, 5.0), (8.0, 3.8), (12.5, 5.0), (14.0, 9.0), (13.0, 12.5), (3.0, 12.5)]); for x in [5.5, 8.0, 10.5] { g.line_c(&[(x - 0.8, 6.0), (x + 0.8, 8.2)], 0.8, dk); } }
        Coin => { g.circle(8.0, 8.0, 6.5); g.ring(8.0, 8.0, 4.6, 0.8); g.line_c(&[(8.0, 5.0), (8.0, 11.0)], 1.0, dk); }
        Flask => { g.rect(6.4, 1.5, 3.2, 5.0); g.poly(&[(6.4, 6.0), (9.6, 6.0), (14.0, 13.0), (13.0, 14.6), (3.0, 14.6), (2.0, 13.0)]); g.poly_c(&[(4.6, 10.0), (11.4, 10.0), (13.4, 13.4), (2.6, 13.4)], Color32::from_rgb(150, 230, 170)); g.rect(5.6, 1.0, 4.8, 1.2); }
        Gear | Cog => { g.circle(8.0, 8.0, 5.0); for i in 0..8 { let a = i as f32 * std::f32::consts::TAU / 8.0; let (c, s) = (a.cos(), a.sin()); let (px, py) = (-s, c); g.poly(&[(8.0 + c * 4.0 + px * 1.3, 8.0 + s * 4.0 + py * 1.3), (8.0 + c * 7.2 + px * 1.0, 8.0 + s * 7.2 + py * 1.0), (8.0 + c * 7.2 - px * 1.0, 8.0 + s * 7.2 - py * 1.0), (8.0 + c * 4.0 - px * 1.3, 8.0 + s * 4.0 - py * 1.3)]); } g.circle_c(8.0, 8.0, 2.0, dk); }
        Pickaxe => { let wood = Color32::from_rgb(150, 104, 62); g.line_c(&[(3.0, 14.5), (10.5, 5.5)], 1.8, wood); g.arc(9.2, 9.6, 7.2, -2.75, -0.35, 2.4); g.poly(&[(2.0, 6.5), (3.8, 4.4), (2.6, 8.8)]); g.poly(&[(15.0, 6.8), (13.6, 4.4), (15.4, 9.2)]); }
        Anvil => { g.poly(&[(1.5, 4.0), (12.0, 4.0), (14.5, 5.5), (12.0, 7.5), (4.5, 7.5)]); g.rect(5.5, 7.5, 5.0, 3.5); g.poly(&[(3.5, 14.0), (5.0, 11.0), (11.0, 11.0), (12.5, 14.0)]); }
        Temple => { g.poly(&[(8.0, 1.5), (15.0, 5.0), (1.0, 5.0)]); g.rect(1.5, 5.0, 13.0, 1.4); for x in [2.6, 6.0, 9.0, 12.4] { g.rect(x, 6.4, 1.6, 6.6); } g.rect(1.0, 13.0, 14.0, 1.8); }
        Smile => { g.circle(8.0, 8.0, 6.6); g.circle_c(5.8, 6.4, 0.9, dk); g.circle_c(10.2, 6.4, 0.9, dk); g.arc(8.0, 8.4, 3.4, 0.4, 2.74, 1.0); g.c = dk; g.arc(8.0, 8.4, 3.4, 0.4, 2.74, 1.0); }
        Sun => { g.circle(8.0, 8.0, 3.6); for i in 0..8 { let a = i as f32 * std::f32::consts::TAU / 8.0; g.line(&[(8.0 + 5.0 * a.cos(), 8.0 + 5.0 * a.sin()), (8.0 + 7.2 * a.cos(), 8.0 + 7.2 * a.sin())], 1.4); } }
        Moon => { g.arc(8.6, 8.0, 5.0, 1.75, 4.55, 3.4); g.arc(9.4, 8.0, 5.6, 1.95, 4.35, 1.6); }
        Rain => { g.circle(5.5, 6.0, 3.0); g.circle(9.5, 5.0, 3.6); g.circle(12.5, 7.0, 2.4); g.rect(3.0, 6.0, 11.5, 3.0); for x in [4.5, 8.0, 11.5] { g.line(&[(x, 11.0), (x - 1.0, 14.5)], 1.2); } }
        Drop | Lava => { g.poly(&[(8.0, 1.5), (12.4, 8.5), (12.4, 11.0), (10.5, 14.0), (5.5, 14.0), (3.6, 11.0), (3.6, 8.5)]); g.circle(8.0, 10.6, 4.4); g.line_c(&[(6.0, 9.0), (5.6, 11.5)], 0.8, Color32::from_white_alpha(150)); }
        Lightning => { g.poly(&[(9.5, 1.0), (4.0, 9.0), (7.5, 9.0)]); g.poly(&[(9.5, 1.0), (7.5, 9.0), (11.0, 7.0)]); g.poly(&[(8.5, 7.0), (12.0, 7.0), (6.5, 15.0)]); }
        Meteor => { g.poly_c(&[(7.0, 10.6), (1.0, 1.0), (10.6, 7.0)], Color32::from_rgb(244, 132, 52)); g.poly_c(&[(8.2, 10.0), (3.6, 3.6), (10.0, 8.2)], Color32::from_rgb(255, 214, 96)); g.circle(10.8, 10.8, 3.9); g.circle_c(12.0, 9.8, 1.0, dk); g.circle_c(9.6, 12.0, 0.7, dk); }
        Snow => { for i in 0..3 { let a = i as f32 * std::f32::consts::PI / 3.0; g.line(&[(8.0 - 6.5 * a.cos(), 8.0 - 6.5 * a.sin()), (8.0 + 6.5 * a.cos(), 8.0 + 6.5 * a.sin())], 1.4); } }
        Raise => { g.poly(&[(1.0, 14.5), (6.0, 6.0), (10.0, 10.0), (12.0, 7.5), (15.0, 14.5)]); g.poly(&[(8.0, 1.0), (11.0, 4.5), (5.0, 4.5)]); g.rect(7.2, 4.5, 1.6, 3.0); }
        Lower => { g.poly(&[(1.0, 9.0), (15.0, 9.0), (15.0, 14.5), (1.0, 14.5)]); g.poly_c(&[(5.0, 9.0), (11.0, 9.0), (9.5, 12.0), (6.5, 12.0)], dk); g.poly(&[(8.0, 7.5), (5.0, 4.0), (11.0, 4.0)]); g.rect(7.2, 1.0, 1.6, 3.0); }
        Block => { let c = g.c; g.poly(&[(8.0, 1.5), (14.5, 4.8), (8.0, 8.1), (1.5, 4.8)]); g.poly_c(&[(1.5, 4.8), (8.0, 8.1), (8.0, 14.8), (1.5, 11.5)], c.lerp_to_gamma(Color32::BLACK, 0.25)); g.poly_c(&[(8.0, 8.1), (14.5, 4.8), (14.5, 11.5), (8.0, 14.8)], c.lerp_to_gamma(Color32::BLACK, 0.45)); }
        Shovel => { g.line(&[(13.5, 2.5), (7.0, 9.0)], 1.4); g.line(&[(12.0, 1.5), (15.0, 4.5)], 1.4); g.poly(&[(5.0, 8.0), (8.0, 11.0), (5.0, 14.5), (1.5, 14.5), (1.5, 11.0)]); }
        Person => { g.circle(8.0, 3.8, 2.8); g.poly(&[(4.0, 15.0), (4.6, 7.8), (11.4, 7.8), (12.0, 15.0)]); }
        Sheep => { g.circle(6.0, 8.0, 3.4); g.circle(9.5, 7.5, 3.6); g.circle(11.5, 9.0, 2.6); g.rect(4.0, 8.0, 8.0, 3.4); g.circle_c(13.2, 6.6, 1.8, Color32::from_rgb(60, 52, 46)); for x in [4.8, 7.0, 9.4, 11.4] { g.rect_c(x, 11.0, 1.0, 3.5, Color32::from_rgb(60, 52, 46)); } }
        Fish => { g.poly(&[(1.5, 8.0), (5.0, 4.5), (10.5, 4.8), (12.0, 8.0), (10.5, 11.2), (5.0, 11.5)]); g.poly(&[(11.0, 8.0), (15.0, 4.5), (15.0, 11.5)]); g.circle_c(4.6, 7.4, 0.8, dk); }
        Sparkle => { g.star(8.0, 8.0, 7.0, 4); g.star(13.0, 3.0, 2.2, 4); }
        Quake => { g.line(&[(1.0, 5.0), (4.0, 3.0), (7.0, 6.0), (10.0, 3.0), (15.0, 5.0)], 1.2); g.poly(&[(1.0, 9.0), (15.0, 9.0), (15.0, 15.0), (1.0, 15.0)]); g.line_c(&[(8.0, 9.0), (6.5, 11.5), (9.0, 13.0), (7.5, 15.0)], 1.1, dk); }
        Bomb => { g.circle(7.0, 10.0, 5.2); g.rect(9.0, 3.5, 3.0, 3.0); g.line(&[(11.0, 4.0), (13.0, 1.8)], 1.0); g.circle_c(13.6, 1.5, 1.2, Color32::from_rgb(255, 210, 90)); g.circle_c(5.2, 8.2, 1.2, Color32::from_white_alpha(120)); }
        Nuke => { g.circle(8.0, 8.0, 7.0); g.c = dk; for i in 0..3 { let a = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::TAU / 3.0; let b = a + 1.0; g.poly(&[(8.0, 8.0), (8.0 + 6.0 * a.cos(), 8.0 + 6.0 * a.sin()), (8.0 + 6.0 * ((a + b) / 2.0).cos(), 8.0 + 6.0 * ((a + b) / 2.0).sin()), (8.0 + 6.0 * b.cos(), 8.0 + 6.0 * b.sin())]); } g.circle(8.0, 8.0, 1.6); }
        Magnifier => { g.ring(6.5, 6.5, 4.4, 1.8); g.line(&[(9.8, 9.8), (14.5, 14.5)], 2.4); }
        Ghost => { g.circle(8.0, 6.5, 5.2); g.poly(&[(2.8, 6.5), (13.2, 6.5), (13.2, 13.0), (2.8, 13.0)]); for x in [4.5, 8.0, 11.5] { g.circle(x, 13.2, 1.7); } g.circle_c(6.0, 6.5, 1.0, dk); g.circle_c(10.0, 6.5, 1.0, dk); }
        Pause => { g.rect(3.5, 2.5, 3.2, 11.0); g.rect(9.3, 2.5, 3.2, 11.0); }
        Play => g.poly(&[(4.0, 2.0), (13.5, 8.0), (4.0, 14.0)]),
        Fast => { g.poly(&[(1.5, 3.0), (8.0, 8.0), (1.5, 13.0)]); g.poly(&[(8.0, 3.0), (14.5, 8.0), (8.0, 13.0)]); }
        Menu => { for y in [3.0, 7.0, 11.0] { g.rect(2.0, y, 12.0, 2.2); } }
        Close => { g.line(&[(3.0, 3.0), (13.0, 13.0)], 2.2); g.line(&[(13.0, 3.0), (3.0, 13.0)], 2.2); }
        Check => g.line(&[(2.5, 8.5), (6.5, 12.5), (13.5, 3.5)], 2.4),
        Save => { g.poly(&[(2.0, 2.0), (12.0, 2.0), (14.0, 4.0), (14.0, 14.0), (2.0, 14.0)]); g.rect_c(4.5, 2.0, 7.0, 4.0, dk); g.rect_c(4.0, 9.0, 8.0, 5.0, Color32::from_rgb(246, 240, 226)); }
        Dice => { g.poly(&[(2.0, 2.0), (14.0, 2.0), (14.0, 14.0), (2.0, 14.0)]); for (x, y) in [(5.0, 5.0), (11.0, 11.0), (8.0, 8.0), (11.0, 5.0), (5.0, 11.0)] { g.circle_c(x, y, 1.2, dk); } }
        ArrowRight => g.poly(&[(4.0, 2.0), (13.0, 8.0), (4.0, 14.0)]),
        ArrowLeft => g.poly(&[(12.0, 2.0), (3.0, 8.0), (12.0, 14.0)]),
        ArrowUp => g.poly(&[(2.0, 12.0), (8.0, 3.0), (14.0, 12.0)]),
        ArrowDown => g.poly(&[(2.0, 4.0), (14.0, 4.0), (8.0, 13.0)]),
        Hammer => { g.line(&[(3.0, 14.0), (10.0, 7.0)], 1.6); g.poly(&[(7.5, 3.5), (11.0, 0.8), (15.2, 5.0), (12.5, 8.5)]); }
    }
}
