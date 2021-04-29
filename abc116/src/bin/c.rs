use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [usize; n]
    }

    let mut r = 0;
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..n {
            if h[i] == 0 {
                continue;
            }

            changed = true;

            let min = *h[i..].iter().take_while(|x| x > &&0).min().unwrap();

            r += min;
            for j in i..n {
                if h[j] == 0 {
                    break;
                }
                h[j] -= min;
            }
            break;
        }
    }

    println!("{}", r);
}
