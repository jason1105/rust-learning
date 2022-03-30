fn main() {
    let mut x = 5;
    let r1 = &x as *const i32;
    println!("{:?}", r1);
    unsafe {
        println!("{:?}", *r1);
    }
}
