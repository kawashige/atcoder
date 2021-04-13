use proconio::input;

fn main() {
    input! {
        r: f64,
        x: f64,
        y: f64
    }

    let mut d = (x * x + y * y).sqrt();
    let mut count = 0;
    while d > 2.0 * r {
        d -= r;
        count += 1;
    }

    if d == r {
        println!("{}", count + 1);
    } else {
        println!("{}", count + 2);
    }
}
