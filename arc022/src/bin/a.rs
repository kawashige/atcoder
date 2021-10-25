use proconio::input;

fn main() {
    input! {
        s: String
    }

    let s = s.to_ascii_lowercase().chars().collect::<Vec<_>>();

    if let Some(i) = (0..s.len()).find(|i| s[*i] == 'i') {
        if let Some(j) = (i..s.len()).find(|j| s[*j] == 'c') {
            if let Some(_) = (j..s.len()).find(|k| s[*k] == 't') {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}
