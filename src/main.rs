fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("{:?}", a);
    a.push(44);
    println!("{:?}", a);

    let idx:usize = 2;

    //a[idx] = 312;

    match a.get(idx)
    {
        Some(x) => {
            println!("a[{}] = {}", idx, x);
        },
        None => {
            println!("error, no such element");
        }
    }

    a.push(20);

    for x in &a
    {
        println!("x: {}", x);
    }

    let last_elem = a.pop();
    println!("{:?}", last_elem);
}

fn main()
{
    vectors()
}
