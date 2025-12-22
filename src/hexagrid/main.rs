use std::f64::consts::PI;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use svg::node::element::Line;

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

pub fn line(a: Point, b: Point, color: &str) -> Line {
    Line::new()
        .set("x1", a.0)
        .set("y1", a.1)
        .set("x2", b.0)
        .set("y2", b.1)
        .set("stroke", color)
}

fn main() {
    let mut document = Document::new().set("viewBox", (0, 0, 1000, 1000));
    let x0 = 500.0;
    let y0 = 500.0;
    let circum_radius = 400.0;
    let side = circum_radius;
    const START_ANGLE: f64 = 0.0;
    const MAX_STEPS: usize = 32;
    let hexagon = hexagon(x0, y0, circum_radius, START_ANGLE);
    let mut document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000));
    // document = document.add(line_path(hexagon, "darkgreen"));
    let mut theta = START_ANGLE;
    let mut all_steps: Vec<Vec<Point>> = vec![];
    for _ in 0..6 {
        let sigma = (theta + 180.0_f64.to_radians());
        let alpha = (theta + 120.0_f64.to_radians());
        let beta = (theta + 60.0_f64.to_radians());
        let pt_a = theta.sin_cos();
        let pt_b = sigma.sin_cos();
        let st_a = alpha.sin_cos();
        let st_b = beta.sin_cos();
        let mut dist: f64 = 0.0;
        let mut side_steps: Vec<Point> = vec![];
        for _ in 0..MAX_STEPS+1 {
            let a = (
                x0 + circum_radius * pt_a.1 + dist * st_a.1,
                y0 + circum_radius * pt_a.0 + dist * st_a.0);
            let b = (
                x0 + circum_radius * pt_b.1 + dist * st_b.1,
                y0 + circum_radius * pt_b.0 + dist * st_b.0);
            side_steps.push(a);
            dist += side / MAX_STEPS as f64;
        };
        all_steps.push(side_steps);
        theta += THETA;
    }
    for i in 0..6 {
        for j in 0..MAX_STEPS {
            let point = all_steps[i][j];
            let diag_down = all_steps[(i+1)%6][MAX_STEPS-j];
            let opposite = all_steps[(i+2)%6][MAX_STEPS-j];
            let parallel = all_steps[(i+3)%6][MAX_STEPS-j];
            let diag_up = all_steps[(i+4)%6][MAX_STEPS-j];
            let up = all_steps[(i+5)%6][MAX_STEPS-j];
            if j % 2 == 0 {
                document = document.add(line(all_steps[i][j], all_steps[(i+1)%6][MAX_STEPS-j], "green"));
                document = document.add(line(all_steps[i][j], all_steps[(i+4)%6][MAX_STEPS-j], "orange"));
                document = document.add(line(all_steps[i][j], all_steps[(i+5)%6][MAX_STEPS-j], "black"));

                // document = document.add(line(point, diag_down, "blue"));
                // document = document.add(line(point, diag_up, "blue"));
                // document = document.add(line(point, up, "blue"));
            };
            // document = document.add(line(point, opposite, "blue"));
            document = document.add(line(all_steps[i][j], all_steps[(i+3)%6][MAX_STEPS-j], "red"));
            document = document.add(line(all_steps[i][j], all_steps[(i+2)%6][MAX_STEPS-j], "blue"));


        }
    }
    svg::save("images/hexagrid.svg", &document).unwrap();
}

