use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let acc = a
        .iter()
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect::<Vec<usize>>();
    let max = a
        .iter()
        .scan(0, |max, x| {
            if x > max {
                *max = *x;
            }
            Some(*max)
        })
        .collect::<Vec<usize>>();

    let mut sum = 0;
    for i in 0..n {
        sum += acc[i];
        println!("{}", sum + max[i] * (i + 1));
    }
}
