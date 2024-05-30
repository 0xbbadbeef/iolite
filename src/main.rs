pub mod game_packet_parser;
pub mod handle_lobby_packet;
pub mod handle_packets;
pub mod packet_transfer;
pub mod structs;

use core::panic;
use std::sync::Arc;
use deku::DekuContainerRead;
use std::collections::HashMap;
use std::mem;
use std::path::Path;
use std::process::Command;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

use crate::structs::common::FFXIVARRPacketHeader;

type Db = Arc<Mutex<HashMap<String, Vec<u8>>>>;

async fn lobby(buf: Vec<u8>, db: &Db, socket: &mut TcpStream) {
  let header_size = mem::size_of::<FFXIVARRPacketHeader>();
  let remaining_bytes = buf.len();
  if remaining_bytes < header_size {
    panic!("Header too small! Malformed request?");
  }

  let (_rest, header) = FFXIVARRPacketHeader::from_bytes((&buf[..], 0)).unwrap();

  let packets = game_packet_parser::process_packets(&buf, header, header_size);

  handle_packets::handle_packets(packets, db, socket).await;
}

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
    "SYS.Region=3 language=1 version=1.0.0.0 DEV.MaxEntitledExpansionID=3 DEV.GMServerHost=127.0.0.1".to_string(),
  ]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("================Starting================");

  Command::new(Path::new(r"G:\FFXIV\game\ffxiv_dx11"))
    .args(exec_params(0))
    .spawn()
    .unwrap();

  println!("starting lobby server..");
  let listener = TcpListener::bind("127.0.0.1:54994").await?;

  let db: Db = Arc::new(Mutex::new(HashMap::<String, Vec<u8>>::new()));

  loop {
    let (mut socket, _) = listener.accept().await?;

    let database = db.clone();

    tokio::spawn(async move {
      loop {
        let mut buf = [0; 2056];
        let n = socket
          .read(&mut buf)
          .await
          .expect("failed to read data from socket");

        if n == 0 {
          return;
        }

        lobby(buf[..n].into(), &database, &mut socket).await;
      }
    });
  }
}
