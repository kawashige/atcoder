use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut candidate = vec![a[0], a[0] + 1];
    for i in 1..n {
        let mut new_candidate = Vec::new();
        let tmp = vec![a[i], a[i] + 1];

        for j in 0..candidate.len() {
            for k in 0..tmp.len() {
                if candidate[j] == tmp[k] {
                    new_candidate.push(candidate[j]);
                }
            }
        }

        candidate = new_candidate;

        if candidate.is_empty() {
            break;
        }
    }

    println!("{}", if candidate.is_empty() { "No" } else { "Yes" });
}
