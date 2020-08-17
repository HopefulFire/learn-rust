
fn strings()
{
    //utf-8 (stack)
    let s:&'static str = "Hello, There!";
    //s = "abc";
    //let h = s[0];
    for c in s.chars().rev()
    {
        println!("{}", c);
    }
    if let Some(first_char) = s.chars().nth(0)
    {
        println!("First letter is {}", first_char);
    }
    //String (heap)
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    let u:&str = &letters;
    // concatenate
    // String + &str

    let mut abc = String::from("Hello world");
    let mut xyz = "Hello world".to_string();
}

fn main()
{
    strings();
}
