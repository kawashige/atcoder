use proconio::input;

fn main() {
    input! {
        t: usize,
        case: [(i32, i32, i32); t]
    }

    for (r, g, b) in case {
        let mut count = std::i32::MAX;
        if (g - b).abs() % 3 == 0 {
            count = count.min(g.max(b));
        }
        if (r - b) % 3 == 0 {
            count = count.min(r.max(b));
        }
        if (g - r) % 3 == 0 {
            count = count.min(g.max(r));
        }

        if count == std::i32::MAX {
            println!("-1");
        } else {
            println!("{}", count);
        }
    }
}
