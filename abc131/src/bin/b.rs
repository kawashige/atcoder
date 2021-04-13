use proconio::input;

fn main() {
    input! {
        n: i32,
        l: i32
    }

    let tastes = (1..=n).map(|i| l + i - 1).collect::<Vec<i32>>();
    let sum = tastes.iter().sum::<i32>();
    let min = tastes.iter().min_by_key(|a| a.abs()).unwrap();

    println!("{}", sum - min);
}
