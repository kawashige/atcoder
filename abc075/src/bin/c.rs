use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut count = vec![0; n];
    let mut seen = vec![false; n];
    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        count[a - 1] += 1;
        count[b - 1] += 1;
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let mut found = true;
    let mut r = 0;
    while found {
        found = false;
        for i in 0..n {
            if count[i] == 1 {
                found = true;
                seen[i] = true;
                count[i] = 0;
                r += 1;
                for next in &list[i] {
                    if seen[*next] {
                        continue;
                    }
                    count[*next] -= 1;
                }
            }
        }
    }

    println!("{}", r);
}
