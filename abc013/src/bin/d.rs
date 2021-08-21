use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: usize,
        a: [usize; m]
    }

    let mut dest = (0..n).collect::<Vec<_>>();
    for x in a {
        dest.swap(x - 1, x);
    }

    let mut dest2 = vec![0; n];
    for i in 0..n {
        dest2[dest[i]] = i;
    }

    let mut r = vec![0; n];
    let mut seen = vec![false; n];

    for i in 0..n {
        if seen[i] {
            continue;
        }

        let mut v = Vec::new();
        let mut idx = i;
        while !seen[idx] {
            seen[idx] = true;
            v.push(idx);
            idx = dest2[idx];
        }

        for i in 0..v.len() {
            r[v[i]] = v[(i + d % v.len()) % v.len()];
        }
    }

    for x in r {
        println!("{}", x + 1);
    }
}
