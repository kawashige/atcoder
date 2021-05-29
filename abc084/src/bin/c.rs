use proconio::input;

fn main() {
    input! {
        n: usize,
        csf: [(usize, usize, usize); n - 1]
    }

    let mut times = vec![0; n];

    for i in 0..(n - 1) {
        for j in 0..=i {
            times[j] = csf[i].0
                + if times[j] <= csf[i].1 {
                    csf[i].1
                } else {
                    csf[i].2 * ((times[j] + csf[i].2 - 1) / csf[i].2)
                };
        }
    }

    for t in times {
        println!("{}", t);
    }
}
