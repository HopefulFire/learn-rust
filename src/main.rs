
fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    return (x+y, x*y)
}

fn tuples()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //destructuring
    let (a,b) = sp;
    println!("a = {}, b = {}", a, b);
    let sp2 = sum_and_product(4,7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last element");
}

fn main()
{
    tuples()
}
