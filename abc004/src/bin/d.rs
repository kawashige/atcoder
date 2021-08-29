use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize
    }

    let mut min = std::usize::MAX;
    let mut acc = vec![0; 601];
    for i in 1..acc.len() {
        acc[i] = i + acc[i - 1];
    }

    for i in 0..(g - 1) {
        let mut c = 0;

        // g
        c += acc[i] + acc[g - 1 - i];

        // r
        c += if i <= 99 {
            let right = (99 - i).min((r - 1) / 2);
            acc[right] + acc[r - 1 - right]
        } else {
            acc[r + (i - 100)] - acc[i - 100]
        };

        // b
        let j = g - 1 - i;
        c += if j <= 99 {
            let right = (99 - j).min((b - 1) / 2);
            acc[right] + acc[b - 1 - right]
        } else {
            acc[b + (j - 100)] - acc[j - 100]
        };

        min = min.min(c);
    }

    println!("{}", min);
}
