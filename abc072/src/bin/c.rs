use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut count = vec![0; 100000];
    for x in a {
        count[x as usize] += 1;
    }

    let mut max = 0;
    for i in 1..(count.len() - 1) {
        max = std::cmp::max(max, count[i - 1] + count[i] + count[i + 1]);
    }

    println!("{}", max);
}
