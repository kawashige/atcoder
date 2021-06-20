use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize
    }

    if h % 3 == 0 || w % 3 == 0 {
        println!("0");
        return;
    }

    let mut r = std::usize::MAX;

    if h > 2 {
        for i in 0..(w / 2) {
            let mut v = vec![
                (i + 1) * h,
                ((w - i - 1) / 2) * h,
                ((w - i - 1) - (w - i - 1) / 2) * h,
            ];
            v.sort_unstable();

            r = std::cmp::min(r, v[2] - v[0]);
        }
    }

    if w > 2 {
        for i in 0..(h / 2) {
            let mut v = vec![
                (i + 1) * w,
                ((h - i - 1) / 2) * w,
                ((h - i - 1) - (h - i - 1) / 2) * w,
            ];
            v.sort_unstable();

            r = std::cmp::min(r, v[2] - v[0]);
        }
    }

    for i in 0..=(w / 2) {
        let mut v = vec![
            (i + 1) * h,
            (w - i - 1) * (h / 2),
            (w - i - 1) * (h - h / 2),
        ];
        v.sort_unstable();

        r = std::cmp::min(r, v[2] - v[0]);
    }

    for i in 0..=(h / 2) {
        let mut v = vec![
            (i + 1) * w,
            (h - i - 1) * (w / 2),
            (h - i - 1) * (w - w / 2),
        ];
        v.sort_unstable();

        r = std::cmp::min(r, v[2] - v[0]);
    }

    println!("{}", r);
}
