use rand::Rng;
use std::io::stdin;
fn main()
{
    let number = rand::thread_rng().gen_range(1, 101);

    loop
    {
        println!("Enter your guess: ");

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer)
        {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed
                {
                    Ok(guess) => {
                        if guess < 1 || guess > 100
                        {
                            println!("Your guess is out of range!");
                            continue;
                        }
                        if guess != number
                        {
                            println!("Your guess was incorrect! ");
                            continue;
                        }
                        println!("Your guess was correct!");
                        break;
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        continue;
                    },
                }
            },
            Err(e) => {
                println!("{:?}", e);
                continue;
            },
        }
    }
}
