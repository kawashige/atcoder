use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let num = format!("{}{}", a, b).parse::<usize>().unwrap();
    for i in 1..=((num as f64).sqrt() as usize) {
        if i * i == num {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
