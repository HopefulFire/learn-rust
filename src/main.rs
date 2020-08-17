
fn main()
{
    let name = "Jade";
    let greeting = format!("hi, I'm {} nice to meet you", name);
    println!("{}", greeting);

    let run = "run";
    let forrest = "forrest";
    let rfr = format!("{0}, {1}, {0}", run, forrest);

    println!("{}", rfr);

    let info = format!(
        "the name's {last}. {first} {last}.",
        first = "James",
        last = "Bond"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        data = "delta"
    );
    println!("{}", mixed);
}
