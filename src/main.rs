use text_io::read;
use rand::prelude::*;
mod cat;

fn main()
{
    let sadie = cat::Cat {
        name: "Sadie".to_string(),
        age: 1,
        tail: true,
        long_hair: false,
        pattern: "Dark Tabby".to_string(),
        personality: "Affectionate, curious, skittish".to_string(),
        deaf: true,
    };
    let rosey = cat::Cat {
        name: "Rosey".to_string(),
        age: 1,
        tail: false,
        long_hair: true,
        pattern: "Black and White".to_string(),
        personality: "Affectionate, introverted, skittish".to_string(),
        deaf: false,
    };
    let dovey = cat::Cat {
        name: "Dovey".to_string(),
        age: 1,
        tail: true,
        long_hair: false,
        pattern: "Grey and White".to_string(),
        personality: "Affectionate, social, skittish".to_string(),
        deaf: false,
    };
    let mut a = [dovey, rosey, sadie];
    let mut rng = rand::thread_rng();
    a.shuffle(&mut rng);
    let unknown_cat = &a[0];

    loop
    {
        println!("Guess the cat!");
        let cat_guess:String = read!();
        if cat_guess == unknown_cat.name
        {
            println!("Great Job, you guessed {}", cat_guess);
            break;
        }
        else
        {
            println!("Nope, wrong cat.");
        }
    }
}