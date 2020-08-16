fn main()
{
    let x = 3.0;
    let y = 2.0;

    let result = if y != 0.0 {Some(x/y)} else {None};
    match result
    {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Cannot divide by zero"),
    }

    if let Some(z) = result {println!("result = {}", z)};
}
