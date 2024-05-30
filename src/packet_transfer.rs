use deku::DekuContainerWrite;
use std::{
  mem,
  time::{SystemTime, UNIX_EPOCH},
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::structs::common::FFXIVARRPacketHeader;

fn get_packet_size(segments: &Vec<Vec<u8>>) -> u32 {
  let size = segments
    .iter()
    .fold(0usize, |acc, segment| acc + segment.len());
  let complete_size = (size + mem::size_of::<FFXIVARRPacketHeader>())
    .try_into()
    .unwrap();

  complete_size
}

pub async fn send_packet(socket: &mut TcpStream, segments: Vec<Vec<u8>>) {
  let complete_size = get_packet_size(&segments);

  let response_header = FFXIVARRPacketHeader {
    timestamp: SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards")
      .as_millis()
      .try_into()
      .unwrap(),
    unknown_0: 0xE2465DFF41A05252,
    unknown_8: 0x75C4997B4D642A7F,
    size: complete_size,
    count: segments.len().try_into().unwrap(),
    ..Default::default()
  };
  let mut finalised_response_header = response_header.to_bytes().unwrap();
  segments
    .iter()
    .for_each(|segment| finalised_response_header.extend(segment));

  socket
    .write_all(&finalised_response_header)
    .await
    .expect("Failed to write to socket");
}

pub async fn send_ipc_packet(socket: &mut TcpStream, segments: Vec<Vec<u8>>) {
  let complete_size = get_packet_size(&segments);

  let response_header = FFXIVARRPacketHeader {
    size: complete_size,
    count: segments.len().try_into().unwrap(),
    ..Default::default()
  };
  let mut finalised_response_header = response_header.to_bytes().unwrap();
  segments
    .iter()
    .for_each(|segment| finalised_response_header.extend(segment));

  socket
    .write_all(&finalised_response_header)
    .await
    .expect("Failed to write to socket");
}
