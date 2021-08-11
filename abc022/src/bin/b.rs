use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut seen = vec![false; 100001];

    let mut c = 0;
    for x in a {
        if seen[x] {
            c += 1;
        }
        seen[x] = true;
    }

    println!("{}", c);
}
