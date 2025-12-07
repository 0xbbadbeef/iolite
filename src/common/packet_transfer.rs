use deku::DekuContainerWrite;
use std::{
  mem,
  time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
  io::{AsyncWriteExt, WriteHalf},
  net::TcpStream,
};

use crate::structs::{
  common::{FFXIVARRPacketHeader, FFXIVARRPacketSegmentRaw, FFXIVARRSegmentHeader},
  lobby_structs::FFXIVIpcHeader,
};

pub fn get_ipc_header(ipc_type: u16) -> FFXIVIpcHeader {
  FFXIVIpcHeader {
    timestamp: SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards")
      .as_secs()
      .try_into()
      .unwrap(),
    ipc_type,
    ..Default::default()
  }
}

fn get_packet_size(segments: &[Vec<u8>]) -> u32 {
  let size = segments
    .iter()
    .fold(0usize, |acc, segment| acc + segment.len());

  (size + mem::size_of::<FFXIVARRPacketHeader>())
    .try_into()
    .unwrap()
}

pub async fn send_packet(socket: &mut WriteHalf<TcpStream>, segments: Vec<Vec<u8>>) {
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

pub async fn send_ipc_packet(socket: &mut WriteHalf<TcpStream>, segments: Vec<Vec<u8>>) {
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

pub fn create_packet_segment(
  segment_type: u16,
  data: Vec<u8>,
  source_actor: u32,
  target_actor: u32,
) -> FFXIVARRPacketSegmentRaw {
  // let seg_size: u32 = mem::size_of::<FFXIVARRSegmentHeader>().try_into().unwrap();
  let seg_hdr = FFXIVARRSegmentHeader {
    segment_type,
    source_actor,
    target_actor,
    size: (data.len() as u32),
    ..Default::default()
  };

  FFXIVARRPacketSegmentRaw { seg_hdr, data }
}

pub async fn send_keep_alive(
  socket: &mut WriteHalf<TcpStream>,
  packet_segment: FFXIVARRPacketSegmentRaw,
) {
  let id = packet_segment.data[0..4].to_vec();
  let time_stamp = packet_segment.data[4..4].to_vec();

  let mut data = vec![];
  data.extend(id);
  data.extend(time_stamp);

  let response_packet = FFXIVARRPacketSegmentRaw {
    seg_hdr: FFXIVARRSegmentHeader {
      size: 0x18,
      segment_type: 0x08,
      ..Default::default()
    },
    data,
  };

  send_packet(socket, vec![response_packet.to_bytes().unwrap()]).await;
}
