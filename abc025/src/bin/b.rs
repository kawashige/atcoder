use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i32,
        b: i32,
        sd: [(String, i32); n]
    }

    let mut x = 0;

    for (s, d) in sd {
        if s == "East" {
            x += d.min(b).max(a);
        } else {
            x -= d.min(b).max(a);
        }
    }

    if x == 0 {
        println!("0");
    } else if x > 0 {
        println!("East {}", x);
    } else {
        println!("West {}", x.abs());
    }
}
