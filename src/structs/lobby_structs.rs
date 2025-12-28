use std::mem::transmute;

use deku::prelude::*;

#[repr(u16)]
pub enum ServerLobbyIpcType {
  LobbyError = 0x0002,
  LobbyServiceAccountList = 0x000C,
  LobbyCharList = 0x000D,
  LobbyCharCreate = 0x000E,
  LobbyEnterWorld = 0x000F,
  LobbyServerList = 0x0015,
  LobbyRetainerList = 0x0017,
}

impl TryInto<u16> for ServerLobbyIpcType {
  type Error = ();
  fn try_into(self) -> Result<u16, Self::Error> {
    Ok(unsafe { transmute::<ServerLobbyIpcType, u16>(self) })
  }
}

#[repr(u16)]
pub enum ClientLobbyIpcType {
  ReqCharList = 0x0003,
  ReqEnterWorld = 0x0004,
  ClientVersionInfo = 0x0005,
  ReqCharDelete = 0x000A,
  ReqCharCreate = 0x000B,
}

impl TryFrom<u16> for ClientLobbyIpcType {
  type Error = ();
  fn try_from(value: u16) -> Result<Self, Self::Error> {
    unsafe { transmute(value) }
  }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcHeader {
  pub reserved: u16,
  pub ipc_type: u16,
  pub padding: u16,
  pub server_id: u16,
  pub timestamp: u32,
  pub padding1: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVServiceAccount {
  pub id: u32,
  pub unknown: u32,
  pub index: u32,
  #[deku(count = "0x44")]
  pub name: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcServiceIdInfo {
  pub seq: u64,
  pub padding: u8,
  pub num_service_accounts: u8,
  pub u1: u8,
  pub u2: u8,
  pub padding1: u32,
  // FFXIVServiceAccount[8] goes here, don't think deku suppots struct vecs? maybe?
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVServer {
  pub id: u16,
  pub index: u16,
  pub flags: u32,
  pub padding1: u32,
  pub icon: u32,
  pub padding2: u32,
  #[deku(count = "0x40")]
  pub name: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcServerList {
  pub seq: u64,
  pub final_unknown: u16,
  pub offset: u16,
  pub num_servers: u32,
  pub padding: u32,
  pub padding1: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcRetainerList {
  #[deku(count = "0x210")]
  pub padding: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVCharDetails {
  pub unique_id: u32,
  pub padding: u32,
  pub content_id: u64,
  pub index: u32,
  pub padding2: u32,
  pub server_id: u16,
  pub server_id1: u16,
  #[deku(count = "16")]
  pub unknown1: Vec<u8>,
  #[deku(count = "32")]
  pub character_name: Vec<u8>,
  #[deku(count = "32")]
  pub character_server_name: Vec<u8>,
  #[deku(count = "32")]
  pub character_server_name1: Vec<u8>,
  #[deku(count = "1024")]
  pub character_detail_json: Vec<u8>,
  #[deku(count = "20")]
  pub unknown2: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcCharList {
  pub seq: u64,
  pub counter: u8,
  pub num_in_packet: u8,
  pub padding: u16,
  pub unknown1: u8,
  pub unknown2: u8,
  pub unknown3: u8,
  pub unknown4: u8,
  #[deku(count = "7")]
  pub unknown5: Vec<u32>,
  pub unknown6: u8,
  pub veteran_rank: u8,
  pub unknown7: u8,
  pub padding1: u8,
  pub days_subscribed: u32,
  pub remaining_days: u32,
  pub days_to_next_rank: u32,
  pub max_char_on_world: u16,
  pub unknown8: u16,
  pub entitled_expansion: u32,
  pub padding2: u32,
  pub padding3: u32,
  pub padding4: u32,
  // FFXIVCharDetails[2]
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcEnterWorld {
  pub seq: u64,
  pub char_id: u32,
  pub padding: u32,
  pub content_id: u64,
  pub padding2: u32,
  #[deku(count = "66")]
  pub session_id: Vec<u8>,
  pub port: u16,
  #[deku(count = "48")]
  pub host: Vec<u8>,
  pub padding3: u64,
  pub padding4: u64,
}
