use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n]
    }

    let r = w
        .into_iter()
        .map(|s| {
            s.to_ascii_lowercase()
                .chars()
                .map(|c| match c {
                    'b' | 'c' => '1',
                    'd' | 'w' => '2',
                    't' | 'j' => '3',
                    'f' | 'q' => '4',
                    'l' | 'v' => '5',
                    's' | 'x' => '6',
                    'p' | 'm' => '7',
                    'h' | 'k' => '8',
                    'n' | 'g' => '9',
                    'z' | 'r' => '0',
                    _ => 'x',
                })
                .filter(|c| c != &'x')
                .collect::<String>()
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", r);
}
