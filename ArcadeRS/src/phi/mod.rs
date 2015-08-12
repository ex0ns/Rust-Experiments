use ::sdl2::render::Renderer;
use ::sdl2::timer;

pub mod data;

#[macro_use] mod events;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
    },
    else: {
        quit: Quit { .. }
    }
}

pub struct Phi<'a> {
    pub events: Events<'a>,
    pub renderer: Renderer<'a>,
}
 
impl<'a> Phi<'a> {
    pub fn output_size(&self) -> (u32, u32) {
        self.renderer.get_output_size().unwrap()
    }
}

pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    fn resume(&mut self, _context: &mut Phi) {}
    fn pause(&mut self,  _context: &mut Phi) {}
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}


pub fn spawn<F>(title: &str, init: F)
    where F: Fn(&mut Phi) -> Box<View> {

    let mut sdl_context = ::sdl2::init().video()
        .build().unwrap();


    let window = sdl_context.window(title, 800, 600)
        .position_centered().opengl().resizable()
        .build().unwrap();

    let renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    let events = Events::new(sdl_context.event_pump());

    let mut context = ::phi::Phi {
        events: events,
        renderer: renderer,
    };

    let mut current_view = init(&mut context);
    current_view.resume(&mut context);

    let interval = 1_000 / 60;
    let mut before = timer::get_ticks();
    let mut last_second = timer::get_ticks();
    let mut fps = 0u16;

    loop {

        let now = timer::get_ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        if dt < interval {
            timer::delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }

        context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => {
                current_view.pause(&mut context);
                break;
            },
            ViewAction::ChangeView(new_view) => {
                current_view.pause(&mut context);
                current_view = new_view;
                current_view.resume(&mut context);
            },
        }
    }
}

