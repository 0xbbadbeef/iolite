use core::panic;
use std::mem;

use deku::DekuContainerRead;

use crate::{
  structs::{
    common::{
      CompressionType, FFXIVARRPacketHeader, FFXIVARRPacketSegmentRaw, FFXIVARRSegmentHeader,
    },
    lobby_structs::FFXIVIpcHeader,
  },
  world::oodle::FFXIVOodle,
};

pub struct IpcResponse {
  pub ipc_header: FFXIVIpcHeader,
  pub segment: Vec<u8>,
}

fn get_segment_header(buffer: &[u8], offset: usize) -> Option<FFXIVARRSegmentHeader> {
  let header_size = mem::size_of::<FFXIVARRSegmentHeader>();
  let remaining_bytes = buffer.len() - offset;
  if remaining_bytes < header_size {
    panic!("Malformed packet segment header!");
  }

  let (_rest, seg_hdr) = FFXIVARRSegmentHeader::from_bytes((&buffer[offset..], 0)).unwrap();
  Some(seg_hdr)
}

fn get_packet(buffer: &[u8], offset: usize) -> Option<FFXIVARRPacketSegmentRaw> {
  let segment_header = get_segment_header(buffer, offset).unwrap();

  if segment_header.size > 256 * 1024 {
    panic!("segment header size too large!");
  }

  let data_offset = offset + mem::size_of::<FFXIVARRSegmentHeader>();

  let raw_packet = FFXIVARRPacketSegmentRaw {
    seg_hdr: segment_header,
    data: buffer[data_offset..].to_vec(),
  };

  Some(raw_packet)
}

pub async fn process_packets(
  decomp_buffer: &[u8],
  header: &FFXIVARRPacketHeader,
  oodle: Option<&mut FFXIVOodle>,
) -> Vec<FFXIVARRPacketSegmentRaw> {
  // match header
  let mut buffer = decomp_buffer.to_vec();

  match unsafe { mem::transmute(header.compression_type) } {
    CompressionType::NoCompression => {}
    CompressionType::Oodle => {
      let result = oodle
        .unwrap()
        .decode(buffer, header.oodle_decompressed_size);

      buffer = result;
    }
    CompressionType::Zlib => {
      panic!("Compression type Zlib was specified but it was unhandled.")
    }
  }

  let mut bytes_processed: usize = 0;
  let mut packets = Vec::<FFXIVARRPacketSegmentRaw>::new();
  let mut count = 0;
  while count < header.count {
    let packet = get_packet(buffer.as_slice(), bytes_processed).unwrap();

    bytes_processed += usize::try_from(packet.seg_hdr.size).unwrap();
    packets.push(packet);
    count += 1;
  }

  packets
}
