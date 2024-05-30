use deku::prelude::*;
use std::mem::transmute;

pub const CURRENT_EXPANSION_ID: u32 = 4;

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
  //SEGMENTTYPE_RESPONSE = 8,
  EncryptionInit = 9,
}

impl TryFrom<u16> for FFXIVARRSegmentType {
  type Error = ();
  fn try_from(value: u16) -> Result<Self, Self::Error> {
    unsafe { transmute(value) }
  }
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
#[deku(endian = "little", ctx = "_endian: deku::ctx::Endian", ctx_default = "deku::ctx::Endian::Little")]
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
