use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut count = vec![0; n];
    for i in 0..n {
        count[(i as i32 - (n - i - 1) as i32).abs() as usize] += 1;
    }

    let mut a_count = vec![0; n];
    for x in a {
        a_count[x] += 1;
    }

    for i in 0..n {
        if count[i] != a_count[i] {
            println!("0");
            return;
        }
    }

    let r = count
        .into_iter()
        .filter(|x| x != &0)
        .fold(1, |acc, x| acc * x % 1_000_000_007);

    println!("{:?}", r);
}
