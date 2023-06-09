const PRINT_MESSAGE: u32 = 1;

extern "C" fn message() {
    println!("hello from message");
}

#[link(name = "def")]
extern "C" {
    pub fn register_syscall(cmd: u32, cb: extern "C" fn()) -> i32;
    pub fn trigger(cmd: u32);
}

fn main() {
    unsafe {
        if register_syscall(PRINT_MESSAGE, message) < 0 {
            println!("Failed to register function call");
            return;
        }
        trigger(PRINT_MESSAGE);
    }
}
