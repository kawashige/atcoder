use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(i32, i32); n],
        cd: [(i32, i32); m],
    }

    for i in 0..n {
        let mut min_j = 0;
        let mut min_d = std::i32::MAX;
        for j in 0..m {
            let d = (ab[i].0 - cd[j].0).abs() + (ab[i].1 - cd[j].1).abs();
            if min_d > d {
                min_j = j;
                min_d = d;
            }
        }

        println!("{}", min_j + 1);
    }
}
