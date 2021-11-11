use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut c: Chars
    }

    if (c.len() == 1 && c[0] == 'a') || (c.len() == 20 && c.iter().all(|x| x == &'z')) {
        println!("NO");
        return;
    }

    if c.iter().all(|x| x == &'a') {
        c.pop();
        c.pop();
        c.push('b');
    } else if c.len() < 20 {
        let i = (0..c.len()).find(|i| c[*i] != 'a').unwrap();
        c[i] = (c[i] as u8 - 1) as char;
        c.push('a');
    } else {
        for i in 0..c.len() {
            for j in (i + 1)..c.len() {
                if 'a' < c[i] && c[j] < 'z' {
                    c[i] = (c[i] as u8 - 1) as char;
                    c[j] = (c[j] as u8 + 1) as char;
                    println!("{}", c.iter().collect::<String>());
                    return;
                } else if 'a' < c[j] && c[i] < 'z' {
                    c[j] = (c[j] as u8 - 1) as char;
                    c[i] = (c[i] as u8 + 1) as char;
                    println!("{}", c.iter().collect::<String>());
                    return;
                }
            }
        }
    }
    println!("{}", c.iter().collect::<String>());
}
