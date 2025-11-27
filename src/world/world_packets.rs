use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
  common::packet_transfer::{create_packet_segment, send_keep_alive, send_packet},
  structs::{
    common::{
      FFXIVARRPacketHeader, FFXIVARRPacketSegmentRaw, FFXIVARRSegmentType, FFXIVConnectionType,
    },
    world_structs::PlayerInfo,
  },
  world::handle_zone_packets,
};
use deku::DekuContainerWrite;
use tokio::{io::WriteHalf, net::TcpStream};

pub async fn handle_world_packets(
  write_socket: &mut WriteHalf<TcpStream>,
  packets: Vec<FFXIVARRPacketSegmentRaw>,
  packet_header: &FFXIVARRPacketHeader,
  player_info: &mut PlayerInfo,
) {
  for packet_segment in packets {
    match packet_segment.seg_hdr.segment_type.try_into().unwrap() {
      FFXIVARRSegmentType::SessionInit => {
        let player_id_bytes: [u8; 4] = packet_segment.data[4..8].try_into().unwrap();
        let player_id = u32::from_le_bytes(player_id_bytes);
        player_info.id = player_id;

        let mut init_data_packet = 0xE0037603u32.to_le_bytes().to_vec();
        init_data_packet.extend(
          SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            .to_ne_bytes(),
        );
        init_data_packet.resize(0x18, 0);

        let init_packet = create_packet_segment(0x07, init_data_packet, 0, 0);
        send_packet(write_socket, vec![init_packet.to_bytes().unwrap()]).await;

        match packet_header.connection_type.try_into().unwrap() {
          FFXIVConnectionType::Zone => {
            println!("[world] init zone");
            let mut zone_init_data = player_id_bytes.to_vec();
            zone_init_data.resize(0x38, 0);

            let zone_init_packet = create_packet_segment(0x02, zone_init_data, 0, 0);
            send_packet(write_socket, vec![zone_init_packet.to_bytes().unwrap()]).await;
          }
          FFXIVConnectionType::Chat => {
            println!("[world]: init chat");
            let mut first_chat_data = player_id_bytes.to_vec();
            first_chat_data.resize(0x38, 0);
            let chat_init_packet = create_packet_segment(0x02, first_chat_data, 0, 0);
            send_packet(write_socket, vec![chat_init_packet.to_bytes().unwrap()]).await;

            let mut second_chat_data = vec![0u8; 0x28];
            second_chat_data[2] = 0x02;
            let second_chat_packet =
              create_packet_segment(0x03, second_chat_data, player_id, player_id);
            send_packet(write_socket, vec![second_chat_packet.to_bytes().unwrap()]).await;
          }
          _ => {
            panic!("unhandled world packet type!")
          }
        }
      }
      FFXIVARRSegmentType::IPC => {
        handle_zone_packets::process(write_socket, packet_segment, player_info).await;
      }
      FFXIVARRSegmentType::KeepAlive => {
        println!("world keep alive");
        send_keep_alive(write_socket, packet_segment).await;
      }
      FFXIVARRSegmentType::SegmentTypeResponse => {
        println!("segment type response?");
      }
      _ => {
        panic!("unhandled world packet!")
      }
    }
  }
}
