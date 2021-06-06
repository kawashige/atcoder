use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let num = a
        .into_iter()
        .map(|x| if x <= 10 { 0 } else { x - 10 })
        .sum::<usize>();
    println!("{}", num);
}
