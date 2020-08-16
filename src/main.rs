
use std::collections::HashSet;

fn main()
{
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    let result = greeks.insert("gamma");

    println!("{:?}", greeks);
    println!("{:?}", result);

    if !greeks.contains("kappa")
    {
        println!("We don't have kappa");
    }
    let removed = greeks.remove("delta");
    if removed
    {
        println!("We removed delta");
    }
    let one_to_five:HashSet<_> = (1..=5).collect();
    let six_to_ten:HashSet<_> = (6..=10).collect();
    let one_to_ten:HashSet<_> = (1..=10).collect();
    let two_to_eight:HashSet<_> = (2..=8).collect();

    println!("is {:?} a subset of {:?}? {}",
        two_to_eight, one_to_ten, two_to_eight.is_subset(&one_to_ten));
    println!("is {:?} disjoint to {:?}? {}",
        one_to_five, six_to_ten, one_to_five.is_disjoint(&six_to_ten));
    println!("items in either {:?} or {:?} are {:?}",
        two_to_eight, six_to_ten, two_to_eight.union(&six_to_ten));
}
