use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        x: usize,
        m: usize
    }

    let mut a = x;
    let mut map = HashMap::new();
    let mut nums = vec![x];
    for i in 1..n {
        a = (a * a) % m;
        if map.contains_key(&a) {
            break;
        } else {
            map.insert(a, i);
            nums.push(a);
        }
    }

    if nums.len() == n {
        println!("{}", nums.iter().sum::<usize>());
    } else {
        let i = *map.get(&a).unwrap();
        let j = (n - i) / (nums.len() - i);
        let result = nums[..i].iter().sum::<usize>()
            + j * nums[i..].iter().sum::<usize>()
            + nums[i..(i + (n - i - j * (nums.len() - i)))]
                .iter()
                .sum::<usize>();

        println!("{}", result);
    }
}
