use proconio::input;

fn main() {
    input! {
        t1: i64,
        t2: i64,
        a1: i64,
        a2: i64,
        b1: i64,
        b2: i64
    }

    if (a1 - b1).signum() == (a2 - b2).signum() {
        println!("0");
        return;
    } else if ((a1 - b1) * t1).abs() == ((a2 - b2) * t2).abs() {
        println!("infinity");
        return;
    }

    let d1 = (a1 - b1) * t1;
    let d2 = (a2 - b2) * t2;
    let d_total = d1 + d2;

    if d_total.signum() == d1.signum() {
        println!("0");
    } else {
        let r = (d1.abs() + d_total.abs() - 1) / d_total.abs();
        println!(
            "{}",
            r * 2 - if d1.abs() % d_total.abs() == 0 { 0 } else { 1 }
        );
    };
}
