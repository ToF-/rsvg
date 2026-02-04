use rand::Rng;
use svg::Document;

const MAX_CIRCLES: usize = 10000;
const MAX_RADIUS: usize = 30;
const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;
type Point = (f64, f64);

#[derive(Clone, Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

pub fn distance(a: &Point, b: &Point) -> f64 {
        let x0 = a.0;
        let y0 = a.1;
        let x1 = b.0;
        let y1 = b.1;
        let dx = x1 - x0;
        let dy = y1 - y0;
        (dx * dx + dy * dy).sqrt()
}

impl Circle {
    pub fn collide(&self, other : &Circle) -> bool {
        distance(&self.center, &other.center) < (self.radius + other.radius)
    }
}


fn main() {
    const CENTER_X: f64 = (MAX_X as f64) / 2.0;
    const CENTER_Y: f64 = (MAX_Y as f64) / 2.0;
    const CENTER: Point = (CENTER_X, CENTER_Y);
    let mut circles: Vec<Circle> = vec![];
    for i in 0..MAX_CIRCLES {
        let x: f64 = rand::thread_rng().gen_range(0..MAX_X) as f64;
        let y: f64 = rand::thread_rng().gen_range(0..MAX_Y) as f64;
        let center = (x, y);
        if distance(&center, &CENTER) < 400.0 {
        let radius: f64 = 5.0; // rand::thread_rng().gen_range(0..MAX_RADIUS) as f64;
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
        }
    };
    let mut document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000));
     for circle in circles { 
        let svg_circle = svg::node::element::Circle::new()
            .set("cx", circle.center.0)
            .set("cy", circle.center.1)
            .set("r", circle.radius)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 2);
        document = document.add(svg_circle);
    };
    svg::save("images/circle.svg", &document).unwrap();

}

