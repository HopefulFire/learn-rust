
pub fn main()
{
    let vec = vec![3,2,1];
    let mut vec2 = vec![1,2,3];
    vec2.extend(vec);

    println!("{:?}", vec2);
}
