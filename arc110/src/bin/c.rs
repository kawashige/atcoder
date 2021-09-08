use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let mut b = a.clone();
    let mut r = Vec::with_capacity(n);
    let mut used = vec![false; n];

    for i in 0..(n - 1) {
        for j in (0..=i).rev() {
            if b[j] < b[j + 1] {
                break;
            }
            if used[j] {
                println!("-1");
                return;
            }
            b.swap(j, j + 1);
            r.push(j);
            used[j] = true;
        }
    }

    if (0..(n - 1)).any(|i| !used[i] || b[i] > b[i + 1]) {
        println!("-1");
    } else {
        for x in r {
            println!("{}", x + 1);
        }
    }
}
