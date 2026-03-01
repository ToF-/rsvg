use svg::node::element::Path;
use svg::node::element::path::Data;
use rand::Rng;
use svg::Document;

const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;
const MAX_POINTS: usize = 1000;


type Point = (f64, f64);

pub fn distance(a: &Point, b: &Point) -> f64 {
        let x0 = a.0;
        let y0 = a.1;
        let x1 = b.0;
        let y1 = b.1;
        let dx = x1 - x0;
        let dy = y1 - y0;
        (dx * dx + dy * dy).sqrt()
}

fn triangle(a: Point, b: Point, c: Point) -> Path {
    let mut data = Data::new()
        .move_to(a);
    data = data.line_to(b);
    data = data.line_to(c);
    data = data.line_to(a);
    data = data.close();
    Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data)
}

fn main() {
    let mut document = Document::new()
        .set("viewBox", (0, 0, MAX_X, MAX_Y));
    let mut points: Vec<Point> = vec![];
    for i in 0..MAX_POINTS {
        let x: f64 = rand::thread_rng().gen_range(0..MAX_X) as f64;
        let y: f64 = rand::thread_rng().gen_range(0..MAX_Y) as f64;
        let p: Point = (x, y);
        points.push(p);
    }
    points.push((f64::MAX, f64::MAX));
    for i in 0..MAX_POINTS {
        let a: usize = i;
        let mut b: usize = MAX_POINTS;
        let mut c: usize = MAX_POINTS;
        for j in 0..MAX_POINTS {
            if j != a &&
                distance(&points[a], &points[j]) < distance(&points[a], &points[b]) {
                    b = j;
            }
        }
        for j in 0..MAX_POINTS {
            if j != a && j != b &&
                distance(&points[a], &points[j]) < distance(&points[a], &points[c]) {
                    c = j;
            }
        }
        document = document.add(triangle(points[a], points[b], points[c]));
        println!("{:?} {:?} {:?}", points[a], points[b], points[c]);
    }
    svg::save("images/triangles.svg", &document).unwrap();
}

