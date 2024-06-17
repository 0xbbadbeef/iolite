use core::panic;
use deku::DekuContainerRead;
use std::collections::HashMap;
use std::mem;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

use crate::common::packets::process_packets;
use crate::structs::common::FFXIVARRPacketHeader;

use super::handle_packets;

pub type UnlockedDb = Arc<Mutex<HashMap<String, Vec<u8>>>>;

async fn lobby(buf: Vec<u8>, db: &UnlockedDb, socket: &mut WriteHalf<TcpStream>) {
  let header_size = mem::size_of::<FFXIVARRPacketHeader>();
  let remaining_bytes = buf.len();
  if remaining_bytes < header_size {
    panic!("Header too small! Malformed request?");
  }

  let (_rest, header) = FFXIVARRPacketHeader::from_bytes((&buf[..], 0)).unwrap();
  let decomp_buf = &buf[header_size..];

  let packets = process_packets(decomp_buf, &header, None).await;

  handle_packets::handle_packets(packets, db, socket).await;
}

pub async fn start_lobby() -> Result<(), Box<dyn std::error::Error>> {
  println!(" - lobby");
  let listener = TcpListener::bind("127.0.0.1:54994").await?;

  let db: UnlockedDb = Arc::new(Mutex::new(HashMap::<String, Vec<u8>>::new()));

  loop {
    let (socket, _) = listener.accept().await?;
    let (mut read, mut write) = tokio::io::split(socket);

    let database = db.clone();

    tokio::spawn(async move {
      let mut buf = [0; 2056];
      loop {
        let n = read
          .read(&mut buf)
          .await
          .expect("failed to read data from socket");

        if n == 0 {
          return;
        }

        lobby(buf[..n].into(), &database, &mut write).await;
      }
    });
  }
}
