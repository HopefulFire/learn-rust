fn for_loop()
{
    for x in 1..=10
    {
        if x == 3 { continue; }
        if x == 8 { break; }
        println!("x = {}", x);
    }
    for (pos, y) in (30..=40).enumerate()
    {
        println!("{}: {}", pos, y);
    }
}

fn main()
{
    for_loop();
}
