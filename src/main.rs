mod graphics;
extern crate noise;

use graphics::{matrix::Matrix, utils::mapper, PPMImg, colors};
use std::f64::consts;

use noise::{NoiseFn, OpenSimplex};

fn main() {
    let n = OpenSimplex::new();
    let (width, height, depth) = (500, 500, 255);
    let mut img = PPMImg::new(width, height, depth);

    // let mut m = Matrix::new(0, 4, vec![]);
    // //
    // m.add_parametric(
    //     |t: f64| 100.0 * (1.0 - (2.0 * consts::PI * t).sin()) * (2.0 * consts::PI * t).cos() + 250.0,
    //     |t: f64| 100.0 * (1.0 - (2.0 * consts::PI * t).sin()) * (2.0 * consts::PI * t).sin() + 250.0,
    //     0.0, 0.001);
    let mag = 3.0_f64.sqrt() / 2.0;
    let to_blue = mapper(-mag, mag, 0.5, 1.0);
    let step = 0.01;

    // img.render_edge_matrix(&m);
    let mut xoff = 0.0;

    for x in 0..width {
        let mut yoff = 0.0;
        for y in 0..height {
            img.fg_color = colors::RGB::from(colors::HSL {
                h: 220.0 / 360.0,
                s: 1.0,
                l: to_blue(n.get([xoff, yoff, 5.0]))
            });
            img.plot(x as i32, y as i32);

            yoff += step;
        }
        xoff += step;
    }
    img.write_binary("img.ppm").expect("Error writing to file");
}
