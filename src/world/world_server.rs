use std::mem;

use deku::DekuContainerRead;
use tokio::{
  io::{AsyncReadExt, WriteHalf},
  net::{TcpListener, TcpStream},
};

use crate::{
  common::packets::process_packets,
  structs::{common::FFXIVARRPacketHeader, world_structs::PlayerInfo},
  world::oodle::FFXIVOodle,
};

use super::world_packets;

fn get_packet_header(buf: &[u8]) -> FFXIVARRPacketHeader {
  let header_size = mem::size_of::<FFXIVARRPacketHeader>();
  let remaining_bytes = buf.len();
  if remaining_bytes < header_size {
    panic!("Header too small! Malformed request?");
  }

  let (_rest, header) = FFXIVARRPacketHeader::from_bytes((buf, 0)).unwrap();

  header
}

async fn world(
  packet: Vec<u8>,
  header: FFXIVARRPacketHeader,
  write_socket: &mut WriteHalf<TcpStream>,
  oodle: &mut FFXIVOodle,
  player_info: &mut PlayerInfo,
) {
  let header_size = mem::size_of::<FFXIVARRPacketHeader>();
  let decomp_buf = &packet[header_size..];
  let packets = process_packets(decomp_buf, &header, Some(oodle)).await;

  world_packets::handle_world_packets(write_socket, packets, &header, player_info).await;
}

pub async fn start_world() -> Result<(), Box<dyn std::error::Error>> {
  println!(" - world");
  let listener = TcpListener::bind("127.0.0.1:54995").await?;

  loop {
    let (socket, addr) = listener.accept().await?;
    let (mut read, mut write) = tokio::io::split(socket);

    println!(
      "[world] connection established from {:?} - port {:?}",
      addr.ip(),
      addr.port()
    );
    let mut oodle_instance = FFXIVOodle::new();
    let mut player_info = PlayerInfo {
      ..Default::default()
    };

    tokio::spawn(async move {
      let mut packet_buf = vec![0; 2056];

      loop {
        let packet_n = read
          .read(&mut packet_buf)
          .await
          .expect("unable to read from socket!");

        if packet_n == 0 {
          return;
        }
        let packet = packet_buf[..packet_n].to_vec();
        let header = get_packet_header(&packet);

        world(
          packet,
          header,
          &mut write,
          &mut oodle_instance,
          &mut player_info,
        )
        .await;
      }
    });
  }
}
