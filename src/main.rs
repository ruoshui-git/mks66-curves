mod graphics;
extern crate noise;

use graphics::{
    colors::{HSL, RGB},
    matrix::Matrix,
    utils::mapper,
    PPMImg,
};
use std::f64::consts;

use noise::{NoiseFn, OpenSimplex};

fn main() {
    let (width, height, depth) = (500, 500, 255);
    let mut img = PPMImg::new_with_bg(width, height, depth, RGB::gray(0));
    
    let mut m = Matrix::new(0, 4, vec![]);
    // translation 
    let (tx, ty) = (250., 350.);
    // scale
    let (sx, sy) = (150., 150.);
    m.add_parametric(
        |t: f64| {
            sx * (1.0 - (2.0 * consts::PI * t).sin()) * (2.0 * consts::PI * t).cos() + tx
        },
        |t: f64| {
            sy * (1.0 - (2.0 * consts::PI * t).sin()) * (2.0 * consts::PI * t).sin() + ty
        },
        0.0,
        0.001,
    );
    let bound_color = RGB::new(101, 67, 33);
    img.fg_color = bound_color;
    img.render_edge_matrix(&m);
    
    // now draw the sky

    let n = OpenSimplex::new();
    // magnitude of cubic simplex noise
    let mag = 3.0_f64.sqrt() / 2.0;
    // closure to map noise range to my luminosity range
    let to_blue = mapper(-mag, mag, 0.5, 1.);
    // change in input to noise fn
    let step = 0.01;

    for i in 1..15
    {
        let z = 5. + i as f64 * step * 6.;
        let sky_filler = |x: f64, y: f64| {
            RGB::from(HSL {
                h: 220.0 / 360.0,
                s: 1.0,
                l: to_blue(n.get([x as f64 * step, y as f64 * step, z])),
            })
        };
        img.bound4_fill_with_fn(250, 230, sky_filler, bound_color);
        img.write_binary(format!("img{}.ppm", i).as_str()).expect("Error writing to file");
    }
    
}

// compile: cargo run --release
// build gif: convert -delay 15 img{1..14}.ppm img{13..2}.ppm img.gif
// requires external library `noise` (using OpenSimplex)