use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: usize,
        mut k: Chars,
        b: [usize; n]
    }

    let mut seen = vec![-1; n];
    seen[a - 1] = 0;
    let mut v = vec![a - 1];
    let mut j = 0;

    for i in 1.. {
        let next = b[v[i - 1]] - 1;
        if seen[next] > -1 {
            j = seen[next];
            break;
        }
        v.push(next);
        seen[next] = i as i32;
    }

    let r = v.len() - j as usize;

    if k.len() < 11 {
        let mut i = k.into_iter().collect::<String>().parse::<usize>().unwrap();
        if i >= j as usize {
            i = (i - j as usize) % r;
            println!("{}", v[j as usize + i] + 1);
        } else {
            println!("{}", v[i] + 1);
        }
    } else {
        let mut x = 0;
        for c in k.into_iter() {
            x = x * 10 + c.to_digit(10).unwrap() as i32;
            x %= r as i32;
        }
        x -= j;
        while x < 0 {
            x += r as i32;
        }

        println!("{}", v[(j + x) as usize] + 1);
    }
}
