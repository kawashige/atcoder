use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if s.len() == 1 {
        println!("YES");
        return;
    } else if s.len() == 2 {
        if s[0] == s[1] {
            println!("NO");
        } else {
            println!("YES");
        }
        return;
    }

    let mut count = s.into_iter().fold([0; 3], |mut count, c| {
        count[c as usize - 0x61] += 1;
        count
    });
    count.sort_unstable_by(|a, b| b.cmp(&a));

    if (count[0] == count[1] && count[0] == count[2])
        || (count[0] == count[1] && count[1] == count[2] + 1)
        || (count[0] == count[1] + 1 && count[1] == count[2])
    {
        println!("YES");
    } else {
        println!("NO");
    }
}
