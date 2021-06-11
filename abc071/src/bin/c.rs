use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.sort_unstable_by(|a, b| b.cmp(&a));

    let mut i = 0;
    while i < a.len() - 1 && a[i] != a[i + 1] {
        i += 1;
    }

    if i == a.len() {
        println!("0");
        return;
    }

    let d1 = a[i];
    i += 2;

    while i < a.len() - 1 && a[i] != a[i + 1] {
        i += 1;
    }

    if i > a.len() - 2 {
        println!("0");
        return;
    }

    println!("{}", d1 * a[i])
}
