use proconio::input;

fn dfs(chars: &mut Vec<char>, n: usize) {
    if chars.len() == n {
        println!("{}", chars.iter().collect::<String>());
        return;
    }

    chars.push('a');
    dfs(chars, n);
    chars.pop();

    chars.push('b');
    dfs(chars, n);
    chars.pop();

    chars.push('c');
    dfs(chars, n);
    chars.pop();
}

fn main() {
    input! {
        n: usize
    }

    dfs(&mut Vec::new(), n);
}
