use std::f64::consts::PI;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use svg::node::element::Line;

const MAX_EDGES: usize = 4;
const MAX_STEPS: usize = 2;

type Point = (f64, f64);


pub fn mid_point(a: Point, b: Point) -> Point {
    let x = a.0 + (b.0 - a.0) / 2.0;
    let y = a.1 + (b.1 - a.1) / 2.0;
    (x,y)
}


pub fn mid_points(level: usize, a: Point, b: Point) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    let m = mid_point(a, b);
    if level > 0 {
        points.append(&mut mid_points(level - 1, a, m));
    }
    points.push(m);
    if level > 0 {
        points.append(&mut mid_points(level - 1, m, b));
    }
    points
}

pub fn line_points(level: usize, a: Point, b: Point) ->Vec<Point> {
    let mut points: Vec<Point> = vec![];
    points.push(a);
    points.append(&mut mid_points(level, a, b));
    points.push(b);
    points
}

pub fn shape(xc: f64, yc: f64, length: f64, start_angle: f64) -> Vec<Vec<Point>> {
    let mut points: Vec<Vec<Point>> = vec![];
    let mut alpha: f64 = start_angle;
    const THETA: f64 = (2.0 * PI) / MAX_EDGES as f64;
    for _ in 0..MAX_EDGES {
        let p = alpha.sin_cos();
        let q = (alpha + THETA).sin_cos();
        let mut l: f64 = 0.0;
        let x0 = xc + length * p.1;
        let y0 = yc + length * p.0;
        let x1 = xc + length * q.1;
        let y1 = yc + length * q.0;
        let pts: Vec<Point> = line_points(2, (x0,y0), (x1,y1));
        println!("{:?}", pts);
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
    const START_ANGLE: f64 = PI / 4.0; 
    let shape = shape(x0, y0, length, START_ANGLE);
    let mut document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000));
    let mut global: Vec<Point> = vec![];
    for i in 0..MAX_EDGES {
        global.push(shape[i][0]);
    }
    document = document.add(line_path(global, "green"));
    for i in 0..MAX_EDGES {
        document = document.add(line(shape[i][0], shape[(i+1)%MAX_EDGES][0], "blue"));
        document = document.add(line(shape[i][1], shape[(i+1)%MAX_EDGES][1], "purple"));
        document = document.add(line(shape[i][2], shape[(i+1)%MAX_EDGES][2], "red"));
        document = document.add(line(shape[i][3], shape[(i+1)%MAX_EDGES][3], "orange"));
        document = document.add(line(shape[i][4], shape[(i+1)%MAX_EDGES][4], "yellow"));
        document = document.add(line(shape[i][5], shape[(i+1)%MAX_EDGES][5], "green"));
        document = document.add(line(shape[i][6], shape[(i+1)%MAX_EDGES][6], "brown"));
        document = document.add(line(shape[i][7], shape[(i+1)%MAX_EDGES][7], "grey"));
    }
    svg::save("images/shapegrid.svg", &document).unwrap();
}

