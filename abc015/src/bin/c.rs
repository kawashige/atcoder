use proconio::input;

fn dfs(i: usize, x: usize, t: &Vec<Vec<usize>>) -> bool {
    if i == t.len() {
        return x == 0;
    }

    for j in 0..t[i].len() {
        if dfs(i + 1, x ^ t[i][j], t) {
            return true;
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; k]; n]
    }

    if dfs(0, 0, &t) {
        println!("Found");
    } else {
        println!("Nothing");
    }
}
