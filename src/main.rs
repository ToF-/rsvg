use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use svg::node::element::Circle;

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

fn main() {
let data = Data::new()
    .move_to((100, 100))
    .line_by((0, 500))
    .line_by((500, 0))
    .line_by((0, -500))
    .close();

let mut document = Document::new()
    .set("viewBox", (0, 0, 1700, 1700));
    for r in 0..150 {
        document = document.add(make_circle(350, 750, r*3));
    };
    for r in 0..150 {
        document = document.add(make_circle(750, 750, r*3));
    };
    svg::save("image.svg", &document).unwrap();
}
