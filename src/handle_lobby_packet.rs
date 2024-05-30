use core::slice::{self};
use std::{
  mem,
  time::{SystemTime, UNIX_EPOCH},
};

use deku::DekuContainerWrite;
use serde_json::Value;
use tokio::{
  fs::{self},
  net::TcpStream,
};

use crate::{
  handle_packets::{blowfish_encode, KEY_BYTES},
  packet_transfer::send_ipc_packet,
  structs::{
    common::{FFXIVARRPacketSegmentRaw, FFXIVARRSegmentHeader, CURRENT_EXPANSION_ID},
    lobby_structs::{
      ClientLobbyIpcType, FFXIVCharDetails, FFXIVIpcCharList, FFXIVIpcHeader, FFXIVIpcRetainerList,
      FFXIVIpcServerList, FFXIVIpcServiceIdInfo, FFXIVServer, FFXIVServiceAccount,
      ServerLobbyIpcType,
    },
  },
};

const WORLD_ID: u16 = 21;
const WORLD_NAME: &str = "WAGYU";

struct LobbyResponse {
  pub ipc_header: FFXIVIpcHeader,
  pub segment: Vec<u8>,
}

fn create_lobby_packet_segments(
  encryption_key: &[u8],
  lobby_responses: &[LobbyResponse],
) -> Vec<Vec<u8>> {
  lobby_responses
    .iter()
    .map(|lobby_response| {
      let mut lobby_result = lobby_response.ipc_header.to_bytes().unwrap();
      lobby_result.extend(lobby_response.segment.clone());
      let lobby_response_encoded = unsafe {
        let encoded = blowfish_encode(
          encryption_key.as_ptr(),
          KEY_BYTES,
          lobby_result.as_ptr(),
          lobby_result.len().try_into().unwrap(),
        );

        slice::from_raw_parts(encoded, lobby_result.len()).to_vec()
      };

      let seg_size: u32 = mem::size_of::<FFXIVARRSegmentHeader>().try_into().unwrap();
      let lobby_size: u32 = lobby_response_encoded.len().try_into().unwrap();
      let response_segment = FFXIVARRPacketSegmentRaw {
        seg_hdr: FFXIVARRSegmentHeader {
          size: seg_size + lobby_size,
          segment_type: 3,
          source_actor: 3758111786,
          target_actor: 3758111786,
          ..Default::default()
        },
        data: lobby_response_encoded,
      };

      response_segment.to_bytes().unwrap()
    })
    .collect()
}

fn get_ipc_header(ipc_type: ServerLobbyIpcType) -> FFXIVIpcHeader {
  FFXIVIpcHeader {
    timestamp: SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards")
      .as_secs()
      .try_into()
      .unwrap(),
    ipc_type: ipc_type.try_into().unwrap(),
    ..Default::default()
  }
}

fn get_service_account_list(packet_segment: FFXIVARRPacketSegmentRaw) -> LobbyResponse {
  let session_id = packet_segment.data[0x22];
  println!("session_id = {:?}", session_id);

  let ipc_header = get_ipc_header(ServerLobbyIpcType::LobbyServiceAccountList);

  let mut name = "FINAL FANTASY XIV".as_bytes().to_vec();
  name.resize(0x44, 0);
  let service_account = FFXIVServiceAccount {
    id: 0x002E4A2B,
    name,
    ..Default::default()
  };

  let service_id_info_packet = FFXIVIpcServiceIdInfo {
    num_service_accounts: 1,
    u1: 3,
    u2: 0x99,
    ..Default::default()
  };

  let mut appended_service_accounts = service_id_info_packet.to_bytes().unwrap();
  appended_service_accounts.extend(service_account.to_bytes().unwrap());
  for _ in 0..7 {
    let empty_name = vec![0; 0x44];
    appended_service_accounts.extend(
      FFXIVServiceAccount {
        name: empty_name,
        ..Default::default()
      }
      .to_bytes()
      .unwrap(),
    );
  }

  LobbyResponse {
    ipc_header,
    segment: appended_service_accounts,
  }
}

fn get_server_list() -> LobbyResponse {
  let ipc_header = get_ipc_header(ServerLobbyIpcType::LobbyServerList);

  let mut server_list = FFXIVIpcServerList {
    seq: 1,
    offset: 0,
    num_servers: 1,
    final_unknown: 1,
    ..Default::default()
  }
  .to_bytes()
  .unwrap();

  let mut name = WORLD_NAME.as_bytes().to_vec();
  name.resize(0x40, 0);
  let server = FFXIVServer {
    id: WORLD_ID,
    index: 0,
    name,
    ..Default::default()
  };
  server_list.extend(server.to_bytes().unwrap());

  for _ in 0..5 {
    let empty_server_name = vec![0; 0x40];
    server_list.extend(
      FFXIVServer {
        name: empty_server_name,
        ..Default::default()
      }
      .to_bytes()
      .unwrap(),
    )
  }

  LobbyResponse {
    ipc_header,
    segment: server_list,
  }
}

fn get_retainers() -> LobbyResponse {
  let ipc_header = get_ipc_header(ServerLobbyIpcType::LobbyRetainerList);

  let mut empty_padding = vec![0; 0x210];
  empty_padding[8] = 1;
  let get_retainers = FFXIVIpcRetainerList {
    padding: empty_padding,
  };

  LobbyResponse {
    ipc_header,
    segment: get_retainers.to_bytes().unwrap(),
  }
}

const SIZED_UNKNOWN1: [u8; 16] = [0u8; 16];
const SIZED_UNKNOWN2: [u8; 20] = [0u8; 20];
fn get_empty_char() -> Vec<u8> {
  // TODO: Figure out why deku isn't sizing with Count attrib :(
  let sized_character_name = vec![0u8; 32];
  let sized_server_name = vec![0u8; 32];
  let sized_server_name1 = vec![0u8; 32];
  let sized_character_detail_json = vec![0u8; 1024];

  FFXIVCharDetails {
    unknown1: SIZED_UNKNOWN1.to_vec(),
    index: 1,
    character_name: sized_character_name,
    character_server_name: sized_server_name,
    character_server_name1: sized_server_name1,
    character_detail_json: sized_character_detail_json,
    unknown2: SIZED_UNKNOWN2.to_vec(),
    ..Default::default()
  }
  .to_bytes()
  .unwrap()
}

async fn get_char_list(
  socket: &mut TcpStream,
  packet_segment: FFXIVARRPacketSegmentRaw,
  encryption_key: &[u8],
) {
  let sequence = u64::from_ne_bytes(packet_segment.data[0x10..0x10 + 8].try_into().unwrap());

  println!("seq: {}", sequence);

  for counter in 0..4 {
    let ipc_header = get_ipc_header(ServerLobbyIpcType::LobbyCharList);
    let sized_unknown5 = vec![0u32; 7];
    let mut char_list = FFXIVIpcCharList {
      seq: sequence,
      num_in_packet: 2,
      counter: counter * 4,
      unknown5: sized_unknown5,
      ..Default::default()
    };

    if counter == 3 {
      char_list.entitled_expansion = CURRENT_EXPANSION_ID;
      char_list.max_char_on_world = 20;
      char_list.unknown8 = 8;
      char_list.veteran_rank = 12;
      char_list.counter = (counter * 4) + 1;
      char_list.unknown4 = 128;
    }

    let mut char_list_bytes = char_list.to_bytes().unwrap();
    if counter == 0 {
      let character = fs::read_to_string("character.json").await.unwrap();
      let character_detail_json: Value =
        serde_json::from_str(character.as_str()).expect("Character JSON was corrupt!");
      let mut character_detail_bytes = character_detail_json.to_string().as_bytes().to_vec();
      character_detail_bytes.resize(1024, 0);
      let mut character_name = "Final Fantasy".as_bytes().to_vec();
      character_name.resize(32, 0);
      let mut world_name = WORLD_NAME.as_bytes().to_vec();
      world_name.resize(32, 0);
      let char_details = FFXIVCharDetails {
        unique_id: 0,
        content_id: 11111111111111111,
        server_id: WORLD_ID,
        server_id1: WORLD_ID,
        index: 0,
        unknown1: SIZED_UNKNOWN1.to_vec(),
        character_name,
        character_server_name: world_name.clone(),
        character_server_name1: world_name.clone(),
        character_detail_json: character_detail_bytes,
        unknown2: SIZED_UNKNOWN2.to_vec(),
        ..Default::default()
      }
      .to_bytes()
      .unwrap();
      char_list_bytes.extend(char_details);
    } else {
      char_list_bytes.extend(get_empty_char());
    }
    char_list_bytes.extend(get_empty_char());

    let segments = create_lobby_packet_segments(
      encryption_key,
      &[LobbyResponse {
        ipc_header,
        segment: char_list_bytes,
      }],
    );

    send_ipc_packet(socket, segments).await;
  }
}

pub async fn handle_lobby_packet(
  socket: &mut TcpStream,
  encryption_key: &[u8],
  packet_segment: FFXIVARRPacketSegmentRaw,
) {
  let op_code = u16::from(packet_segment.data[2]);
  println!("Opcode: {}", op_code);

  match op_code.try_into().unwrap() {
    ClientLobbyIpcType::ClientVersionInfo => {
      let segments = create_lobby_packet_segments(
        encryption_key,
        &[get_service_account_list(packet_segment)],
      );
      send_ipc_packet(socket, segments).await;
    }
    ClientLobbyIpcType::ReqCharList => {
      let mut segment_buffer = vec![];
      segment_buffer.push(get_server_list());
      segment_buffer.push(get_retainers());
      let segments = create_lobby_packet_segments(encryption_key, &segment_buffer);
      send_ipc_packet(socket, segments).await;
      segment_buffer.clear();

      get_char_list(socket, packet_segment, encryption_key).await;
    }
    ClientLobbyIpcType::ReqEnterWorld => {
      // let lookup_id = u64::from_ne_bytes(packet_segment.data[0x18..(0x18 + 8)].try_into().unwrap());
    }
    _ => {
      panic!("Unknown opcode!")
    }
  }
}
