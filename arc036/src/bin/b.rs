use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n]
    }

    if h.len() == 1 {
        println!("1");
        return;
    }

    let mut signum = (h[1] - h[0]).signum();
    let mut count = 2;
    let mut max = 2;

    for i in 2..n {
        let current_signum = (h[i] - h[i - 1]).signum();
        if signum == -1 && current_signum == 1 {
            count = 2;
        } else {
            count += 1;
            max = max.max(count);
        }
        signum = current_signum;
    }

    println!("{}", max);
}
