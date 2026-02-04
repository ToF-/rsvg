use rand::Rng;
use svg::Document;

const MAX_CIRCLES: usize = 10;
type Point = (f64, f64);

#[derive(Clone, Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn collide(&self, other : &Circle) -> bool {
        let x0 = self.center.0;
        let y0 = self.center.1;
        let x1 = other.center.0;
        let y1 = other.center.1;
        let dx = x1 - x0;
        let dy = y1 - y0;
        let distance: f64 = (dx * dx + dy * dy).sqrt();
        distance < (self.radius + other.radius)
    }
}


fn main() {
    let mut document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000));
    let mut circles: Vec<Circle> = vec![];
    for i in 0..MAX_CIRCLES {
        let x: f64 = rand::thread_rng().gen_range(0..1000) as f64;
        let y: f64 = rand::thread_rng().gen_range(0..1000) as f64;
        let center = (x, y);
        let radius: f64 = rand::thread_rng().gen_range(0..100) as f64;
        let circle = Circle { center, radius };
        let mut collisions: usize = 0;
        for j in 0..circles.len() {
            if circle.collide(&circles[j]) {
                collisions += 1;
            }
        }
        if collisions < 1 {
            circles.push(circle)
        }
    };
    println!("{:?}", circles);
}

