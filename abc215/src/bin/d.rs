use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }

    let mut nums = vec![true; 100001];
    nums[0] = false;
    for i in 0..n {
        nums[a[i]] = false;
    }

    let mut nums2 = vec![true; m + 1];

    let mut r = vec![1];
    for i in 2..=m {
        if !nums2[i] {
            continue;
        }
        if (i..nums.len()).step_by(i).all(|j| nums[j]) {
            r.push(i);
        } else {
            (i..=m).step_by(i).for_each(|j| nums2[j] = false);
        }
    }

    println!("{}", r.len());
    println!(
        "{}",
        r.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
