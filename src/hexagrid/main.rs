use std::f64::consts::PI;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

const THETA: f64 = 60.0_f64.to_radians();

type Point = (f64, f64);

pub fn hexagon(x: f64, y: f64, radius: f64, start_angle: f64) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    let mut theta: f64 = start_angle;
    for _ in 0..6 {
        let pt = theta.sin_cos();
        points.push((x + radius * pt.1, y + radius * pt.0));
        theta += THETA;
    }
    points
}

pub fn line_path(points: Vec<Point>, color: &str) -> Path {
    let mut data = Data::new()
        .move_to(points[0]);
    for i in 1..points.len() {
        data = data.line_to(points[i]);
    }
    data = data.line_to(points[0]);
    data = data.close();
    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", 1)
        .set("d", data)
}

fn main() {
    let mut document = Document::new().set("viewBox", (0, 0, 1000, 1000));
    let x0 = 500.0;
    let y0 = 500.0;
    let circum_radius = 400.0;
    let side = circum_radius;
    let hexagon = hexagon(x0, y0, circum_radius, THETA / 2.0);
    let mut document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000));
    document = document.add(line_path(hexagon, "darkgreen"));
    svg::save("images/hexagrid.svg", &document).unwrap();
}

