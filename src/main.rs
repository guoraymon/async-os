#![no_std]
#![no_main]

mod async_utils;
mod console;
mod lang;
mod sync;
mod timer;

use async_utils::{delay, RUNTIME};
use buddy_system_allocator::LockedHeap;

core::arch::global_asm!(include_str!("entry.asm"));

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const KERNEL_HEAP_SIZE: usize = 0x40_0000; // 4 MB
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[no_mangle]
fn rust_main() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }

    let mut rt = RUNTIME.exclusive_access();
    rt.spawn(task1());
    rt.spawn(task2());
    rt.run();

    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::SystemFailure);
}

async fn task1() {
    println!("start task 1");
    delay(200).await;
    println!("end task 1");
}

async fn task2() {
    println!("start task 2");
    delay(500).await;
    println!("end task 2");
}
