use proconio::input;

struct Bit {
    n: usize,
    pub data: Vec<usize>,
}

impl Bit {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0; n + 1],
        }
    }

    pub fn sum(&self, i: usize) -> usize {
        let mut r = 0;
        let mut i = i as i32;
        while i > 0 {
            r += self.data[i as usize];
            i -= i & -i;
        }
        r
    }

    pub fn add(&mut self, i: usize, x: usize) {
        let mut i = i as i32;
        while i as usize <= self.n {
            self.data[i as usize] += x;
            i += i & -i;
        }
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut bit = Bit::new(n + 1);
    let mut count = 0;
    for i in 0..n {
        bit.add(a[i] + 1, 1);
        count += i - bit.sum(a[i]);
    }
    println!("{}", count);

    for i in 0..(n - 1) {
        count += n - 1;
        count -= a[i] * 2;
        println!("{}", count);
    }
}
