use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ServerZoneIpcType {
  Ping = 0x1D9,
  Init = 0x12A,
  InitZone = 0x02D1,

  PlayerStats = 0x034F,
  PlayerSetup = 0x035F,
  PlayerClassInfo = 0x238,
  PlayerSpawn = 0x039C,

  SocialList = 0x1F2,
  BlackList = 0x38A,

  ActorControlSelf = 0x025D,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ClientZoneIpcType {
  None,
  PingHandler = 0x2AE,
  InitHandler = 0x1CE,
  FinishLoadingHandler = 0x12A,
  SocialListHandler = 0x10B,
  BlackListHandler = 0x284,
  FcInfoReqHandler = 0x33B,
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ActorControlType {
  SetCharaGearParamUI = 0x260,
}
