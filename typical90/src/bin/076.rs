use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }

    let sum = a.iter().sum::<u64>();
    if sum % 10 != 0 {
        println!("No");
        return;
    }
    let target = sum / 10;

    let mut remains = target;
    for i in 0..n {
        if remains > a[i] {
            remains -= a[i];
            a.push(a[i]);
        } else {
            break;
        }
    }

    let mut i = 0;
    let mut sum = a[0];

    if sum == target {
        println!("Yes");
        return;
    }

    for j in 1..a.len() {
        sum += a[j];
        while sum > target {
            sum -= a[i];
            i += 1;
        }
        if sum == target {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
