#![no_std]

use core::arch::asm;

pub use self::Interrupt as interrupt;
//use bare_metal::Nr;

pub enum Interrupt {}

pub struct Peripherals {
    _0: (),
}

impl Peripherals {
    /// Kind of useless as there's no register API but this is required by RTFM
    #[inline]
    pub unsafe fn steal() -> Self {
        Peripherals { _0: () }
    }
}

pub fn exit(main_res: u32) {
    let main_res = main_res | 0x80000000;
    let ret_reg = 0x1a1040a0;

    unsafe {
        asm!(
            "sw	{0},0({1})",
            in(reg) main_res,
            in(reg) ret_reg
        );

        loop {
            asm!("wfi");
        }
    }
}
