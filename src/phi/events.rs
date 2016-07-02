macro_rules! struct_events {
    ( keyboard: { $( $k_alias:ident : $k_sdl:ident), * },
    //match against a pattern
    else: { $( $e_alias:ident : $e_sdl:pat), *}
    ) =>  {
        use sdl2::EventPump;

        pub struct ImmediateEvents {
            $( pub $k_alias: Option<bool>, ) *
            $( pub $e_alias: bool,) *
            resize: Option<(u32, u32)>,
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    $( $k_alias: None,)*
                    $( $e_alias: false,)*
                    resize: None,
                }
            }
        }

        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,

            // true = pressed
            // false = not pressed
            $( pub $k_alias: bool,)*
            $( pub $e_alias: bool), *
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),

                    $( $k_alias: false,)*
                    $( $e_alias: false),*
                }
            }

            //update record
            pub fn pump(&mut self, renderer: &mut ::sdl2::render::Renderer) {
                self.now = ImmediateEvents::new();
                //sdl_context dropped then poll_iter() wont yield input
                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::event::WindowEventId::Resized;
                    use sdl2::keyboard::Keycode::*;

                    match event {
                        Window {win_event_id: Resized, .. } => {
                            self.now.resize = Some(renderer.output_size().unwrap());
                        },

                        KeyDown {keycode, ..} => match keycode {
                            $(
                                Some($k_sdl) => {
                                    //prevent multiple presses
                                    //checks if previously pressed
                                    if !self.$k_alias {
                                        //pressed
                                        self.now.$k_alias = Some(true);
                                    }
                                    self.$k_alias = true;
                                }
                            ),*

                            _ => {}
                        },

                        KeyUp{keycode, ..} => match keycode {
                            $(
                                Some($k_sdl) => {
                                    //released
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false;
                                }
                            ),*
                            _=>{}
                        },

                        $(
                            $e_sdl => {
                                self.now.$e_alias = true;
                            }
                        )*,

                        _ => {}
                    }
                }
            }
        }
    }
}
