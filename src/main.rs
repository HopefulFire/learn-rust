use std::mem;

fn arrays()
{
    let mut a:[i32;5] = [1,2,3,4,5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a);
    if a != [1,2,3,4,5]
    {
        println!("Does not match");
    }
    let b = [1u8;10];
    for i in 0..b.len()
    {
        println!("{}", i);
    }
    println!("b took up {} bytes", mem::size_of_val(&b));
    let mtx:[[f64;3];2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i == j
            {
                println!("{:?}", i);
            }
        }
    }
}

fn main()
{
    arrays();
}
