fn operators()
{
    let mut a = 2+3*4;
    println!("{:?}", a);
    a -= 1;
    println!("remainder of {} / {} = {}", a, 3, (a%3));
    let a_cubed = i64::pow(a, 3);
    println!("{} cubed = {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} {}", b_cubed, b_to_pi);
}

fn main()
{
    operators()
}
