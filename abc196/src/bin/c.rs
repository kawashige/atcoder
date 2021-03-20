use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut sum = 0;

    for i in 1..n {
        let j = format!("{}{}", i, i).parse::<u64>().unwrap();
        if n < j {
            break;
        }
        sum += 1;
    }

    println!("{}", sum);
}
