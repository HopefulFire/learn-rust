
fn if_statement()
{
    let temp = 15;
    if temp > 30
    {
        println!("Really hot outside!");
    }
    else if temp < 10
    {
        println!("Baby it's cold outside!");
    }
    else
    {
        println!("Temperature is ok.");
    }
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);
    println!("it is {}", if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"normal"});
    // more stuff
}

fn main()
{
    if_statement();
}
