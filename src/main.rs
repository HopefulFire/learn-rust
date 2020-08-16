use std::collections::HashMap;
fn main()
{
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("quadrilateral"), 4);
    shapes.insert(String::from("heptagon"), 5);
    shapes.insert(String::from("hexagon"), 6);

    shapes.insert(String::from("quadrilateral"), 5);

    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
        println!("{:?}", shapes);
    }
}
