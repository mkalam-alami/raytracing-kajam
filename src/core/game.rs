use winit::window::Window;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::main_scene::MainScene;

#[derive(Clone)]
pub struct GameSettings {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub scene: MainScene // TODO figure out how to use a trait without annoying the compiler with dynamic sizes
}

pub struct Game;
impl Game {
    pub fn run(settings: &GameSettings) -> Result<(), Error> {
        let event_loop = EventLoop::new();
        let window = Game::create_window(settings, &event_loop);
        let mut input = WinitInputHelper::new();
        let mut pixels = Game::create_pixels(settings, &window)?;
        let mut scene = settings.scene.clone();

        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                scene.draw(pixels.get_frame());
                if pixels.render().map_err(|e| error!("pixels.render() failed: {}", e)).is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
    
            // Handle input events
            if input.update(event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
    
                // Resize the window
                if let Some(size) = input.window_resized() {
                    pixels.resize(size.width, size.height);
                }
    
                // Update internal state and request a redraw
                scene.update();
                window.request_redraw();
            }
        });
    }

    fn create_window(settings: &GameSettings, event_loop: &EventLoop<()>) -> Window {
        let size = LogicalSize::new((settings.width * 2) as f64, (settings.height * 2) as f64);
        WindowBuilder::new()
            .with_title(settings.title.clone())
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(event_loop)
            .unwrap()
    }

    fn create_pixels(settings: &GameSettings, window: &Window) -> Result<Pixels<Window>, Error> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        Pixels::new(settings.width, settings.height, surface_texture)
    }
}