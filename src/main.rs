use std::mem;

fn main() {
    let a:u8 = 123;
    println!("a = {:?}", a);
    //cannot a = 128;
    let mut b:i8 = 0;
    println!("b = {:?}", b);
    b = 42;
    println!("b = {:?}", b);

    let mut c = 123456789;
    println!("c = {:?}, size = {:?}", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {:?}, size = {:?}", c, mem::size_of_val(&c));

    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {}", d);

    let g = false;
    println!("g = {}", g);

    let f = 4>0;
    println!("f = {}", f);
}
