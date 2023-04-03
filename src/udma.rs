use volatile_register::RO;
use volatile_register::RW;

use crate::UDMA;

/// Writes the `bits` into `base_values` at pos [low_bit, high_bit] both included
#[inline]
fn write_bits(base_value: u32, high_bit: u8, low_bit: u8, bits: u32) -> u32 {
    let mut mask = 0;
    for i in low_bit..high_bit + 1 {
        mask += 1 << i;
    }
    let offset = low_bit;
    (base_value & !mask) | bits << offset
}

/// Reads the bits from `base_values` at pos [low_bit, high_bit] both included
#[inline]
fn read_bits(base_value: u32, high_bit: u8, low_bit: u8) -> u32 {
    let mut mask = 0;
    for i in low_bit..high_bit + 1 {
        mask += 1 << i;
    }
    let offset = low_bit;
    (base_value & mask) >> offset
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    pub clock_gating: RW<u32>,
    pub event_input: RW<u32>,
}

impl UDMA {
    #[inline]
    pub fn get_clock_gating() -> u32 {
        unsafe { 
            (*Self::PTR).clock_gating.read()
        }
    }
    
    #[inline]
    pub fn get_event_input() -> u32 {
        unsafe { 
            (*Self::PTR).event_input.read()
        }
    }

    #[inline]
    pub fn disable_clock_gating(&mut self, device_id: u32) {
        unsafe { 
            let before = (*Self::PTR).clock_gating.read();
            let new_value = write_bits(before , 1 << device_id, 1 << device_id,1);
            (*Self::PTR).clock_gating.write(new_value);
        }
    }

    #[inline]
    pub fn enable_clock_gating(&mut self, device_id: u32) {
        unsafe { 
            let before = (*Self::PTR).clock_gating.read();
            let new_value = write_bits(before , 1 << device_id, 1 << device_id,0);
            (*Self::PTR).clock_gating.write(new_value);
        }
    }

    #[inline]
    pub fn enable_event_forward(&mut self, event_block_number:u8,  event_id:u32) {
        unsafe { 
            let event_id = event_id as u8;
            let before = (*Self::PTR).clock_gating.read();
            let new_value = write_bits(before , 8*event_block_number, (8*event_block_number+1)-1,0);
            let new_value = write_bits(new_value , 8*event_block_number+event_id, (8*event_block_number+1)-1+event_id,0);
            (*Self::PTR).clock_gating.write(new_value);
            
        }
    }

    #[inline]
    pub fn enable_event_forward(&mut self, event_block_number:u8,  event_id:u32) {
        unsafe { 
            let event_id = event_id as u8;
            let before = (*Self::PTR).clock_gating.read();
            let new_value = write_bits(before , 8*event_block_number, (8*event_block_number+1)-1,0);
            let new_value = write_bits(new_value , 8*event_block_number+event_id, (8*event_block_number+1)-1+event_id,0);
            (*Self::PTR).clock_gating.write(new_value);
        }
    }

}