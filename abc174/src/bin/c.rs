use proconio::input;

fn main() {
    input! {
        k: usize
    }

    if k % 2 == 0 {
        println!("-1")
    } else {
        let mut n = k;
        let mut count = 0;
        while n != 0 {
            count += 1;
            let mut found = false;
            for i in 0..10 {
                if (n + k * i) % 10 == 7 {
                    n = (n + k * i) / 10;
                    found = true;
                    break;
                }
            }
            if !found {
                println!("-1");
                return;
            }
        }
        println!("{}", count);
    }
}
