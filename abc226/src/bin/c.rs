use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut list = vec![vec![]; n];
    let mut time = vec![0; n];

    for i in 0..n {
        input! {
            t: u64,
            k: usize,
            a: [usize; k]
        }

        time[i] = t;
        list[i] = a;
    }

    let mut seen = vec![false; n];
    let mut stack = vec![n - 1];
    let mut t = 0;

    while let Some(x) = stack.pop() {
        if seen[x] {
            continue;
        }
        seen[x] = true;
        t += time[x];

        for next in &list[x] {
            if seen[next - 1] {
                continue;
            }
            stack.push(next - 1);
        }
    }

    println!("{}", t);
}
