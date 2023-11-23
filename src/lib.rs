use winapi::um::winuser::MessageBoxA;

#[no_mangle]
pub unsafe extern "C" fn legitfunction() {
    MessageBoxA(
        std::ptr::null_mut(),
        "Hello from legitfunction!\0".as_ptr()as *const i8 ,
        "Hello\0".as_ptr() as *const i8,
        0,
    );
}