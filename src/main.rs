#![no_std]
#![no_main]

mod console;
mod lang_item;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() -> ! {
    println!("Hello world!");
    loop {}
}
