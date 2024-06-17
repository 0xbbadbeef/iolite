use deku::prelude::*;
use derivative::Derivative;

use super::common::FFXIVARRPosition;

#[derive(Default, Debug)]
pub struct PlayerInfo {
  pub id: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcInit {
  pub unknown: u64,
  pub char_id: u32,
  pub unknown1: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Derivative)]
#[deku(endian = "little")]
#[derivative(Default)]
pub struct FFXIVIpcPlayerStats {
  pub strength: u32,
  pub dexterity: u32,
  pub vitality: u32,
  pub intelligence: u32,
  pub mind: u32,
  pub piety: u32,
  pub hp: u32,
  pub mp: u32,
  pub tp: u32,
  pub gp: u32,
  pub cp: u32,
  pub delay: u32,
  pub tenacity: u32,
  pub attack_power: u32,
  pub defense: u32,
  pub direct_hit_rate: u32,
  pub evasion: u32,
  pub magice_defense: u32,
  pub critical_hit: u32,
  pub attack_magic_potency: u32,
  pub healing_magic_potency: u32,
  pub elemental_bonus: u32,
  pub determination: u32,
  pub skill_speed: u32,
  pub spell_speed: u32,
  pub haste: u32,
  pub craftmanship: u32,
  pub control: u32,
  pub gathering: u32,
  pub perception: u32,
  #[derivative(Default(value = "vec![0; 26]"))]
  #[deku(count = "26")]
  pub unknown: Vec<u32>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Derivative)]
#[deku(endian = "little")]
#[derivative(Default)]
pub struct FFXIVIpcPlayerSetup {
  pub content_id: u64,
  pub crest: u64,
  pub unknown10: u64,
  pub char_id: u32,
  pub rested_exp: u32,
  pub companion_current_exp: u32,
  pub unknown1c: u32,
  pub fish_caught: u32,
  pub use_bait_catalog_id: u32,
  pub unknown28: u32,
  pub unknown_pvp2c: u16,
  pub unknown2e: u16,
  pub pvp_frontline_overall_campaigns: u32,
  pub unknown_timestamp34: u32,
  pub unknown_timestamp38: u32,
  pub unknown3c: u32,
  pub unknown40: u32,
  pub unknown44: u32,
  pub companion_time_passed: f32,
  pub unknown4c: u32,
  pub unknown50: u16,
  pub unknown_pvp52: [u16; 4],
  pub pvp_series_exp: u16,
  pub player_commendations: u16,
  pub unknown64: [u16; 8],
  pub pvp_rival_wings_total_matches: u16,
  pub pvp_rival_wings_total_victories: u16,
  pub pvp_rival_wings_weekly_matches: u16,
  pub pvp_rival_wings_weekly_victories: u16,
  pub max_level: u8,
  pub expansion: u8,
  pub unknown76: u8,
  pub unknown77: u8,
  pub unknown78: u8,
  pub race: u8,
  pub tribe: u8,
  pub gender: u8,
  pub current_job: u8,
  pub current_class: u8,
  pub deity: u8,
  pub nameday_month: u8,
  pub nameday_day: u8,
  pub city_state: u8,
  pub homepoint: u8,
  #[derivative(Default(value = "[0; 3]"))]
  pub unknown8d: [u8; 3],
  pub companion_rank: u8,
  pub companion_stars: u8,
  pub companion_sp: u8,
  pub companion_unk93: u8,
  pub companion_color: u8,
  pub companion_fav_feed: u8,
  pub fav_aetheryte_count: u8,
  #[derivative(Default(value = "[0; 5]"))]
  pub unknown97: [u8; 5],
  pub sightseeing21_to_80_unlock: u8,
  pub sightseeing_heavensward_unlock: u8,
  #[derivative(Default(value = "[0; 26]"))]
  pub unknown9e: [u8; 26],
  #[derivative(Default(value = "[0; 32]"))]
  pub exp: [u32; 32],
  pub pvp_total_exp: u32,
  pub unknown_pvp124: u32,
  pub pvp_exp: u32,
  #[derivative(Default(value = "[0; 3]"))]
  pub pvp_frontline_overall_ranks: [u32; 3],
  pub unknown138: u32,
  #[derivative(Default(value = "[0; 32]"))]
  pub levels: [u16; 32],
  #[deku(count = "218")]
  #[derivative(Default(value = "vec![0; 218]"))]
  pub unknown194: Vec<u8>,
  #[derivative(Default(value = "[0; 21]"))]
  pub companion_name: [u8; 21],
  pub companion_def_rank: u8,
  pub companion_att_rank: u8,
  pub companion_heal_rank: u8,
  #[deku(count = "33")]
  #[derivative(Default(value = "vec![0; 33]"))]
  pub mount_guide_mask: Vec<u8>,
  #[derivative(Default(value = "[0; 4]"))]
  pub ornament_mask: [u8; 4],
  #[derivative(Default(value = "[0; 23]"))]
  pub unknown281: [u8; 23],
  #[deku(count = "32")]
  #[derivative(Default(value = "vec![0; 32]"))]
  pub name: Vec<u8>,
  #[derivative(Default(value = "[0; 16]"))]
  pub unknown293: [u8; 16],
  pub unknown2a3: u8,
  #[deku(count = "64")]
  #[derivative(Default(value = "vec![0; 64]"))]
  pub unlock_bitmask: Vec<u8>,
  #[derivative(Default(value = "[0; 26]"))]
  pub aetheryte: [u8; 26],
  #[derivative(Default(value = "[0; 4]"))]
  pub favorite_aetheryte_ids: [u16; 4],
  pub free_aetheryte_id: u16,
  pub ps_plus_free_aetheryte_id: u16,
  #[deku(count = "480")]
  #[derivative(Default(value = "vec![0; 480]"))]
  pub discovery: Vec<u8>,
  #[deku(count = "36")]
  #[derivative(Default(value = "vec![0; 36]"))]
  pub howto: Vec<u8>,
  #[derivative(Default(value = "[0; 4]"))]
  pub unknown554: [u8; 4],
  #[deku(count = "60")]
  #[derivative(Default(value = "vec![0; 60]"))]
  pub minions: Vec<u8>,
  #[derivative(Default(value = "[0; 12]"))]
  pub chocobo_taxi_mask: [u8; 12],
  #[deku(count = "159")]
  #[derivative(Default(value = "vec![0; 159]"))]
  pub watched_cutscenes: Vec<u8>,
  #[derivative(Default(value = "[0; 12]"))]
  pub companion_barding_mask: [u8; 12],
  pub companion_equipped_head: u8,
  pub companion_equipped_body: u8,
  pub companion_equipped_legs: u8,
  #[deku(count = "287")]
  #[derivative(Default(value = "vec![0; 287]"))]
  pub unknown_mask: Vec<u8>,
  #[derivative(Default(value = "[0; 7]"))]
  pub pose: [u8; 7],
  #[derivative(Default(value = "[0; 3]"))]
  pub unknown6df: [u8; 3],
  #[derivative(Default(value = "[0; 13]"))]
  pub challenge_log_complete: [u8; 13],
  #[derivative(Default(value = "[0; 12]"))]
  pub secret_recipe_book_mask: [u8; 12],
  #[derivative(Default(value = "[0; 29]"))]
  pub unknown_mask6f7: [u8; 29],
  #[derivative(Default(value = "[0; 12]"))]
  pub relic_completion: [u8; 12],
  #[deku(count = "37")]
  #[derivative(Default(value = "vec![0; 37]"))]
  pub sightseeing_mask: Vec<u8>,
  #[deku(count = "102")]
  #[derivative(Default(value = "vec![0; 102]"))]
  pub hunting_mark_mask: Vec<u8>,
  #[deku(count = "45")]
  #[derivative(Default(value = "vec![0; 45]"))]
  pub triple_triad_cards: Vec<u8>,
  pub unknown895: u8,
  #[derivative(Default(value = "[0; 15]"))]
  pub unknown7d7: [u8; 15],
  pub unknown7d8: u8,
  #[deku(count = "49")]
  #[derivative(Default(value = "vec![0; 49]"))]
  pub unknown7e6: Vec<u8>,
  #[derivative(Default(value = "[0; 6]"))]
  pub regional_folklore_mask: [u8; 6],
  #[deku(count = "87")]
  #[derivative(Default(value = "vec![0; 87]"))]
  pub orchestrion_mask: Vec<u8>,
  #[derivative(Default(value = "[0; 3]"))]
  pub hall_of_novice_completion: [u8; 3],
  #[derivative(Default(value = "[0; 11]"))]
  pub anima_completion: [u8; 11],
  #[deku(count = "41")]
  #[derivative(Default(value = "vec![0; 41]"))]
  pub unknown85e: Vec<u8>,
  #[derivative(Default(value = "[0; 28]"))]
  pub unlocked_raids: [u8; 28],
  #[derivative(Default(value = "[0; 18]"))]
  pub unlocked_dungeons: [u8; 18],
  #[derivative(Default(value = "[0; 10]"))]
  pub unlocked_guildhests: [u8; 10],
  #[derivative(Default(value = "[0; 12]"))]
  pub unlocked_trials: [u8; 12],
  #[derivative(Default(value = "[0; 5]"))]
  pub unlocked_pvp: [u8; 5],
  #[derivative(Default(value = "[0; 28]"))]
  pub cleared_raids: [u8; 28],
  #[derivative(Default(value = "[0; 18]"))]
  pub cleared_dungeons: [u8; 18],
  #[derivative(Default(value = "[0; 10]"))]
  pub cleared_guildhests: [u8; 10],
  #[derivative(Default(value = "[0; 12]"))]
  pub cleared_trials: [u8; 12],
  #[derivative(Default(value = "[0; 5]"))]
  pub cleared_pvp: [u8; 5],
  #[derivative(Default(value = "[0; 15]"))]
  pub unknown948: [u8; 15],
}

#[derive(Debug, DekuRead, DekuWrite, Derivative)]
#[deku(endian = "little")]
#[derivative(Default)]
pub struct FFXIVIpcPlayerSpawn {
  pub title: u16,
  pub u1b: u16,
  pub current_world_id: u16,
  pub home_world_id: u16,

  pub gm_rank: u8,
  pub u3c: u8,
  pub u4: u8,
  pub online_status: u8,

  pub pose: u8,
  pub u5a: u8,
  pub u5b: u8,
  pub u5c: u8,

  pub target_id: u64,
  pub u6: u32,
  pub u7: u32,
  pub main_weapon_model: u64,
  pub sec_weapon_model: u64,
  pub craft_tool_model: u64,

  pub u14: u32,
  pub u15: u32,
  pub b_npc_base: u32,
  pub b_npc_name: u32,
  pub u18: u32,
  pub u19: u32,
  pub director_id: u32,
  pub owner_id: u32,
  pub u22: u32,
  pub hp_max: u32,
  pub hp_curr: u32,
  pub display_flags: u32,
  pub fate_id: u16,
  pub mp_curr: u16,
  pub mp_max: u16,
  pub unk: u16,
  pub model_chara: u16,
  pub rotation: u16,
  pub current_mount: u16,
  pub active_minion: u16,
  pub u23: u8,
  pub u24: u8,
  pub u25: u8,
  pub u26: u8,
  pub spawn_index: u8,
  pub state: u8,
  pub persistent_emote: u8,
  pub model_type: u8,
  pub subtype: u8,
  pub voice: u8,
  pub enemy_type: u8,
  pub unk27: u8,
  pub level: u8,
  pub class_job: u8,
  pub unk28: u8,
  pub unk29: u8,
  pub unk30: u8,
  pub mount_head: u8,
  pub mount_body: u8,
  pub mount_feet: u8,
  pub mount_color: u8,
  pub scale: u8,
  pub element_data: [u8; 6],
  #[deku(count = "30")]
  #[derivative(Default(value = "vec![StatusEffect::default(); 30]"))]
  pub effect: Vec<StatusEffect>,
  pub pos: FFXIVARRPosition,
  pub models: [u32; 10],
  pub unknown6_58: [u8; 10],
  pub name: [u8; 32],
  pub look: [u8; 26],
  pub fc_tag: [u8; 6],
  pub padding: [u8; 6],
}

#[derive(Debug, Clone, DekuRead, DekuWrite, Default)]
#[deku(
  endian = "little",
  ctx = "_endian: deku::ctx::Endian",
  ctx_default = "deku::ctx::Endian::Little"
)]
pub struct StatusEffect {
  effect_id: u16,
  param: u16,
  duration: f32,
  source_actor_id: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcPlayerClassInfo {
  pub class_id: u16,
  pub unknown: u8,
  pub is_specialist: u8,
  pub synced_level: u16,
  pub class_level: u16,
  pub role_actions: [u32; 10],
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcInitZone {
  pub server_id: u16,
  pub zone_id: u16,
  pub zone_index: u16,
  pub content_finder_condition_id: u16,
  pub layer_set_id: u32,
  pub layout_id: u32,
  pub weather_id: u32,
  pub bitmask: u8,
  pub bitmask1: u8,
  pub unknown5: u8,
  pub unknown8: u32,
  pub festival_id: u16,
  pub additional_festival_id: u16,
  pub unknown9: u32,
  pub unknown10: u32,
  pub unknown11: u32,
  pub unknown12: [u32; 4],
  pub unknown13: [u32; 3],
  pub pos: FFXIVARRPosition,
  pub unknown14: [u32; 4],
  pub unknown15: u32,
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcPingHandler {
  pub timestamp: u32,
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcPing {
  pub time_in_milliseconds: u64,
  #[deku(count = "0x38")]
  pub unknown_8: Vec<u8>,
}

#[derive(Debug, Clone, DekuRead, DekuWrite, Default)]
#[deku(
  endian = "little",
  ctx = "_endian: deku::ctx::Endian",
  ctx_default = "deku::ctx::Endian::Little"
)]
pub struct PlayerEntry {
  pub content_id: u64,
  pub bytes: [u8; 12],
  pub zone_id: u16,
  pub zone_id1: u16,
  pub bytes1: [u8; 8],
  pub online_status_mask: u64,
  pub class_job: u8,
  pub padding: u8,
  pub level: u8,
  pub padding1: u8,
  pub padding2: u16,
  pub one: u8,
  pub name: [u8; 20],
  pub fc_tag: [u8; 9],
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcSocialList {
  pub padding: u32,
  pub padding1: u32,
  pub padding2: u32,
  pub request_type: u8,
  pub sequence: u8,
  pub padding3: u16,
  #[deku(count = "10")]
  pub entries: Vec<PlayerEntry>,
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcBlackList {
  #[deku(count = "20")]
  pub entry: Vec<[u8; 40]>,
  pub padding: u8,
  pub padding1: u8,
  pub sequence: u16,
  pub padding2: u32,
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
#[deku(endian = "little")]
pub struct FFXIVIpcActorControlSelf {
  pub category: u16,
  pub padding: u16,
  pub param1: u32,
  pub param2: u32,
  pub param3: u32,
  pub param4: u32,
  pub param5: u32,
  pub param6: u32,
  pub padding1: u32,
}
