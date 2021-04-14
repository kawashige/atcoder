use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        l: [usize; n]
    }

    let mut count = 1;
    let mut j = 0;
    for i in 0..n {
        j += l[i];
        if j > x {
            break;
        }
        count += 1;
    }

    println!("{}", count);
}
