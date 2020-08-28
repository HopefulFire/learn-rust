use rand::prelude::*;
use std::collections::HashMap;

fn main()
{
    {
        // generates 100 numbers from -100 to 100
        let mut rng = rand::thread_rng();
        let mut num_vec:Vec<isize> = Vec::new();
        for _ in 0..100
        {
            num_vec.push(rng.gen_range(-100, 101));
        }
        // prints mean
        let mut sum = 0;
        for i in 0..num_vec.len()
        {
            sum += num_vec[i];
        }
        println!("{}", sum as f64 / num_vec.len() as f64);
        // prints median
        num_vec.sort();
        println!("{}", num_vec[num_vec.len() / 2]);
        // prints mode
        let mut number_amounts:HashMap<isize, isize> = HashMap::new();
        for i in 0..num_vec.len()
        {
            let count = number_amounts.entry(num_vec[i]).or_insert(0);
            *count += 1;
        }
        let mut most_used = 0;
        let mut biggest_value = 0;
        for (key, value) in number_amounts.into_iter()
        {
            if value > biggest_value
            {
                most_used = key;
                biggest_value = value;
            }
        }
        println!("{}", most_used);
    }
}