use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ServerZoneIpcType {
  Ping = 0x065, // 7.3h1
  Init = 0x067, // 7.3h1
  InitZone = 0x36C, // 7.3h1

  PlayerStats = 0x0E6,   // 7.3h1
  PlayerSetup = 0x359,   // 7.3h1
  PlayerClassInfo = 0x22D, // 7.3h1
  PlayerSpawn = 0x37A,   // 7.3h1

  ActorControlSelf = 0x1F4, // 7.3h1
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ClientZoneIpcType {
  None,
  PingHandler = 0x0F1, // 7.3h1
  InitHandler = 0x152, // 7.3h1
  FinishLoadingHandler = 0x336, // 7.3h1
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ActorControlType {
  SetCharaGearParamUI = 0x260,
}
