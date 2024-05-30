use std::mem;

use deku::DekuContainerRead;

use crate::structs::common::{
  CompressionType, FFXIVARRPacketHeader, FFXIVARRPacketSegmentRaw, FFXIVARRSegmentHeader,
};

fn get_segment_header(buffer: &Vec<u8>, offset: usize) -> Option<FFXIVARRSegmentHeader> {
  let header_size = mem::size_of::<FFXIVARRSegmentHeader>();
  let remaining_bytes = buffer.len() - offset;
  if remaining_bytes < header_size {
    eprintln!("Malformed packet segment header!");
    return None;
  }

  let (_rest, seg_hdr) = FFXIVARRSegmentHeader::from_bytes((&buffer[offset..], 0)).unwrap();
  Some(seg_hdr)
}

fn get_packet(buffer: &Vec<u8>, offset: usize) -> Option<FFXIVARRPacketSegmentRaw> {
  let segment_header = get_segment_header(&buffer, offset).unwrap();

  if segment_header.size > 256 * 1024 {
    eprintln!("segment header size too large!");
    return None;
  }

  let data_offset = offset + mem::size_of::<FFXIVARRSegmentHeader>();

  let raw_packet = FFXIVARRPacketSegmentRaw {
    seg_hdr: segment_header,
    data: buffer[data_offset..].to_vec(),
  };

  return Some(raw_packet);
}

pub fn process_packets(
  buffer: &Vec<u8>,
  header: FFXIVARRPacketHeader,
  offset: usize,
) -> Vec<FFXIVARRPacketSegmentRaw> {
  // match header
  match unsafe { mem::transmute(header.compression_type) } {
    CompressionType::NoCompression => {}
    CompressionType::Oodle => {}
    _ => println!("Compression type not found!"),
  }

  let mut bytes_processed: usize = 0;
  let mut packets = Vec::<FFXIVARRPacketSegmentRaw>::new();
  let mut count = 0;
  while count < header.count {
    let packet = get_packet(&buffer, offset + bytes_processed).unwrap();

    bytes_processed += usize::try_from(packet.seg_hdr.size).unwrap();
    packets.push(packet);
    count += 1;
  }

  return packets;
}
