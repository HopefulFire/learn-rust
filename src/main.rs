enum Color
{
    Red,
    Green,
    Blue,
    RGB(u8,u8,u8),
    CMYK{cyan:u8, magenta:u8, yellow:u8, black:u8}
}


fn enums()
{
    let c = Color::RGB(0,0,0);

    match c
    {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RGB(0,0,0) => println!("black"),
        Color::RGB(r,g,b) => println!("rgb({} {} {})", r, g, b),
        Color::CMYK{cyan:c,magenta:m,yellow:y,black:k} => println!("cmyk({} {} {} {})", c, m, y, k),
    }
}

fn main()
{
    enums()
}
