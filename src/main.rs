use rand::Rng;
use std::collections::HashSet;

fn main() {
    // |A|
    let n: usize = 7;
    // |U|
    let m: usize = 8;
    // U
    let mut u = Vec::with_capacity(m);
    {
        let mut rng = rand::thread_rng();
        for _ in 0..m {
            let size = rng.gen::<usize>() % n;
            let mut a = Vec::with_capacity(size);
            for _ in 0..size {
                a.push(rng.gen::<usize>() % n)
            }
            a.sort();
            a.dedup();
            u.push(a)
        }
    }
    println!("U:");
    for (i, a) in u.iter().enumerate() {
        println!("{i}: {a:?}")
    }
    let u = u;

    // Compute <_U
    {
        let mut u_s = u
            .iter()
            .enumerate()
            .map(|(i, e)| (e, i))
            .collect::<Vec<_>>();
        u_s.sort();
        println!(
            "<_U: {}",
            u_s.iter()
                .map(|(_, i)| i.to_string())
                .collect::<Vec<_>>()
                .join(" < ")
        );
    }

    // Compute C
    let mut ans_c = HashSet::new();
    {
        let all = (0..n).collect::<HashSet<usize>>();
        for t in 0..1 << m {
            let mut set = HashSet::new();
            let mut ans_i = Vec::new();
            let mut ans = Vec::new();
            for i in 0..m {
                if t >> i & 1 == 1 {
                    ans_i.push(i);
                    ans.push(u[i].clone());
                    for e in &u[i] {
                        set.insert(*e);
                    }
                }
            }
            if set == all {
                ans_c.insert(ans.clone());
                // println!("{ans_i:?}: {ans:?}");
            }
        }
    }
    let ans_c = ans_c;

    // Compute S
    let mut ans_s = HashSet::new();
    {
        'out: for c in &ans_c {
            let size_c = c.len();
            for t in 0..(1 << size_c) - 1 {
                let sub = {
                    let mut s = Vec::new();
                    for i in 0..size_c {
                        if t >> i & 1 == 1 {
                            s.push(c[i].clone());
                        }
                    }
                    s
                };
                if ans_c.contains(&sub) {
                    continue 'out;
                }
            }
            ans_s.insert(c);
        }
        println!("\nS:");
        for (i, s) in ans_s.iter().enumerate() {
            println!("{i}: {s:?}")
        }
    }
}
