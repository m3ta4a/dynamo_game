use dynamo_lib::geometry::quad::Quad;

pub struct Player {
  pub body: Vec<Quad>,
  pub score: u32,
  pub visible: bool,
}

impl Player {
  pub fn new(position: cgmath::Vector2<f32>) -> Player {
    Player {
      body: vec![Quad::new(position, (0.0, 0.0).into())],
      score: 0,
      visible: false,
    }
  }
}
