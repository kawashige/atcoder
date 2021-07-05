use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        xya: [(usize, usize, usize); n]
    }

    let mut x_min = 0;
    let mut x_max = w;
    let mut y_min = 0;
    let mut y_max = h;

    for (x, y, a) in xya {
        match a {
            1 => x_min = std::cmp::max(x_min, x),
            2 => x_max = std::cmp::min(x_max, x),
            3 => y_min = std::cmp::max(y_min, y),
            _ => y_max = std::cmp::min(y_max, y),
        }
    }

    if x_min >= x_max || y_min >= y_max {
        println!("0");
    } else {
        println!("{}", (x_max - x_min) * (y_max - y_min));
    }
}
