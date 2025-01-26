use std::{path::Path, process::Command};

const GAME_PATH: &str = "G:\\FFXIV\\game\\ffxiv_dx11";

fn exec_params(session_id: i32) -> [String; 11] {
  [
    format!("DEV.TestSID={} DEV.UseSqPack=1 DEV.DataPathType=1", session_id),
    "DEV.LobbyHost01=127.0.0.1 DEV.LobbyPort01=54994".to_string(),
    "DEV.LobbyHost02=127.0.0.1 DEV.LobbyPort02=54994".to_string(),
    "DEV.LobbyHost03=127.0.0.1 DEV.LobbyPort03=54994".to_string(),
    "DEV.LobbyHost04=127.0.0.1 DEV.LobbyPort04=54994".to_string(),
    "DEV.LobbyHost05=127.0.0.1 DEV.LobbyPort05=54994".to_string(),
    "DEV.LobbyHost06=127.0.0.1 DEV.LobbyPort06=54994".to_string(),
    "DEV.LobbyHost07=127.0.0.1 DEV.LobbyPort07=54994".to_string(),
    "DEV.LobbyHost08=127.0.0.1 DEV.LobbyPort08=54994".to_string(),
    "DEV.LobbyHost09=127.0.0.1 DEV.LobbyPort09=54994".to_string(),
    "SYS.Region=3 language=1 version=1.0.0.0 DEV.MaxEntitledExpansionID=5 DEV.GMServerHost=127.0.0.1".to_string(),
  ]
}

pub fn main() {
  println!("starting client at {}", GAME_PATH);
  Command::new(Path::new(GAME_PATH))
    .args(exec_params(0))
    .spawn()
    .unwrap();
}
