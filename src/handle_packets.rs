use core::slice::{self};

use deku::DekuContainerWrite;
use md5::Digest;
use tokio::net::TcpStream;

use crate::{
  handle_lobby_packet::handle_lobby_packet,
  packet_transfer::send_packet,
  structs::common::{FFXIVARRPacketSegmentRaw, FFXIVARRSegmentHeader, FFXIVARRSegmentType},
  Db,
};

#[link(name = "FFXIVBlowfish")]
extern "C" {
  pub fn blowfish_encode(key: *const u8, keybytes: u32, pInput: *const u8, lSize: u32)
    -> *const u8;
  pub fn blowfish_decode(key: *const u8, keybytes: u32, pInput: *const u8, lSize: u32)
    -> *const u8;
}

pub fn generate_encryption_key(key: [u8; 4], phrase: &str) -> Digest {
  let size: usize = 0x2C;
  let mut base_key: Vec<u8> = Vec::with_capacity(size);
  base_key.resize(base_key.capacity(), 0);

  base_key[0] = 0x78;
  base_key[1] = 0x56;
  base_key[2] = 0x34;
  base_key[3] = 0x12;
  let before = &base_key[..4];
  let after = &base_key[8..];
  base_key = [before, &key, after].concat();
  // base_key[8] = 0xD4;
  // base_key[9] = 0x17;

  // TODO: Use game version finder (bruteforcer from xivmon)
  let version = (6100 as u16).to_ne_bytes();
  base_key[8] = version[0];
  base_key[9] = version[1];

  base_key[10] = 0x0;
  base_key[11] = 0x0;
  base_key = [&base_key[..12], phrase.as_bytes()].concat();

  if base_key.len() > size {
    panic!("generated key too large!");
  }

  let foo = md5::compute(&base_key);
  return foo;
}

pub const KEY_BYTES: u32 = 0x10;

pub async fn handle_packets(
  packets: Vec<FFXIVARRPacketSegmentRaw>,
  db: &Db,
  socket: &mut TcpStream,
) {
  for mut packet_segment in packets {
    let mut locked_db = db.lock().await;
    let encryption_key_bytes = locked_db.get("encryption_key");

    let empty_encryption_key = vec![0u8; 16];
    let encryption_key = encryption_key_bytes.unwrap_or(&empty_encryption_key);
    if !encryption_key.is_empty() && packet_segment.seg_hdr.segment_type != 9 {
      let size = u32::try_from(packet_segment.data.len()).unwrap();
      let data_received = unsafe {
        let test_move = packet_segment.data.clone();
        let decryption_result =
          blowfish_decode(encryption_key.as_ptr(), KEY_BYTES, test_move.as_ptr(), size);
        slice::from_raw_parts(decryption_result, usize::try_from(size).unwrap()).to_vec()
      };

      packet_segment.data = data_received;
    }

    println!("seg_hdr: {:?}", packet_segment.seg_hdr.segment_type);
    match packet_segment.seg_hdr.segment_type.try_into().unwrap() {
      FFXIVARRSegmentType::EncryptionInit => {
        println!("encryption init!");

        let phrase_start_offset = 36;
        let phrase_end_index = packet_segment.data[phrase_start_offset..]
          .iter()
          .position(|&byte| byte == 0x0)
          .unwrap();
        let phrase = std::str::from_utf8(
          &packet_segment.data[phrase_start_offset..(phrase_start_offset + phrase_end_index)],
        )
        .unwrap();

        let key_offset = 100;
        let key: [u8; 4] = packet_segment.data[key_offset..(key_offset + 4)]
          .try_into()
          .unwrap();

        let new_encryption_key: [u8; 16] = generate_encryption_key(key, phrase).0;
        locked_db.insert("encryption_key".into(), new_encryption_key.to_vec());

        let data = unsafe {
          let mut out_data = 0xE0003C2Au32.to_ne_bytes().to_vec();
          out_data.resize(0x280, 0);

          let result = blowfish_encode(
            new_encryption_key.as_ptr(),
            KEY_BYTES,
            out_data.as_ptr(),
            0x280,
          );
          slice::from_raw_parts(result, 0x280).to_vec()
        };

        let response_packet = FFXIVARRPacketSegmentRaw {
          seg_hdr: FFXIVARRSegmentHeader {
            size: 0x290,
            segment_type: 0x0A,
            ..Default::default()
          },
          data,
        };

        send_packet(socket, vec![response_packet.to_bytes().unwrap()]).await;
        return;
      }
      FFXIVARRSegmentType::IPC => {
        println!("Game packet");

        handle_lobby_packet(socket, encryption_key, packet_segment).await;
        return;
      }
      FFXIVARRSegmentType::KeepAlive => {
        println!("Keep alive packet");

        let id = packet_segment.data[0..4].to_vec();
        let time_stamp = packet_segment.data[4..4].to_vec();

        let mut data = vec![];
        data.extend(id);
        data.extend(time_stamp);

        let response_packet = FFXIVARRPacketSegmentRaw {
          seg_hdr: FFXIVARRSegmentHeader {
            size: 0x18,
            segment_type: 0x08,
            ..Default::default()
          },
          data,
        };

        send_packet(socket, vec![response_packet.to_bytes().unwrap()]).await;
        return;
      }
      _ => {
        eprintln!("Unknown segment type, aborting");
      }
    }
  }

  panic!("unhandled packet!");
}
