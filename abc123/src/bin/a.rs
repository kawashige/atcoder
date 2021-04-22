use proconio::input;

fn main() {
    input! {
        v: [i32; 6]
    }

    for i in 0..5 {
        for j in i..5 {
            if (v[i] - v[j]).abs() > v[5] {
                println!(":(");
                return;
            }
        }
    }
    println!("Yay!");
}
