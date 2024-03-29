#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(lang_items)]

use core::mem::zeroed;
use core::panic::PanicInfo;
use core::ptr::{read, write_volatile};
//use attiny_hal::pac::PORTB;
use avrd::current::DDRB;
use avrd::current::PORTB;

unsafe fn init() {
    // pins
    write_volatile(PORTB, 0x00);
    write_volatile(DDRB, 0xFF);

    // timers, adc, twi/spi
    // interrupts

}

#[no_mangle]
pub fn main() -> ! {
    unsafe { init(); }
   // without_interrupts(unsafe { write_volatile(DDRB, 0xFF) });
    loop {};
}

#[link_section = "_vectors.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: fn() -> ! = reset_handler;

pub fn reset_handler() -> ! {
    extern "C" {
        // These symbols come from `linker.ld`
        static mut __sbss: u32; // Start of .bss section
    static mut __ebss: u32; // End of .bss section
    static mut __sdata: u32; // Start of .data section
    static mut __edata: u32; // End of .data section
    static __sidata: u32; // Start of .rodata section
    }
    // Initialize (Zero) BSS
    unsafe {
        let mut sbss: *mut u32 = &mut __sbss;
        let ebss: *mut u32 = &mut __ebss;

        while sbss < ebss {
            write_volatile(sbss, zeroed());
            sbss = sbss.offset(1);
        }
    }

    // Initialize Data
    unsafe {
        let mut sdata: *mut u32 = &mut __sdata;
        let edata: *mut u32 = &mut __edata;
        let mut sidata: *const u32 = &__sidata;

        while sdata < edata {
            write_volatile(sdata, read(sidata));
            sdata = sdata.offset(1);
            sidata = sidata.offset(1);
        }
    }
    main();
}


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    reset_handler(); // TODO log some error / do something different ?
}

#[lang="eh_personality"]
extern fn eh_personality() -> !{
    reset_handler();
}