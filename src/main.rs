use draw::{draw_grid, draw_o, draw_x};
use input::InputHandler;
use render::{RenderGroup, WindowState};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

mod draw;
mod game;
mod input;
mod render;

enum Scene {
    Game(game::GameState),
}

impl Scene {
    fn get_meshes<'a>(&'a self, ws: &WindowState) -> Vec<RenderGroup<'a>> {
        match self {
            Self::Game(gs) => gs.get_meshes(ws),
        }
    }

    fn update(&mut self, inp: &InputHandler) {
        match self {
            Self::Game(gs) => gs.update(inp),
        }
    }
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut ws = render::WindowState::new(&window).await;
    let mut inp = input::InputHandler::new(ws.window().inner_size());

    let grid_sprite_bytes = draw_grid(120, 120, 5.0);
    let x_sprite_bytes = draw_x(120, 120, 5.0);
    let o_sprite_bytes = draw_o(120, 120, 5.0);
    let grid_sprite = ws.make_sprite(120, 120, grid_sprite_bytes);
    let x_sprite = ws.make_sprite(120, 120, x_sprite_bytes);
    let o_sprite = ws.make_sprite(120, 120, o_sprite_bytes);

    let mut gs = game::GameState::new(vec![grid_sprite, x_sprite, o_sprite]);
    gs.register_ent(-0.75, 0.75, 1.5, 1.5, 0);

    let mut scene = Scene::Game(gs);

    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == ws.window().id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                WindowEvent::RedrawRequested => {
                    scene.update(&inp);
                    match ws.render(&scene.get_meshes(&ws)[..]) {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => ws.resize(ws.size),
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                    ws.window().request_redraw();
                }
                _ => {
                    inp.register_event(&event);
                }
            },
            _ => {}
        })
        .unwrap();
}

fn main() {
    pollster::block_on(run());
}
