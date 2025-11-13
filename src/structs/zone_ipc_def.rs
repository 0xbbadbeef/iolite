use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ServerZoneIpcType {
  Ping = 0x3E2, // 7.38
  Init = 0x277, // 7.38
  InitZone = 0x2B1, // 7.38

  PlayerStats = 0x2C4,   // 7.38
  PlayerSetup = 0x39A,   // 7.38
  PlayerClassInfo = 0x204, // 7.38
  PlayerSpawn = 0x270,   // 7.38

  ActorControlSelf = 0x254, // 7.38
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ClientZoneIpcType {
  None,
  PingHandler = 0x30A, // 7.38
  InitHandler = 0x157, // 7.38
  FinishLoadingHandler = 0x3A5, // 7.38
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ActorControlType {
  SetCharaGearParamUI = 0x260,
}
