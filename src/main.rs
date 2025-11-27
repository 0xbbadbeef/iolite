pub mod blowfish;
pub mod common;
pub mod lobby;
pub mod structs;
pub mod world;

#[tokio::main]
async fn main() {
  println!("================Starting================");

  println!("starting servers..");
  let (_, _) = tokio::join!(
    lobby::lobby_server::start_lobby(),
    world::world_server::start_world()
  );
}
