use proconio::input;

fn main() {
    input! {
        x: String,
        n: usize,
        mut s: [String; n]
    }

    let map = x.chars().enumerate().fold(['a'; 26], |mut map, (i, c)| {
        map[c as usize - 0x61] = (0x61 + i as u8) as char;
        map
    });

    s.sort_unstable_by_key(|s| {
        s.chars()
            .map(|c| map[c as usize - 0x61])
            .collect::<String>()
    });

    for x in s {
        println!("{}", x);
    }
}
