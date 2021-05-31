use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        x: i32,
        y: i32
    }
    let len = s.len();

    let mut p_x = vec![false; len * 2 + 1];
    let mut p_y = vec![false; len * 2 + 1];

    let mut i = 0;
    let mut x_s = 0;
    while i < s.len() && s[i] == 'F' {
        x_s += 1;
        i += 1;
    }

    p_x[x_s + len] = true;
    p_y[len] = true;

    let mut nums = vec![vec![]; 2];
    let mut index = 0;
    let mut count = 0;
    for j in i..s.len() {
        if s[j] == 'T' {
            if count > 0 {
                nums[index].push(count);
                count = 0;
            }
            index = (index + 1) % 2;
        } else {
            count += 1;
        }
    }
    if count > 0 {
        nums[index].push(count);
    }

    nums[0].sort_unstable();
    nums[1].sort_unstable();

    for i in 0..nums[0].len() {
        let mut new_p_x = vec![false; p_x.len()];
        for j in 0..p_x.len() {
            if p_x[j] {
                if j >= nums[0][i] {
                    new_p_x[j - nums[0][i]] = true;
                }
                if j + nums[0][i] < p_x.len() {
                    new_p_x[j + nums[0][i]] = true;
                }
            }
        }
        p_x = new_p_x;
    }
    for i in 0..nums[1].len() {
        let mut new_p_y = vec![false; p_y.len()];
        for j in 0..p_y.len() {
            if p_y[j] {
                if j > nums[1][i] {
                    new_p_y[j - nums[1][i]] = true;
                }
                if j + nums[1][i] < p_y.len() {
                    new_p_y[j + nums[1][i]] = true;
                }
            }
        }
        p_y = new_p_y;
    }

    if p_x[(x + len as i32) as usize] && p_y[(y + len as i32) as usize] {
        println!("Yes");
    } else {
        println!("No");
    }
}
