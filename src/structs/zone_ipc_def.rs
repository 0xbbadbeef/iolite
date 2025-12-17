use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ServerZoneIpcType {
  Ping = 0x1df,     // 7.40
  Init = 0x184,     // 7.40
  InitZone = 0x175, // 7.40

  PlayerStats = 0x160,     // 7.40
  PlayerSetup = 0x90,     // 7.40
  PlayerClassInfo = 0xac, // 7.40
  PlayerSpawn = 0x107,     // 7.40

  ActorControlSelf = 0x1d8, // 7.40
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ClientZoneIpcType {
  None,
  PingHandler = 0x311,          // 7.40
  InitHandler = 0x2ec,          // 7.40
  FinishLoadingHandler = 0x1e5, // 7.40
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ActorControlType {
  SetCharaGearParamUI = 0x260,
}
