use proconio::input;

fn main() {
    input! {
        c: [[String; 4]; 4]
    }

    for i in (0..4).rev() {
        println!(
            "{}",
            c[i].clone()
                .into_iter()
                .rev()
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
