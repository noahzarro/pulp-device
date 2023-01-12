#![no_std]

use core::arch::asm;
use core::arch::global_asm;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the CLIC for configuring priority"]
pub const CLIC_PRIO_BITS: u8 = 8;

pub use self::Interrupt as interrupt;
pub use riscv_clic::peripheral::Peripherals as CorePeripherals;
pub use riscv_clic::peripheral::CLIC;

// TODO: Find out why this was in the original implementation, it defines `interrupt` twice
//pub use riscv_clic::interrupt;

#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Interrupt {
    #[doc = "0 - dummy interrupt 0"]
    DUMMY0 = 0,
    #[doc = "1 - dummy interrupt 1"]
    DUMMY1 = 1,
    #[doc = "2 - dummy interrupt 2"]
    DUMMY2 = 2,
    #[doc = "3 - dummy interrupt 3"]
    DUMMY3 = 3,
    #[doc = "4 - dummy interrupt 4"]
    DUMMY4 = 4,
    #[doc = "5 - dummy interrupt 5"]
    DUMMY5 = 5,
    #[doc = "6 - dummy interrupt 6"]
    DUMMY6 = 6,
    #[doc = "7 - dummy interrupt 7"]
    DUMMY7 = 7,
    #[doc = "8 - dummy interrupt 8"]
    DUMMY8 = 8,
    #[doc = "9 - dummy interrupt 9"]
    DUMMY9 = 9,
    #[doc = "10 - timer low interrupt"]
    TIMER_LO = 10,
    #[doc = "11 - dummy interrupt 11"]
    DUMMY11 = 11,
    #[doc = "12 - dummy interrupt 12"]
    DUMMY12 = 12,
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