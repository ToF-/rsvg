use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

const THETA: f64 = 72.0_f64.to_radians();

type Point = (f64, f64);

pub fn pentagon(x: f64, y: f64, radius: f64, start_angle: f64) -> Vec<Point> {
    let mut points: Vec<(f64,f64)> = vec![];
    let mut theta: f64 = start_angle;
    for _ in 0..5 {
        let pt = theta.sin_cos();
        points.push((x + radius * pt.1, y + radius * pt.0));
        theta += THETA; 
    }
    points
}

pub fn lines_path(points: Vec<Point>, color: &str) -> Path {
    let mut data = Data::new()
        .move_to(points[0]);
    for i in 1..5 {
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

pub fn arcs_path(points: Vec<Point>, arc_radius: f64, color: &str) -> Path {
    let mut data = Data::new();
    for i in 0..5 {
        data = data.move_to(points[i]);
        let dest = points[(i + 2) % 5]; 
        data = data.elliptical_arc_to((
                arc_radius,
                arc_radius,
                0.0,
                0,
                0,
                dest.0,
                dest.1)
        );
        //        0.0,
        //        false,
        //        true,
        //        dest.0,
        //        dest.1,);
    };
    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", 1)
        .set("d", data)
}

fn main() {
    let x0 = 500.0;
    let y0 = 500.0;
    let circum_radius = 100.0;
    let side = circum_radius * 1.176;
    let in_radius = circum_radius * 0.809016;
    let large_pentagon = pentagon(x0, y0, in_radius * 2.0, 0.0);
    let mut theta = THETA / 2.0;
    let mut document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000));
    let central_pentagon = pentagon(x0, y0, circum_radius, THETA / 2.0);
    document = document.add(arcs_path(central_pentagon, side, "green"));
    for i in 0..5 {
        let pt = large_pentagon[i];
        let small_pentagon = pentagon(pt.0, pt.1, circum_radius, theta + THETA / 2.0);
        let xt: Point = (pt.0 + in_radius * 2.0 * theta.cos(), pt.1 + in_radius * 2.0 * theta.sin());
        let extra_pentagon = pentagon(xt.0, xt.1, circum_radius, theta);
        let yt: Point = (pt.0 + in_radius * 2.0 * (theta + THETA * 4.0 ).cos(), pt.1 + in_radius * 2.0 * (theta + THETA * 4.0).sin());
        let yxtra_pentagon = pentagon(yt.0, yt.1, circum_radius, theta);
        theta += THETA;
        document = document.add(arcs_path(small_pentagon, side, "green"));
        document = document.add(arcs_path(extra_pentagon, side, "green"));
        document = document.add(arcs_path(yxtra_pentagon, side, "green"));
    }
    svg::save("images/petals.svg", &document).unwrap();
}
