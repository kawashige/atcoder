use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    let mut counts = vec![0; primes.len()];

    for i in 2..=n {
        let mut num = i;
        for j in 0..primes.len() {
            while num % primes[j] == 0 {
                counts[j] += 1;
                num /= primes[j];
            }
        }
    }

    let count_4 = counts.iter().filter(|c| c > &&3).count();
    let count_2 = counts.iter().filter(|c| c > &&1).count();
    let count_14 = counts.iter().filter(|c| c > &&13).count();
    let count_24 = counts.iter().filter(|c| c > &&23).count();
    let count_74 = counts.iter().filter(|c| c > &&73).count();

    let mut count = 0;

    if count_4 >= 2 && count_2 >= 3 {
        count += (count_2 - 2) * count_4 * (count_4 - 1) / 2;
    }
    if count_14 >= 1 && count_4 >= 2 {
        count += count_14 * (count_4 - 1);
    }
    if count_24 >= 1 && count_2 >= 2 {
        count += count_24 * (count_2 - 1);
    }
    if count_74 > 0 {
        count += count_74;
    }

    println!("{}", count);
}
