

fn main()
{
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweeden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };

    println!("The country with code {} is {}", country_code, country);
    let x = false;
    let s = match x {
        true => "yes",
        false => "no"
    };
}
