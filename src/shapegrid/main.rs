use std::f64::consts::PI;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use svg::node::element::Line;

const MAX_EDGES: usize = 5;
const MAX_STEPS: usize = 4;

type Point = (f64, f64);


pub fn mid_point(a: Point, b: Point) -> Point {
    let x = a.0 + (b.0 - a.0) / 2.0;
    let y = a.1 + (b.1 - a.1) / 2.0;
    (x,y)
}

pub fn shape(xc: f64, yc: f64, length: f64, start_angle: f64) -> Vec<Vec<Point>> {
    let mut points: Vec<Vec<Point>> = vec![];
    let mut alpha: f64 = start_angle;
    const THETA: f64 = (2.0 * PI) / MAX_EDGES as f64;
    for _ in 0..MAX_EDGES {
        let p = alpha.sin_cos();
        let q = (alpha + THETA).sin_cos();
        let mut pts: Vec<Point> = vec![];
        let mut l: f64 = 0.0;
        let x0 = xc + length * p.1;
        let y0 = yc + length * p.0;
        let x1 = xc + length * q.1;
        let y1 = yc + length * q.0;
        let dx = x1 - x0;
        let dy = y1 - y0;
        let d = f64::sqrt(dx*dx + dy*dy);
        pts.push((x0,y0));
        pts.push(mid_point((x0,y0),(x1,y1)));
        pts.push(mid_point((x0,y0),mid_point((x0,y0),(x1,y1))));
        pts.push(mid_point(mid_point((x0,y0),(x1,y1)), (x1,y1)));
        alpha += THETA;
        points.push(pts);
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

pub fn line(a: Point, b: Point, color: &str) -> Line {
    Line::new()
        .set("x1", a.0)
        .set("y1", a.1)
        .set("x2", b.0)
        .set("y2", b.1)
        .set("stroke", color)
}


fn main() {
    let x0 = 500.0;
    let y0 = 500.0;
    let length = 400.0;
    const START_ANGLE: f64 = 0.0;
    let shape = shape(x0, y0, length, START_ANGLE);
    let mut document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000));
    let mut global: Vec<Point> = vec![];
    for i in 0..MAX_EDGES {
        global.push(shape[i][0]);
    }
    document = document.add(line_path(global, "green"));
    for i in 0..MAX_EDGES {
        for j in 0..MAX_STEPS {
            for k in 0..MAX_STEPS {
                for l in 0..MAX_EDGES {
                    document = document.add(line(shape[i][j], shape[l][k], "blue"));
                    }
            }
        }
    }
    svg::save("images/shapegrid.svg", &document).unwrap();
}

