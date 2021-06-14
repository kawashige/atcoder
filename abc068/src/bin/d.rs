use proconio::input;

fn main() {
    input! {
        k: u64
    }

    println!("50");

    let mut a: Vec<u64>;

    if k < 50 {
        a = vec![0; 50];
        for i in 0..k {
            a[i as usize] = 50;
        }
    } else {
        a = vec![50 + (k / 50) - 1; 50];
        for i in 0..50 {
            if i < k % 50 {
                a[i as usize] += 51 - k % 50;
            } else {
                a[i as usize] -= k % 50;
            }
        }
    }

    println!(
        "{}",
        a.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
