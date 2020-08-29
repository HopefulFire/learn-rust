use rand::prelude::*;
use regex::Regex;
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

    {
        let vowel = Regex::new(r"[aeiou]").unwrap();
        let s1 = String::from("Hello my name is Fred");
        let mut s2 = String::new();
        let split_s1 = s1.split_whitespace();
        for word in split_s1
        {
            let mut new_word = String::new();
            let mut suffix = String::from("hay");
            for (i, c) in word.chars().enumerate()
            {
                if i == 0 && !vowel.is_match(&format!("{}", c))
                {
                    suffix = format!("{}ay", c);
                    continue;
                }
                new_word.push_str(&format!("{}", c));
            }
            s2.push_str(&format!("{}{} ", new_word, suffix));
        }
        println!("{}", s2);
    }
}