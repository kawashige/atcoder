use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    }

    let d = t.as_bytes()[0] as i32 - s.as_bytes()[0] as i32;
    let new_s = s
        .chars()
        .map(|c| (((c as i32 - 0x61 + 26 + d) % 26) as u8 + 0x61) as char)
        .collect::<String>();

    if new_s == t {
        println!("Yes");
    } else {
        println!("No");
    }
}
