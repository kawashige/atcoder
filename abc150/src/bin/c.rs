use proconio::input;

fn recurse(used: usize, n: usize, num: &mut String, nums: &mut Vec<String>) {
    if num.len() == n {
        nums.push(num.clone());
        return;
    }

    for i in 0..n {
        if used & 1 << i > 0 {
            continue;
        }

        num.push(('0' as u8 + i as u8 + 1) as char);
        recurse(used | 1 << i, n, num, nums);
        num.pop();
    }
}

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n]
    }

    let mut nums = Vec::new();
    recurse(0, n, &mut String::new(), &mut nums);
    nums.sort_unstable();

    let p = p
        .into_iter()
        .map(|i| ('0' as u8 + i as u8) as char)
        .collect::<String>();
    let q = q
        .into_iter()
        .map(|i| ('0' as u8 + i as u8) as char)
        .collect::<String>();

    let a = (0..nums.len()).find(|i| nums[*i] == p).unwrap();
    let b = (0..nums.len()).find(|i| nums[*i] == q).unwrap();

    println!("{}", (a as i32 - b as i32).abs());
}
