use proconio::input;

fn main() {
    input! {
        _n: usize,
        a: usize,
        b: usize,
        k: usize,
        p: [usize; k]
    }

    let mut seen = vec![false; 101];
    seen[a] = true;
    seen[b] = true;

    for x in p {
        if seen[x] {
            println!("NO");
            return;
        }
        seen[x] = true;
    }

    println!("YES");
}
