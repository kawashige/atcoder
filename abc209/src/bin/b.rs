use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }

    let sum = a
        .into_iter()
        .enumerate()
        .map(|(i, x)| if i % 2 == 0 { x } else { x - 1 })
        .sum::<usize>();

    if sum <= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
