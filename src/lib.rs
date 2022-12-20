#![no_std]

use core::arch::asm;
use core::arch::global_asm;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the CLIC for configuring priority"]
pub const CLIC_PRIO_BITS: u8 = 4;

pub use self::Interrupt as interrupt;
pub use riscv_clic::peripheral::Peripherals as CorePeripherals;
pub use riscv_clic::peripheral::CLIC;

// TODO: Find out why this was in the original implementation, it defines `interrupt` twice
//pub use riscv_clic::interrupt;

#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Interrupt {
    #[doc = "0 - TEST interrupt"]
    TEST = 0,
    #[doc = "0 - NEST interrupt"]
    NEST = 1,
}

unsafe impl riscv_clic::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> usize {
        self as usize
    }
}

// all non core peripherals
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {}

impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {}
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

/*
// adding interrupt vector
global_asm!("
.global
.option norvc
.interrupt_vector
j int_0 
j int_1 
j int_2 
");
*/