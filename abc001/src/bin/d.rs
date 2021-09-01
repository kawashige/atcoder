use proconio::input;

fn main() {
    input! {
        n: usize,
        mut se: [String; n]
    }

    se.sort_unstable();

    se.into_iter()
        .fold(
            Vec::new(),
            |mut ranges: Vec<((usize, usize), (usize, usize))>, se| {
                let ((sh, sm), (eh, em)) = {
                    let v = se
                        .split("-")
                        .map(|t| {
                            (
                                t[..2].parse::<usize>().unwrap(),
                                t[2..].parse::<usize>().unwrap(),
                            )
                        })
                        .collect::<Vec<_>>();
                    (
                        (
                            v[0].0,
                            (v[0].1 / 10) * 10 + if v[0].1 % 10 < 5 { 0 } else { 5 },
                        ),
                        (
                            v[1].0 + if v[1].1 > 55 { 1 } else { 0 },
                            ((v[1].1 / 10) * 10
                                + if v[1].1 % 10 == 0 {
                                    0
                                } else if v[1].1 % 10 <= 5 {
                                    5
                                } else {
                                    10
                                })
                                % 60,
                        ),
                    )
                };

                let l = ranges.len();
                if !ranges.is_empty()
                    && (ranges[l - 1].1 .0 > sh
                        || (ranges[l - 1].1 .0 == sh && ranges[l - 1].1 .1 >= sm))
                {
                    if ranges[l - 1].1 .0 < eh
                        || (ranges[l - 1].1 .0 == eh && ranges[l - 1].1 .1 < em)
                    {
                        ranges[l - 1].1 = (eh, em);
                    }
                } else {
                    ranges.push(((sh, sm), (eh, em)));
                }

                ranges
            },
        )
        .into_iter()
        .for_each(|(s, e)| println!("{:<02}{:<02}-{:<02}{:<02}", s.0, s.1, e.0, e.1));
}
