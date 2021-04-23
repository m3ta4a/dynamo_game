use dynamo_lib::start;

mod dynamo_game;
mod input;
mod player;
mod state;
mod system;
mod util;

use dynamo_game::DynamoGame;

fn main() {
  let game = DynamoGame::new();
  start("Game", Box::new(game));
}
