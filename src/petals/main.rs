use std::f64::consts::PI;
use svg::Document;

use svg::node::element::Circle;
use svg::node::element::Path;
use svg::node::element::path::Data;

const DELTA: f64 = 72.0_f64.to_radians();

pub fn pentagon(x0: f64, y0: f64, length: f64, angle: f64) -> Path {
    let mut points: Vec<(f64,f64)> = vec![];
    let mut theta: f64 = angle;
    for _ in 0..5 {
        let pt = theta.sin_cos();
        points.push((x0 + length * pt.1, y0 + length * pt.0));
        theta += DELTA; 
    }
    let mut data = Data::new()
        .move_to(points[0]);
    for i in 1..5 {
        data = data.line_to(points[i]);
    }
    data = data.line_to(points[0]);
    data = data.close();
    Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 1)
        .set("d", data)
}

fn main() {
    let mut document = Document::new().set("viewBox", (0, 0, 1000, 1000));
    let x0 = 500.0;
    let y0 = 500.0;
    let l = 100.0;
    let a = 2.0 * l * (PI / 5.0).sin();
    let r = a / 2.0 * (1.0 / (PI / 5.0).tan());
    let mut document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000));
    document = document.add(pentagon(500.0, 500.0, 100.0, 0.0));
    document = document.add(pentagon(500.0 + 2.0 * r * (DELTA / 2.0).cos(), 500.0 + 2.0 * r * (DELTA / 2.0).sin(), 100.0, DELTA / 2.0));
    document = document.add(pentagon(500.0 + 2.0 * r * (DELTA + DELTA / 2.0).cos(), 500.0 + 2.0 * r * (DELTA + DELTA / 2.0).sin(), 100.0, DELTA / 2.0));
    document = document.add(pentagon(500.0 + 2.0 * r * (DELTA * 2.0 + DELTA / 2.0).cos(), 500.0 + 2.0 * r * (DELTA * 2.0 + DELTA / 2.0).sin(), 100.0, DELTA / 2.0));
    document = document.add(pentagon(500.0 + 2.0 * r * (DELTA * 3.0 + DELTA / 2.0).cos(), 500.0 + 2.0 * r * (DELTA * 3.0 + DELTA / 2.0).sin(), 100.0, DELTA / 2.0));
    document = document.add(pentagon(500.0 + 2.0 * r * (DELTA * 4.0 + DELTA / 2.0).cos(), 500.0 + 2.0 * r * (DELTA * 4.0 + DELTA / 2.0).sin(), 100.0, DELTA / 2.0));
    svg::save("images/petals.svg", &document).unwrap();
}
