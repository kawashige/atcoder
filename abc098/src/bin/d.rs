use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut r = 0;
    let mut count = vec![0; 20];
    let mut j = 0;
    for i in 0..n {
        for k in 0..20 {
            if a[i] & 1 << k > 0 {
                count[k] += 1;
            }
        }

        while count.iter().any(|x| x > &1) {
            for k in 0..20 {
                if a[j] & 1 << k > 0 {
                    count[k] -= 1;
                }
            }
            j += 1;
        }

        r += i - j + 1;
    }

    println!("{}", r)
}
