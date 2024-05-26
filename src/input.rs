use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, WindowEvent},
};

fn physical_to_screen_space(pos: [f64; 2], screen_size: PhysicalSize<u32>) -> [f64; 2] {
    let x = (pos[0] / screen_size.width as f64) * 2.0 - 1.0;
    let y = (pos[1] / screen_size.height as f64) * 2.0 - 1.0;
    [x, -y]
}

// just care about button presses, makes my life easy
pub struct InputHandler {
    pub lmb: bool,
    pub rmb: bool,
    mpos: [f64; 2],
    pub scrn_size: PhysicalSize<u32>,
}

impl InputHandler {
    pub fn new(scrn_size: PhysicalSize<u32>) -> Self {
        Self {
            lmb: false,
            rmb: false,
            mpos: [0.0, 0.0],
            scrn_size,
        }
    }

    pub fn register_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseInput {
                state,
                button,
                device_id,
            } => match button {
                MouseButton::Left => self.lmb = *state == ElementState::Pressed,
                MouseButton::Right => self.rmb = *state == ElementState::Pressed,
                _ => {}
            },
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => self.mpos = [position.x, position.y],
            _ => {}
        }
    }

    pub fn get_screen_mouse_position(&self) -> [f64; 2] {
        physical_to_screen_space(self.mpos, self.scrn_size)
    }
}
