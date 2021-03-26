use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut num = 0;
    for i in 0..n {
        if a[i] == num + 1 {
            num += 1;
        }
    }

    if num == 0 {
        println!("-1");
    } else {
        println!("{}", n - num);
    }
}
