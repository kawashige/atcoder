use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }

    let mut points = Vec::new();

    for i in 0..h {
        for j in 0..w {
            let k = if i % 2 == 0 { j } else { w - 1 - j };
            if a[i][k] % 2 == 1 {
                points.push((i, k));
            }
        }
    }

    if points.len() % 2 == 1 {
        points.pop();
    }

    let mut path = Vec::new();

    for i in (0..points.len()).step_by(2) {
        let mut s = points[i];

        if s.1 == points[i + 1].1 {
            while s.0 < points[i + 1].0 {
                path.push((s.0, s.1, s.0 + 1, s.1));
                s.0 += 1;
            }
        } else if s.1 < points[i + 1].1 {
            if points[i].0 % 2 == 0 {
                while s.1 < points[i + 1].1 {
                    path.push((s.0, s.1, s.0, s.1 + 1));
                    s.1 += 1;
                }
                while s.0 < points[i + 1].0 {
                    path.push((s.0, s.1, s.0 + 1, s.1));
                    s.0 += 1;
                }
            } else {
                while s.0 < points[i + 1].0 {
                    path.push((s.0, s.1, s.0 + 1, s.1));
                    s.0 += 1;
                }
                while s.1 < points[i + 1].1 {
                    path.push((s.0, s.1, s.0, s.1 + 1));
                    s.1 += 1;
                }
            }
        } else {
            if points[i].0 % 2 == 0 {
                while s.0 < points[i + 1].0 {
                    path.push((s.0, s.1, s.0 + 1, s.1));
                    s.0 += 1;
                }
                while s.1 > points[i + 1].1 {
                    path.push((s.0, s.1, s.0, s.1 - 1));
                    s.1 -= 1;
                }
            } else {
                while s.1 > points[i + 1].1 {
                    path.push((s.0, s.1, s.0, s.1 - 1));
                    s.1 -= 1;
                }
                while s.0 < points[i + 1].0 {
                    path.push((s.0, s.1, s.0 + 1, s.1));
                    s.0 += 1;
                }
            }
        }
    }

    println!("{}", path.len());
    for (a, b, c, d) in path {
        println!("{} {} {} {}", a + 1, b + 1, c + 1, d + 1);
    }
}
