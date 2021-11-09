use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut prev = vec![n; n];
    let mut next = vec![n; n];

    for _ in 0..q {
        input! {
            t: usize
        }

        if t == 1 {
            input! {
                x: usize,
                y: usize
            }
            prev[y - 1] = x - 1;
            next[x - 1] = y - 1;
        } else if t == 2 {
            input! {
                x: usize,
                y: usize
            }
            prev[y - 1] = n;
            next[x - 1] = n;
        } else {
            input! {
                x: usize,
            }
            let mut i = x - 1;
            while prev[i] != n {
                i = prev[i];
            }
            let mut v = Vec::new();
            while i != n {
                v.push((i + 1).to_string());
                i = next[i];
            }
            println!("{} {}", v.len(), v.join(" "));
        }
    }
}
