use std::collections::BTreeSet;

use proconio::input;

const M: usize = 1_000_000_007;

fn mul(matrix1: &Vec<Vec<usize>>, matrix2: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; matrix1[0].len()]; matrix1.len()];

    for i in 0..matrix1.len() {
        for j in 0..matrix2[0].len() {
            for k in 0..matrix1[0].len() {
                result[i][j] += (matrix1[i][k] * matrix2[k][j]) % M;
                result[i][j] %= M;
            }
        }
    }

    result
}

fn mod_pow(matrix: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    if n == 1 {
        matrix.clone()
    } else if n % 2 == 0 {
        let r = mod_pow(matrix, n / 2);
        mul(&r, &r)
    } else {
        mul(matrix, &mod_pow(matrix, n - 1))
    }
}

fn main() {
    input! {
        n: usize,
        m: usize
    }

    let mut nums = BTreeSet::new();
    for i in 1..=((m as f64).sqrt() as usize) {
        if m % i == 0 {
            nums.insert(i);
            nums.insert(m / i);
        }
    }

    let nums = nums.into_iter().collect::<Vec<_>>();

    let mut matrix = vec![vec![0; nums.len()]; nums.len()];
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[j] % nums[i] == 0 {
                matrix[i][j] = 1;
            }
        }
    }

    let matrix2 = mod_pow(&matrix, n - 1);
    let r = matrix2[0].iter().fold(0, |acc, x| (acc + x) % M);

    println!("{}", r);
}
