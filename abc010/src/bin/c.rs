use proconio::input;

fn main() {
    input! {
        txa: f64,
        tya: f64,
        txb: f64,
        tyb: f64,
        t: f64,
        v: f64,
        n: usize,
        xy: [(f64, f64); n]
    }

    for (x, y) in xy {
        if ((x - txa) * (x - txa) + (y - tya) * (y - tya)).sqrt()
            + ((x - txb) * (x - txb) + (y - tyb) * (y - tyb)).sqrt()
            <= t * v
        {
            println!("YES");
            return;
        }
    }

    println!("NO");
}
