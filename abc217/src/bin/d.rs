use proconio::input;
fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize, usize); q]
    }

    let mut r = Vec::with_capacity(q);
    let mut len = Vec::with_capacity(q + 2);
    len.push(0);
    for i in 0..q {
        if cx[i].0 == 1 {
            len.push(cx[i].1);
        }
    }
    len.push(l);
    len.sort_unstable();

    for i in (0..q).rev() {
        let mut ng = 0;
        let mut ok = len.len() - 1;

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if len[mid] >= cx[i].1 {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        if cx[i].0 == 2 {
            r.push(len[ok] - len[ok - 1]);
        } else {
            len.remove(ok);
        }
    }

    for x in r.into_iter().rev() {
        println!("{}", x);
    }
}
