use svg::node::element::Path;
use svg::node::element::path::Data;

pub mod petals {

    pub fn pentagon(x0: f64, y0: f64, length: f64, angle: f64) -> Path {
        let mut points: Vec<(f64,f64)> = vec![];
        let mut theta: f64 = 0.0;
        const DELTA: f64 = 72.0_f64.to_radians();
        for _ in 0..5 {
            let pt = theta.sin_cos();
            points.push((x0 + l * pt.1, y0 + l * pt.0));
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
            .set("d", data);
    }
}
