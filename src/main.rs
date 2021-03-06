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
            let size = 1 + rng.gen::<usize>() % (n - 1);
            let mut a = Vec::with_capacity(size);
            for _ in 0..size {
                a.push(rng.gen::<usize>() % n)
            }
            a.sort();
            a.dedup();
            u.push(a)
        }
    }
    u.sort();
    println!("U:");
    for (i, a) in u.iter().enumerate() {
        println!("{i}: {a:?}")
    }
    let u = u;

    // Compute <_U by <_Vector
    let order_u;
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
        order_u = u_s.into_iter().map(|(_, i)| i).collect::<Vec<_>>();
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
                ans_c.insert(ans_i.clone());
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
            ans_s.insert(c.clone());
        }
        println!("\nS:");
        for (i, s) in ans_s.iter().enumerate() {
            println!("{i}: {s:?}")
        }
    }
    let mut ans_s = ans_s;

    // Compute S_bar
    println!("\nS_bar:");
    {
        let mut ans_s_bar = HashSet::new();
        for s in ans_s {
            let mut bar = vec![true; m];
            for i in 0..m {
                if s.contains(&i) {
                    bar[i] = false;
                }
            }
            let s_bar = bar
                .iter()
                .enumerate()
                .filter(|(_, &e)| e)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            ans_s_bar.insert(s_bar);
        }
        ans_s = ans_s_bar;
        for (i, s) in ans_s.iter().enumerate() {
            println!("{i}: {s:?}")
        }
    }

    // Compute pi(S)
    let mut pi_i = Vec::with_capacity(ans_s.len());
    {
        println!();
        for (i, s) in ans_s.iter().enumerate() {
            let mut pi = Vec::new();
            print!("?? (S_{i}): ");
            for i in &order_u {
                if s.contains(i) {
                    pi.push(*i);
                    print!("{i} ");
                }
            }
            pi_i.push(pi);
            println!()
        }
    }
    let pi_i = pi_i;

    // Compute P_S*(S) + cx_S*(S)
    {
        println!("\ncxs:");
        for (i, s) in ans_s.iter().enumerate() {
            'next: for j in 0..ans_s.len() {
                if i == j {
                    continue;
                }
                print!("cx(S_{i}, S_{j}): {{");
                for p in &pi_i[j] {
                    if !s.contains(p) {
                        println!("{p}}}");
                        continue 'next;
                    }
                    print!("{p} ")
                }
                println!("}}")
            }
            println!()
        }
    }
}
