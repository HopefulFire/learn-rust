fn scope_and_shadowing()
{
    let a = 123;
    {
        let b = 456;
        println!("inside {:?}", b);
        let a = 777;
        println!("inside {:?}", a);
    }
    println!("outside {:?}", a);
    // println!("{:?}", b);
}

fn main()
{
    scope_and_shadowing()
}
