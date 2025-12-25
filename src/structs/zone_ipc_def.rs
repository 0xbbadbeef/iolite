use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ServerZoneIpcType {
  Ping = 0x30d,     // 7.40h1
  Init = 0x278,     // 7.40h1
  InitZone = 0x12e, // 7.40h1

  PlayerStats = 0x2bf,     // 7.40h1
  PlayerSetup = 0x36f,     // 7.40h1
  PlayerClassInfo = 0x379, // 7.40h1
  PlayerSpawn = 0x18b,     // 7.40h1

  ActorControlSelf = 0x38a, // 7.40h1
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ClientZoneIpcType {
  None,
  PingHandler = 0x2d6,          // 7.40h1
  InitHandler = 0x164,          // 7.40h1
  FinishLoadingHandler = 0x0c9, // 7.40h1
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ActorControlType {
  SetCharaGearParamUI = 0x260,
}
