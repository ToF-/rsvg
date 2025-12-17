
use rand::Rng; // provides the `random` method
use std::f64::consts::PI;
use svg::Document;

use svg::node::element::Circle;
use svg::node::element::Path;
use svg::node::element::path::Data;

fn make_circle(x: i64, y: i64, radius: i64) -> Circle {
    let circle = Circle::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("r", radius)
        .set("cx", x)
        .set("cy", y);
    circle.clone()
}

fn nail(x0: f64, y0: f64, l: f64, a: f64) -> Path {
    let x1 = x0 + l * f64::cos(a);
    let y1 = y0 + l * f64::sin(a);
    let x2 = x1 + (l / 4.0) * f64::cos(a - PI / 2.0);
    let y2 = y1 + (l / 4.0) * f64::sin(a - PI / 2.0);
    let x3 = x0 + (l + (l / 4.0)) * f64::cos(a);
    let y3 = y0 + (l + (l / 4.0)) * f64::sin(a);
    let x4 = x1 + (l / 4.0) * f64::cos(a + PI / 2.0);
    let y4 = y1 + (l / 4.0) * f64::sin(a + PI / 2.0);

    let data = Data::new()
        .move_to((x0, y0))
        .line_to((x1, y1))
        .line_to((x2, y2))
        .line_to((x3, y3))
        .line_to((x4, y4))
        .line_to((x1, y1))
        .close();
    Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data)
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dx: f64 = a.0 - b.0;
    let dy: f64 = a.1 - b.1;
    f64::sqrt(dx * dx + dy * dy)
}


fn main() {
    const n:usize = 5000;
    let mut rng = rand::rng(); // a local handle to the generator
    let mut document = Document::new().set("viewBox", (0, 0, 1000, 1000));
    let mut x0 = 500.0;
    let mut y0 = 500.0;
    for _ in 0..n {
        let l = rng.random::<f64>() * 200.0 - 100.0;
        let a = rng.random::<f64>() * PI * 2.0;
        let x1 = x0 + l * f64::cos(a);
        let y1 = y0 + l * f64::sin(a);
        if distance((x1, y1), (500.0, 500.0)) < 400.0 {
            let l = rng.random::<f64>() * 5.0 + 8.0;
            let a = rng.random::<f64>() * PI * 2.0;
            document = document.add(nail(x1, y1, l, a));
            x0 = x1;
            y0 = y1;
        } else {
            x0 = rng.random::<f64>() * 1000.0;
            y0 = rng.random::<f64>() * 1000.0;
        }
    }
    svg::save("images/nails.svg", &document).unwrap();
}
