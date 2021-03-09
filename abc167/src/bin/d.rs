use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut seen = vec![0; n];
    let mut current = 0;
    let mut count = 0;
    let mut cycle_found = false;

    while count < k {
        if !cycle_found && 0 < seen[current] {
            cycle_found = true;
            let l = count - seen[current];
            count = k - (k - count) % l;
        } else {
            seen[current] = count;
            current = a[current] - 1;
            count += 1;
        }
    }

    println!("{}", current + 1);
}
