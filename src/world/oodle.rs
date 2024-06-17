use std::ptr::null;

use libc::{c_void, intptr_t};

#[link(name = "oo2net_9_win64")]
extern "C" {
  pub fn OodleNetwork1TCP_State_Size() -> intptr_t;
  pub fn OodleNetwork1_Shared_Size(htbits: i32) -> intptr_t;
  pub fn OodleNetwork1_Shared_SetWindow(
    shared: *mut c_void,
    htbits: i32,
    window: *const c_void,
    window_size: i32,
  ) -> c_void;
  pub fn OodleNetwork1TCP_Train(
    state: *mut c_void,
    shared: *const c_void,
    training_packet_pointers: *const c_void,
    training_packet_sizes: i32,
    num_training_packets: i32,
  ) -> c_void;
  pub fn OodleNetwork1TCP_Decode(
    state: *mut c_void,
    shared: *const c_void,
    enc: *const c_void,
    enc_size: intptr_t,
    dec: *mut c_void,
    dec_size: intptr_t,
  ) -> bool;
  pub fn OodleNetwork1TCP_Encode(
    state: *mut c_void,
    shared: *const c_void,
    dec: *const c_void,
    dec_size: intptr_t,
    enc: *mut c_void,
  ) -> bool;
}

#[derive(Debug, Default)]
pub struct FFXIVOodle {
  state: Vec<u8>,
  shared: Vec<u8>,
  #[allow(dead_code)] // unused in rust but required to still be available for low-level oodle
  window: Vec<u8>,
}

impl FFXIVOodle {
  pub fn new() -> FFXIVOodle {
    let htbits: i32 = 17;
    unsafe {
      let oodle_state_size: usize = OodleNetwork1TCP_State_Size().try_into().unwrap();
      let oodle_shared_size: usize = OodleNetwork1_Shared_Size(17).try_into().unwrap();
      let mut oodle_state = vec![0u8; oodle_state_size];
      let mut oodle_shared = vec![0u8; oodle_shared_size];
      let mut oodle_window = [0u8; 0x100000].to_vec();

      OodleNetwork1_Shared_SetWindow(
        oodle_shared.as_mut_ptr() as *mut c_void,
        htbits,
        oodle_window.as_mut_ptr() as *mut c_void,
        oodle_window.len().try_into().unwrap(),
      );
      OodleNetwork1TCP_Train(
        oodle_state.as_mut_ptr() as *mut c_void,
        oodle_shared.as_mut_ptr() as *mut c_void,
        null(),
        0,
        0,
      );

      FFXIVOodle {
        state: oodle_state,
        shared: oodle_shared,
        window: oodle_window,
      }
    }
  }

  pub fn decode(&mut self, input: Vec<u8>, decompressed_size: u32) -> Vec<u8> {
    unsafe {
      let mut out_buf: Vec<u8> = vec![0u8; decompressed_size.try_into().unwrap()];
      let mut in_buf = input.to_vec();
      println!("window ptr: {:?}", self.window.as_ptr());
      let success = OodleNetwork1TCP_Decode(
        self.state.as_mut_ptr() as *mut c_void,
        self.shared.as_mut_ptr() as *mut c_void,
        in_buf.as_mut_ptr() as *const c_void,
        in_buf.len().try_into().unwrap(),
        out_buf.as_mut_ptr() as *mut c_void,
        out_buf.len().try_into().unwrap(),
      );

      if !success {
        panic!("Failed to oodle decode for an unknown reason.");
      }

      out_buf
    }
  }
}
