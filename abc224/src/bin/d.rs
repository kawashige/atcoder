use std::collections::{HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        m: usize,
        uv: [(usize, usize); m],
        p: [usize; 8]
    }

    let mut list = vec![vec![]; 9];
    for (u, v) in uv {
        list[u - 1].push(v - 1);
        list[v - 1].push(u - 1);
    }

    let mut state = [8; 9];
    for i in 0..p.len() {
        state[p[i] - 1] = i;
    }
    let blank = (0..state.len()).find(|i| state[*i] == 8).unwrap();

    let mut queue = VecDeque::new();
    queue.push_back((0, blank, state));

    let mut seen = HashSet::new();

    while let Some((m, b, s)) = queue.pop_front() {
        let state_string = s.iter().map(|i| i.to_string()).collect::<String>();
        if state_string == "012345678" {
            println!("{}", m);
            return;
        }
        if seen.contains(&state_string) {
            continue;
        }
        seen.insert(state_string);

        for next in &list[b] {
            let mut next_state = s.clone();
            next_state.swap(b, *next);
            queue.push_back((m + 1, *next, next_state));
        }
    }

    println!("-1");
}
