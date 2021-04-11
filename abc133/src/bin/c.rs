use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize
    }

    if r - l >= 2019 {
        println!("0");
    } else {
        let mut min = std::usize::MAX;
        for i in l..=r {
            for j in (i + 1)..=r {
                let m = (i % 2019) * (j % 2019) % 2019;
                min = std::cmp::min(min, m);
            }
        }

        println!("{}", min);
    }
}
