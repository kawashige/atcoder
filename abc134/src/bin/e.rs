use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut nums = vec![a[n - 1]];

    for i in (0..(n - 1)).rev() {
        match nums.binary_search(&(a[i] + 1)) {
            Ok(j) => {
                nums[j] = a[i];
            }
            Err(j) => {
                if j < nums.len() {
                    nums[j] = a[i];
                } else {
                    nums.push(a[i]);
                }
            }
        }
    }

    println!("{}", nums.len());
}
