use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;
use sdl2::rect::Rect as SdlRect;

const PLAYER_SPEED: f64 = 180.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h:f64,
}

impl Rectangle {
    ///Sdl equivalent to self
    pub fn to_sdl(self) -> Option<SdlRect> {
        //no negs
        assert!(self.w >= 0.0 && self.h >= 0.0);
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32).unwrap()
    }
}


struct Ship {
        rect: Rectangle,
}


pub struct ShipView{
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        ShipView {
            player: Ship{
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: 32.0,
                    h: 32.0,
                }
            }
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        //logic
        phi.render.set_draw_color(Color::RGB(0,0,0));
        phi.render.clear(); //clear screen

        //rendering
        phi.render.set_draw_color(Color::RGB(200,200,50));
        phi.render.fill_rect(self.player.rect.to_sdl().unwrap());
        ViewAction::None

    }
}
