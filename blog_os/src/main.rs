#![no_std]
#![no_main]


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello Peterson OS!";

#[unsafe(no_mangle)]
pub extern "C" fn _start()-> ! {
    let vba_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vba_buffer.offset(i as isize* 2) = byte;
            *vba_buffer.offset(i as isize*2 + 1) = 0xb;
        }
    }


    loop {}
}


