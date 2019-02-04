/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	20/01/2019
 */

/*
run
	cls && bootimage build && bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none
tests
	bootimage test
bootable USB
	dd if=target/x86_64-dandelion/debug/bootimage-dandelion.bin of=/dev/sdX && sync
*/

// configuration
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

extern crate pic8259_simple;
extern crate dandelion;

// modules
mod vga_buffer;
mod serial;

/*
 * OS entry point override
 * This function is the entry point, since the linker looks for a function named `_start` by default
 */

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	use dandelion::interrupts::PICS;

	println!("Hello World{}", "!");

	dandelion::gdt::init();
	dandelion::interrupts::init_idt();
	unsafe { PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();

	println!("It did not crash!");
	dandelion::hlt_loop();
}

/*
 * This function is called on panic.
 * @param	info	information about the panic error
 */

use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	dandelion::hlt_loop();
}