fn main() {
    println!("Hello, world!");
    unsafe { core::mem::transmute::<usize, extern "C" fn() -> !>(0)(); }
    // segfault kek
}
