use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ac: [(i64, char); 2 * n]
    }

    ac.sort_unstable();

    let mut r_count = 0;
    let mut g_count = 0;
    let mut b_count = 0;

    let mut rg_min = std::i64::MAX;
    let mut gb_min = std::i64::MAX;
    let mut br_min = std::i64::MAX;

    const MIN_VALUE: i64 = -1_000_000_000_000_001;
    let mut r_min = MIN_VALUE;
    let mut g_min = MIN_VALUE;
    let mut b_min = MIN_VALUE;

    for (a, c) in ac {
        match c {
            'R' => {
                r_count += 1;
                r_min = a;
                rg_min = rg_min.min((r_min - g_min).abs());
                br_min = br_min.min((r_min - b_min).abs());
            }
            'G' => {
                g_count += 1;
                g_min = a;
                rg_min = rg_min.min((g_min - r_min).abs());
                gb_min = gb_min.min((g_min - b_min).abs());
            }
            _ => {
                b_count += 1;
                b_min = a;
                gb_min = gb_min.min((b_min - g_min).abs());
                br_min = br_min.min((b_min - r_min).abs());
            }
        }
    }

    if r_count % 2 == 0 && g_count % 2 == 0 && b_count % 2 == 0 {
        println!("0");
    } else if r_count % 2 == 0 {
        println!("{}", gb_min.min(rg_min + br_min));
    } else if g_count % 2 == 0 {
        println!("{}", br_min.min(gb_min + rg_min));
    } else {
        println!("{}", rg_min.min(br_min + gb_min));
    }
}
