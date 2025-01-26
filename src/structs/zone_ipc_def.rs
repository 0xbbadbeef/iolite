use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ServerZoneIpcType {
  Ping = 0x19e, // 7.16
  Init = 0x2bf, // ?? 7.16
  InitZone = 0x033E, // 7.16

  PlayerStats = 0x008F,   // 7.16
  PlayerSetup = 0x025D,   // 7.16
  PlayerClassInfo = 0x02E9, // 7.16
  PlayerSpawn = 0x02AC,   // 7.16

  SocialList = 0x1F2,
  BlackList = 0x38A,

  ActorControlSelf = 0x02D7, // 7.16
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ClientZoneIpcType {
  None,
  PingHandler = 0x33A, // 7.16
  InitHandler = 0x219, // 7.16
  FinishLoadingHandler = 0x23E, // 7.16
  SocialListHandler = 0x10B,
  BlackListHandler = 0x284,
  FcInfoReqHandler = 0x33B,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ActorControlType {
  SetCharaGearParamUI = 0x260,
}
