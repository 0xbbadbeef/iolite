use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ServerZoneIpcType {
  Ping = 0x1a0,     // 7.40h2
  Init = 0x250,     // 7.40h2
  InitZone = 0x242, // 7.40h2

  PlayerStats = 0x1e1,     // 7.40h2
  PlayerSetup = 0x256,     // 7.40h2
  PlayerClassInfo = 0x0a0, // 7.40h2
  PlayerSpawn = 0x0ca,     // 7.40h2

  ActorControlSelf = 0x347, // 7.40h2
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ClientZoneIpcType {
  None,
  PingHandler = 0x142,          // 7.40h2
  InitHandler = 0x312,          // 7.40h2
  FinishLoadingHandler = 0x324, // 7.40h2
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum ActorControlType {
  SetCharaGearParamUI = 0x260,
}
