use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ServerZoneIpcType {
  Ping = 0x19e, // 7.16
  Init = 0x1EF, // 7.18h
  InitZone = 0x311, // 7.18h

  PlayerStats = 0x1FA,   // 7.18h
  PlayerSetup = 0x006B,   // 7.18h
  PlayerClassInfo = 0x006A, // 7.18h
  PlayerSpawn = 0x01AB,   // 7.18h

  ActorControlSelf = 0x018C, // 7.18h
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ClientZoneIpcType {
  None,
  PingHandler = 0x2B5, // 7.18h
  InitHandler = 0x2ED, // 7.18h
  FinishLoadingHandler = 0x397, // 7.18h
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ActorControlType {
  SetCharaGearParamUI = 0x260,
}
