
struct Point<T>
{
    x: T,
    y: T
}

struct Line<T>
{
    start: Point<T>,
    end: Point<T>
}


fn generics()
{
    let a:Point<f64> = Point{x:0.0, y:0.0};
    let b:Point<f64> = Point{x:4.1, y:1.2};
    let myline = Line{start:a, end:b};
}

fn main()
{
    generics()
}
