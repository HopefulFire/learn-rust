union IntOrFloat
{
    i: i64,
    f: f64
}

fn process_value(iof:IntOrFloat)
{
    unsafe
    {
        match iof
        {
            IntOrFloat{i: 42} => {
                println!("meaning of life");
            }
            IntOrFloat{f} => {
                println!("value = {}", f);
            }
        }
    }
}

fn main()
{
    let mut iof = IntOrFloat{ f:1.24 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("{:?}", value);
    process_value(IntOrFloat{i:5})
}
