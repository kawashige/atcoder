use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut prime = vec![true; n];
    prime[0] = false;
    if n > 1 {
        prime[1] = true;
    }

    let mut count = 0;

    for i in 2..prime.len() {
        if !prime[i] {
            continue;
        }
        count += 1;

        for j in ((i + i)..prime.len()).step_by(i) {
            prime[j] = false;
        }
    }

    println!("{}", count);
}
