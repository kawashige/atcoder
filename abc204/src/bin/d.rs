use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n]
    }

    let sum = t.iter().sum::<usize>();
    let mut times = vec![false; sum + 1];
    times[0] = true;
    for i in 0..n {
        for j in (0..times.len()).rev() {
            if times[j] && j + t[i] < times.len() {
                times[j + t[i]] = true;
            }
        }
    }

    let mut min = sum;
    for i in (0..sum).rev() {
        if times[i] {
            min = std::cmp::min(min, (sum - i).max(i));
        }
    }

    println!("{}", min);
}
