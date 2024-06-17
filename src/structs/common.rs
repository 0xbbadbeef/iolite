use deku::prelude::*;
use libc::c_float;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::mem::transmute;

pub const CURRENT_EXPANSION_ID: u32 = 4;
pub const CONTENT_ID: u64 = 11111111111111111;

pub const CLASSJOB_TOTAL: u8 = 40;
pub const CLASSJOB_SLOTS: u8 = 32;

pub enum CompressionType {
  NoCompression = 0,
  Zlib = 1,
  Oodle = 2,
}

#[repr(u16)]
pub enum FFXIVARRSegmentType {
  SessionInit = 1,
  IPC = 3,
  KeepAlive = 7,
  SegmentTypeResponse = 8,
  EncryptionInit = 9,
}

impl TryFrom<u16> for FFXIVARRSegmentType {
  type Error = ();
  fn try_from(value: u16) -> Result<Self, Self::Error> {
    unsafe { transmute(value) }
  }
}

#[repr(u16)]
pub enum FFXIVConnectionType {
  None = 0,
  Zone = 1,
  Chat = 2,
  Lobby = 3,
}

impl TryFrom<u16> for FFXIVConnectionType {
  type Error = ();
  fn try_from(value: u16) -> Result<Self, Self::Error> {
    unsafe { transmute(value) }
  }
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
#[deku(
  endian = "little",
  ctx = "_endian: deku::ctx::Endian",
  ctx_default = "deku::ctx::Endian::Little"
)]
pub struct FFXIVARRPosition {
  pub x: c_float,
  pub y: c_float,
  pub z: c_float,
}

// https://github.com/SapphireServer/Sapphire/blob/master/src/common/Network/CommonNetwork.h#L51
#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVARRPacketHeader {
  pub unknown_0: u64,
  pub unknown_8: u64,
  pub timestamp: u64,
  pub size: u32,
  pub connection_type: u16,
  pub count: u16,
  pub unknown_20: u8,
  pub compression_type: u8,
  pub unknown_22: u16,
  pub oodle_decompressed_size: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(
  endian = "little",
  ctx = "_endian: deku::ctx::Endian",
  ctx_default = "deku::ctx::Endian::Little"
)]
pub struct FFXIVARRSegmentHeader {
  pub size: u32,
  pub source_actor: u32,
  pub target_actor: u32,
  pub segment_type: u16,
  pub padding: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVARRPacketSegmentRaw {
  pub seg_hdr: FFXIVARRSegmentHeader,
  #[deku(count = "0")]
  pub data: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum GearModelSlot {
  // pub model_invalid: -1,
  ModelHead = 0,
  ModelBody = 1,
  ModelHands = 2,
  ModelLegs = 3,
  ModelFeet = 4,
  ModelEar = 5,
  ModelNeck = 6,
  ModelWrist = 7,
  ModelRing1 = 8,
  ModelRing2 = 9,
}
