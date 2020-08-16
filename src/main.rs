
fn while_and_loop()
{
    let mut x:u64 = 1;
    while x < 1000
    {
        x *= 2;
        if x == 64
        {
            continue;
        }
        println!("{}", x);
    }
    let mut y:u64 = 0;
    loop
    {
        y += 1;
        println!("y = {}", y);

        if y == 1<<10
        {
            break;
        }
    }
}

fn main()
{
    while_and_loop();
}
