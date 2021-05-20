use proconio::input;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        ee: [f64; q]
    }

    for e in ee {
        let theta = (360.0 * e) / t;

        let rad = theta * std::f64::consts::PI / 180.0;
        let p = (
            0.0,
            rad.sin() * (-l / 2.0),
            rad.cos() * (-l / 2.0) + l / 2.0,
        );

        let xy = (x, y - p.1, 0.0);

        let r = (p.2 / (xy.0 * xy.0 + xy.1 * xy.1).sqrt()).atan() * 180.0 / std::f64::consts::PI;
        println!("{}", r);
    }
}
