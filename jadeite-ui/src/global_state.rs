pub struct GlobalState {
    pub context: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub event_pump: sdl2::EventPump,
}

impl GlobalState {
    pub fn init() -> Self {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let event_pump = context.event_pump().unwrap();
        Self { context, video, event_pump }
    }
}
