use proconio::input;

fn dfs(chars: &mut Vec<char>, l: usize, n: usize, count: &mut usize) {
    if chars.contains(&'7')
        && chars.contains(&'5')
        && chars.contains(&'3')
        && chars.iter().collect::<String>().parse::<usize>().unwrap() <= n
    {
        *count += 1;
    }

    if chars.len() < l {
        chars.push('3');
        dfs(chars, l, n, count);
        chars.pop();

        chars.push('5');
        dfs(chars, l, n, count);
        chars.pop();

        chars.push('7');
        dfs(chars, l, n, count);
        chars.pop();
    }
}

fn main() {
    input! {
        n: usize
    }

    let l = n.to_string().len();
    let mut count = 0;
    dfs(&mut Vec::with_capacity(l), l, n, &mut count);

    println!("{}", count);
}
