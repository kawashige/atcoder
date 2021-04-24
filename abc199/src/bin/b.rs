use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let min = a.iter().max().unwrap();
    let max = b.iter().min().unwrap();

    if max < min {
        println!("0");
    } else {
        println!("{}", max - min + 1);
    }
}
