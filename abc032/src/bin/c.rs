use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        s: [u64; n]
    }

    if s.iter().any(|x| x == &0) {
        println!("{}", n);
        return;
    }

    let mut m = s[0];
    let mut max = if s[0] <= k { 1 } else { 0 };
    let mut j = 0;

    for i in 1..n {
        m *= s[i];
        while j < i && m > k {
            m /= s[j];
            j += 1;
        }

        if m <= k {
            max = max.max(i - j + 1);
        }
    }

    println!("{}", max);
}
