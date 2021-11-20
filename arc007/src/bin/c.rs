use proconio::input;
use proconio::marker::Chars;

fn dfs(i: usize, c: &Vec<char>, can_see: usize, used: usize, min_count: &mut usize) {
    if (0..c.len()).all(|i| can_see & 1 << i != 0) {
        *min_count = std::cmp::min(*min_count, used.count_ones() as usize);
        return;
    }

    if i < c.len() {
        dfs(i + 1, c, can_see, used, min_count);

        let mut new_can_see = can_see;
        for j in 0..c.len() {
            if c[j] == 'o' {
                new_can_see |= 1 << (i + j) % c.len();
            }
        }
        dfs(i + 1, c, new_can_see, used | 1 << i, min_count);
    }
}

fn main() {
    input! {
        c: Chars
    }

    let mut can_see = 0;
    for i in 0..c.len() {
        if c[i] == 'o' {
            can_see |= 1 << i;
        }
    }
    let mut min_count = std::usize::MAX;

    dfs(1, &c, can_see, 1, &mut min_count);

    println!("{}", min_count);
}
