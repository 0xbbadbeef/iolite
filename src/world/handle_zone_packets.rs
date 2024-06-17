use deku::{DekuContainerRead, DekuContainerWrite};
use tokio::{io::WriteHalf, net::TcpStream};

use crate::{
  common::{
    packet_transfer::{create_packet_segment, get_ipc_header, send_ipc_packet},
    packets::IpcResponse,
  },
  structs::{
    common::{
      FFXIVARRPacketSegmentRaw, FFXIVARRPosition, CLASSJOB_SLOTS, CONTENT_ID,
    },
    world_structs::{
      FFXIVIpcActorControlSelf, FFXIVIpcBlackList, FFXIVIpcInit, FFXIVIpcInitZone, FFXIVIpcPing,
      FFXIVIpcPingHandler, FFXIVIpcPlayerClassInfo, FFXIVIpcPlayerSetup, FFXIVIpcPlayerSpawn,
      FFXIVIpcPlayerStats, FFXIVIpcSocialList, PlayerEntry, PlayerInfo,
    },
    zone_ipc_def::{ActorControlType, ClientZoneIpcType, ServerZoneIpcType},
  },
};

const ZONE_ID: u16 = 129;

fn create_zone_packet_segments(
  segments: Vec<IpcResponse>,
  source_actor: u32,
  target_actor: u32,
) -> Vec<Vec<u8>> {
  segments
    .iter()
    .map(|lobby_response| {
      let mut lobby_result = lobby_response.ipc_header.to_bytes().unwrap();
      lobby_result.extend(lobby_response.segment.clone());

      let response_segment = create_packet_segment(3, lobby_result, source_actor, target_actor);

      response_segment.to_bytes().unwrap()
    })
    .collect()
}

pub async fn process(
  socket: &mut WriteHalf<TcpStream>,
  packet: FFXIVARRPacketSegmentRaw,
  player_info: &mut PlayerInfo,
) {
  let opcode = u16::from_le_bytes(packet.data[0x02..0x04].try_into().unwrap());
  let ipc_handler = ClientZoneIpcType::try_from(opcode).unwrap_or(ClientZoneIpcType::None);

  // TODO: Move out of match case to function handlers
  match ipc_handler {
    ClientZoneIpcType::InitHandler => {
      println!("== zone in ==");

      let ipc_init = FFXIVIpcInit {
        char_id: player_info.id,
        ..Default::default()
      };
      let init_packet = create_zone_packet_segments(
        vec![IpcResponse {
          ipc_header: get_ipc_header(ServerZoneIpcType::Init.into()),
          segment: ipc_init.to_bytes().unwrap(),
        }],
        player_info.id,
        player_info.id,
      );
      send_ipc_packet(socket, init_packet).await;

      let ipc_control_self = FFXIVIpcActorControlSelf {
        category: ActorControlType::SetCharaGearParamUI.into(),
        param1: 1,
        param2: 1,
        ..Default::default()
      };
      let ipc_control_self_packet = create_zone_packet_segments(
        vec![IpcResponse {
          ipc_header: get_ipc_header(ServerZoneIpcType::ActorControlSelf.into()),
          segment: ipc_control_self.to_bytes().unwrap(),
        }],
        player_info.id,
        player_info.id,
      );
      send_ipc_packet(socket, ipc_control_self_packet).await;

      let stats = FFXIVIpcPlayerStats {
        hp: 100,
        mp: 100,
        strength: 1,
        ..Default::default()
      };
      let stats_packet = create_zone_packet_segments(
        vec![IpcResponse {
          ipc_header: get_ipc_header(ServerZoneIpcType::PlayerStats.into()),
          segment: stats.to_bytes().unwrap(),
        }],
        player_info.id,
        player_info.id,
      );
      send_ipc_packet(socket, stats_packet).await;

      let mut player_name = "Final Fantasy".as_bytes().to_vec();
      player_name.resize(32, 0);
      let player_setup = FFXIVIpcPlayerSetup {
        content_id: CONTENT_ID,
        char_id: player_info.id,
        max_level: 90,
        name: player_name,
        expansion: 5,
        levels: vec![90; CLASSJOB_SLOTS.into()].try_into().unwrap(),
        exp: vec![10000; CLASSJOB_SLOTS.into()].try_into().unwrap(),
        ..Default::default()
      };
      let player_setup_packet = create_zone_packet_segments(
        vec![IpcResponse {
          ipc_header: get_ipc_header(ServerZoneIpcType::PlayerSetup.into()),
          segment: player_setup.to_bytes().unwrap(),
        }],
        player_info.id,
        player_info.id,
      );
      send_ipc_packet(socket, player_setup_packet).await;

      let player_class_info = FFXIVIpcPlayerClassInfo {
        class_id: 35,
        unknown: 1,
        synced_level: 90,
        class_level: 90,
        ..Default::default()
      };
      let player_class_info_packet = create_zone_packet_segments(
        vec![IpcResponse {
          ipc_header: get_ipc_header(ServerZoneIpcType::PlayerClassInfo.into()),
          segment: player_class_info.to_bytes().unwrap(),
        }],
        player_info.id,
        player_info.id,
      );
      send_ipc_packet(socket, player_class_info_packet).await;

      // send item level??

      let player_init_zone = FFXIVIpcInitZone {
        zone_id: ZONE_ID,
        weather_id: 1,
        bitmask: 0x10,
        pos: FFXIVARRPosition {
          x: 0.0,
          y: 0.0,
          z: 0.0,
        },
        server_id: 69,
        ..Default::default()
      };
      let player_init_zone_packet = create_zone_packet_segments(
        vec![IpcResponse {
          ipc_header: get_ipc_header(ServerZoneIpcType::InitZone.into()),
          segment: player_init_zone.to_bytes().unwrap(),
        }],
        player_info.id,
        player_info.id,
      );
      send_ipc_packet(socket, player_init_zone_packet).await;
    }
    ClientZoneIpcType::FinishLoadingHandler => {
      let mut player_name = "Final Fantasy".as_bytes().to_vec();
      player_name.resize(32, 0);
      let mut player_spawn = FFXIVIpcPlayerSpawn {
        name: player_name.try_into().unwrap(),
        class_job: 35,
        current_world_id: 69,
        home_world_id: 69,
        hp_curr: 100,
        hp_max: 100,
        mp_curr: 100,
        mp_max: 100,
        level: 90,
        gm_rank: 0,
        pose: 0,
        look: [0; 26],
        pos: FFXIVARRPosition {
          x: -82.0,
          y: 50.0,
          z: 0.0,
        },
        rotation: 0,
        model_type: 0x01,
        owner_id: 0xE0000000,
        u22: 0xE0000000,
        spawn_index: 1,
        state: 1,
        // current_mount: 290,
        display_flags: 294912, // flight
        ..Default::default()
      };
      player_spawn.models[0] = 1073008762;
      player_spawn.models[1] = 1225130480;
      player_spawn.models[2] = 1224802952;
      player_spawn.models[3] = 1248591897;
      player_spawn.models[4] = 205775;
      player_spawn.models[5] = 65690;
      player_spawn.models[6] = 65589;
      player_spawn.models[7] = 65690;
      player_spawn.models[8] = 65589;
      player_spawn.models[9] = 65690;
      let player_spawn_packet = create_zone_packet_segments(
        vec![IpcResponse {
          ipc_header: get_ipc_header(ServerZoneIpcType::PlayerSpawn.into()),
          segment: player_spawn.to_bytes().unwrap(),
        }],
        player_info.id,
        player_info.id,
      );

      send_ipc_packet(socket, player_spawn_packet).await;
    }
    ClientZoneIpcType::PingHandler => {
      println!("ping");

      let (_, ping_request_packet) = FFXIVIpcPingHandler::from_bytes((&packet.data, 0)).unwrap();

      let ping = FFXIVIpcPing {
        time_in_milliseconds: u64::from(ping_request_packet.timestamp) + 0x000014D00000000u64,
        unknown_8: [0; 0x38].to_vec(),
      };
      let ping_packet = create_zone_packet_segments(
        vec![IpcResponse {
          ipc_header: get_ipc_header(ServerZoneIpcType::Ping.into()),
          segment: ping.to_bytes().unwrap(),
        }],
        player_info.id,
        player_info.id,
      );

      send_ipc_packet(socket, ping_packet).await;
    }
    ClientZoneIpcType::SocialListHandler => {
      let request_type = packet.data[0x1A];
      let count = packet.data[0x1B];
      println!("social list handler type: {:?}", request_type);

      match request_type {
        1 => {
          // party list
          let mut name = "Final Fantasy".as_bytes().to_vec();
          name.resize(20, 0);
          let mut party_entries = vec![PlayerEntry {
            class_job: 0,
            content_id: CONTENT_ID,
            level: 90,
            zone_id: ZONE_ID,
            zone_id1: 0x0100,
            name: name.try_into().unwrap(),
            online_status_mask: 0,
            ..Default::default()
          }];
          party_entries[0].bytes[2] = 0;
          party_entries[0].bytes[3] = 0x80;
          party_entries[0].bytes[4] = 0x02;
          party_entries[0].bytes[6] = 0x3B;
          party_entries[0].bytes[11] = 0x10;

          party_entries[0].bytes1[0] = 2; // gc icon
          party_entries[0].bytes1[1] = 1; // language (1 = english)
          party_entries[0].bytes1[2] = 2; // user settings language

          party_entries.resize(10, PlayerEntry::default());
          let party_list = FFXIVIpcSocialList {
            request_type,
            sequence: count,
            entries: party_entries,
            ..Default::default()
          };
          let party_list_packet = create_zone_packet_segments(
            vec![IpcResponse {
              ipc_header: get_ipc_header(ServerZoneIpcType::SocialList.into()),
              segment: party_list.to_bytes().unwrap(),
            }],
            0,
            player_info.id,
          );

          send_ipc_packet(socket, party_list_packet).await;
        }
        2 => {
          // friend list
          let mut friends_list_entries = vec![];
          friends_list_entries.resize(10, PlayerEntry::default());
          let friends_list = FFXIVIpcSocialList {
            request_type,
            sequence: count,
            entries: friends_list_entries,
            ..Default::default()
          };
          let friends_list_packet = create_zone_packet_segments(
            vec![IpcResponse {
              ipc_header: get_ipc_header(ServerZoneIpcType::SocialList.into()),
              segment: friends_list.to_bytes().unwrap(),
            }],
            player_info.id,
            player_info.id,
          );

          send_ipc_packet(socket, friends_list_packet).await;
        }
        _ => {
          println!("unhandled social list")
        }
      }
    }
    ClientZoneIpcType::BlackListHandler => {
      println!("blacklist");

      let count = packet.data[0x11];
      let blacklist = FFXIVIpcBlackList {
        sequence: count.into(),
        entry: vec![[0; 40]; 20],
        ..Default::default()
      };
      let blacklist_packet = create_zone_packet_segments(
        vec![IpcResponse {
          ipc_header: get_ipc_header(ServerZoneIpcType::BlackList.into()),
          segment: blacklist.to_bytes().unwrap(),
        }],
        player_info.id,
        player_info.id,
      );

      send_ipc_packet(socket, blacklist_packet).await
    }
    ClientZoneIpcType::FcInfoReqHandler => {
      println!("fc handler");
    }
    _ => {
      println!("[Warning]: unhandled opcode {:?}", opcode);
    }
  };
}
