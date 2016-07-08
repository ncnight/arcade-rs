use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use sdl2::pixels::Color;
use phi::gfx::{CopySprite, Sprite};

const PLAYER_SPEED: f64 = 180.0;
const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;

#[derive(Clone, Copy)]
enum ShipFrame { //N = normal, F = fast, S = slow
    UpN = 0,
    UpF = 1,
    UpS = 2,
    MidN = 3,
    MidF = 4,
    MidS = 5,
    DownN = 6,
    DownF = 7,
    DownS = 8,
}

struct Ship {
        rect: Rectangle,
        sprites: Vec<Sprite>,
        current: ShipFrame,
}


pub struct ShipView{
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        //find ship
        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        let mut sprites = Vec::with_capacity(9);

        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                    w: SHIP_W,
                    h: SHIP_H,
                    x: SHIP_W * x as f64,
                    y: SHIP_H * y as f64,
                    }).unwrap());
            }
        }

        //construct ship
        ShipView {
            player: Ship{
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: SHIP_W,
                    h: SHIP_H,
                },
                sprites: sprites,
                current: ShipFrame::MidN,
            }
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        //ship movement
        let diagonal = (phi.events.key_up ^ phi.events.key_down) && (phi.events.key_left ^ phi.events.key_right);

        let moved = if diagonal{1.0 / 2.0f64.sqrt()} else {1.0} * PLAYER_SPEED * elapsed;

        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;

        self.player.rect.y += dy;

        //height of the window and 70% of the width
        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 *0.70,
            h: phi.output_size().1,
        };

        //if cant fill abort
        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();

        //right sprite
        self.player.current =
            if dx == 0.0 && dy < 0.0 {ShipFrame::UpN}
            else if dx > 0.0 && dy < 0.0 {ShipFrame::UpF}
            else if dx < 0.0 && dy < 0.0 {ShipFrame::UpS}
            else if dx == 0.0 && dy == 0.0 {ShipFrame::MidN}
            else if dx > 0.0 && dy == 0.0 {ShipFrame::MidF}
            else if dx < 0.0 && dy == 0.0 {ShipFrame::MidS}
            else if dx == 0.0 && dy > 0.0 {ShipFrame::DownN}
            else if dx > 0.0 && dy > 0.0 {ShipFrame::DownF}
            else if dx < 0.0 && dy > 0.0 {ShipFrame::DownS}
            else {unreachable!()};

        //logic
        phi.renderer.set_draw_color(Color::RGB(0,0,0));
        phi.renderer.clear(); //clear screen

        //rendering
        phi.renderer.set_draw_color(Color::RGB(200,200,50));
        phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());

        //render
        phi.renderer.copy_sprite(&self.player.sprites[self.player.current as usize], self.player.rect);

        ViewAction::None

    }
}
