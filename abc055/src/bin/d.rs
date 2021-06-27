use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        mut s: Chars
    }

    s.push(s[0]);

    let target = if s[1] == 'o' {
        vec![
            vec!['S', 'S', 'S'],
            vec!['W', 'S', 'W'],
            vec!['S', 'W', 'W'],
            vec!['W', 'W', 'S'],
        ]
    } else {
        vec![
            vec!['S', 'S', 'W'],
            vec!['W', 'S', 'S'],
            vec!['S', 'W', 'S'],
            vec!['W', 'W', 'W'],
        ]
    };

    for t in target {
        let mut t = t;
        for i in 2..s.len() {
            if s[i] == 'o' {
                if t[t.len() - 1] == t[t.len() - 2] {
                    t.push('S');
                } else {
                    t.push('W');
                }
            } else {
                if t[t.len() - 1] == t[t.len() - 2] {
                    t.push('W');
                } else {
                    t.push('S');
                }
            }
        }

        if t[0] == t[t.len() - 2] && t[1] == t[t.len() - 1] {
            println!("{}", t[..(t.len() - 2)].into_iter().collect::<String>());
            return;
        }
    }

    println!("-1");
}
