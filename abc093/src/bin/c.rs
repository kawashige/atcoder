use proconio::input;

fn main() {
    input! {
        mut a: [usize; 3]
    }

    a.sort_unstable();

    let mut count = 0;
    while a[0] + 2 <= a[2] {
        a[0] += 2;
        count += 1;
    }
    while a[1] + 2 <= a[2] {
        a[1] += 2;
        count += 1;
    }

    if a[0] == a[2] - 1 && a[1] == a[2] - 1 {
        count += 1;
    } else if a[0] == a[2] - 1 || a[1] == a[2] - 1 {
        count += 2;
    }

    println!("{:?}", count)
}
