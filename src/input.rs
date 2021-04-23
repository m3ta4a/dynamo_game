use dynamo_lib::keyboard::*;

#[derive(Debug, Default)]
pub struct Input {
  pub up_pressed: bool,
  pub down_pressed: bool,
  pub left_pressed: bool,
  pub right_pressed: bool,
  pub enter_pressed: bool,
  pub esc_pressed: bool,
}

impl Input {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn update(&mut self, input: KeyboardInput) {
    let pressed = input.state == KeyboardKeyState::Pressed;
    match input.key {
      KeyboardKey::Up => {
        self.up_pressed = pressed;
      }
      KeyboardKey::Down => {
        self.down_pressed = pressed;
      }
      KeyboardKey::W => {
        self.up_pressed = pressed;
      }
      KeyboardKey::S => {
        self.down_pressed = pressed;
      }
      KeyboardKey::Left => {
        self.left_pressed = pressed;
      }
      KeyboardKey::Right => {
        self.right_pressed = pressed;
      }
      KeyboardKey::A => {
        self.left_pressed = pressed;
      }
      KeyboardKey::D => {
        self.right_pressed = pressed;
      }
      KeyboardKey::Return => {
        self.enter_pressed = pressed;
      }
      KeyboardKey::Escape => {
        self.esc_pressed = true;
      }
      _ => (),
    }
  }

  pub fn ui_up_pressed(&self) -> bool {
    self.up_pressed
  }

  pub fn ui_down_pressed(&self) -> bool {
    self.down_pressed || self.down_pressed
  }

  pub fn clear(&mut self) {
    self.up_pressed = false;
    self.down_pressed = false;
    self.up_pressed = false;
    self.down_pressed = false;
    self.enter_pressed = false;
    self.esc_pressed = false;
  }
}
