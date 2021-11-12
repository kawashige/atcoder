use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut cards = Vec::new();

    let mut i = 0;
    while i < s.len() {
        cards.push(
            s[i..(i + if s[i + 1] == '1' { 3 } else { 2 })]
                .iter()
                .collect::<String>(),
        );
        if s[i + 1] == '1' {
            i += 3;
        } else {
            i += 2;
        }
    }

    // println!("{:?}", cards);

    let mut r = Vec::new();
    for m in ['S', 'H', 'D', 'C'].iter() {
        let targets = ["10", "J", "Q", "K", "A"]
            .iter()
            .map(|d| format!("{}{}", m, d))
            .collect::<Vec<_>>();
        let i = targets
            .iter()
            .map(|target| {
                (0..cards.len())
                    .find(|i| &cards[*i] == target)
                    .unwrap_or(std::usize::MAX)
            })
            .max()
            .unwrap();
        if i != std::usize::MAX {
            r.push(
                cards[0..i]
                    .iter()
                    .filter(|x| !targets.contains(x))
                    .cloned()
                    .collect::<String>(),
            );
        }
    }

    let r = r.iter().min_by_key(|x| x.len()).unwrap();
    println!("{}", if r.is_empty() { "0" } else { r });
}
