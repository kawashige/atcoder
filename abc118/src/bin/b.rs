use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize]; n]
    }

    let mut count = vec![0; m + 1];
    for x in a {
        for y in x {
            count[y] += 1;
        }
    }
    let sum = count.into_iter().filter(|c| c == &n).count();

    println!("{}", sum);
}
