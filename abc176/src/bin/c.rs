use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut sum = 0;
    let mut max = 0;
    for n in a {
        if n < max {
            sum += max - n;
        } else if max < n {
            max = n;
        }
    }
    println!("{}", sum);
}
