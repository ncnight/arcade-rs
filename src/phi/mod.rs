
#[macro_use]
mod events;

use sdl2::render::Renderer;

struct_events!(
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
    },
    else: {
        quit: Quit {..}
    }
);


pub struct Phi<'window>{
    pub events: Events,
    pub render: Renderer<'window>,
}

pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}


pub trait View {
    //called every frame
    //elapsed is in seconds
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

pub fn spawn<F>(title: &str, init: F)
    where F: Fn(&mut Phi) -> Box<View> {
        //init sdl2
        let sdl_context = ::sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let mut timer = sdl_context.timer().unwrap();

        //window
        let window = video.window(title, 800, 600).position_centered().opengl().build().unwrap();

        let mut context = Phi{
            //events record
            events: Events::new(sdl_context.event_pump().unwrap()),
            render: window.renderer().accelerated().build().unwrap(),
        };

        //DefaultView
        let mut current_view = init(&mut context);

        //frames
        let interval = 1_000 /60;
        let mut before = timer.ticks();
        let mut last_second = timer.ticks();
        let mut fps = 0u16;

        loop {

            //frame timing
            let now = timer.ticks();
            let dt = now - before;
            let elapsed = dt as f64 / 1_000.0;

            //fps cap
            if dt < interval {
                timer.delay(interval - dt);
                continue;
            }

            before = now;
            fps += 1;

            if now - last_second > 1_000 {
                println!("FPS: {}", fps);
                last_second = now;
                fps = 0;
            }

            //logic & renders

            context.events.pump();

            match current_view.render(&mut context, 0.01) {
                ViewAction::None => context.render.present(),
                ViewAction::Quit => break,
                ViewAction::ChangeView(new_view) =>
                    current_view = new_view,
            }
        }
    }
