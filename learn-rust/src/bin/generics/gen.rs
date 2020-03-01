struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn  x(&self) -> &T {
        &self.x
    }

    fn mix<W>(&self, other: Point<W>) -> W {
        other.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn point() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    let other = Point { x: 1.1, y: 2.2 };
    println!("o {}",other.distance_from_origin());
    println!("p.mix = {}",p.mix(other));

}
