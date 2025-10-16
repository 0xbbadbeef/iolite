use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ServerZoneIpcType {
  Ping = 0x078, // 7.35h1
  Init = 0x30E, // 7.35h1
  InitZone = 0x3A3, // 7.35h1

  PlayerStats = 0x3AD,   // 7.35h1
  PlayerSetup = 0x26F,   // 7.35h1
  PlayerClassInfo = 0x2CC, // 7.35h1
  PlayerSpawn = 0x0E3,   // 7.35h1

  ActorControlSelf = 0x3BD, // 7.35h1
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ClientZoneIpcType {
  None,
  PingHandler = 0x3E0, // 7.35h1
  InitHandler = 0x1AB, // 7.35h1
  FinishLoadingHandler = 0x197, // 7.35h1
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ActorControlType {
  SetCharaGearParamUI = 0x260,
}
