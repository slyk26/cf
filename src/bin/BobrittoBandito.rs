//IMPORTANT!! Submit Code Region Begin(Do not remove this line)

use std::io::stdin;

#[macro_export]
macro_rules! call {
    ($cnt:expr, $e:expr) => {
        let n: u32 = $cnt;
        for _ in 0..n {
            ($e)()
        }
    };
}

macro_rules! rl {
    ($t:ty ) => {
        {
            let mut ret = Vec::new();
            let mut input = String::new();
            let _ = stdin().read_line(&mut input);
            input.split(' ').for_each(|x| ret.push(x.trim().parse::<$t>().unwrap()));
            ret
        }
    };
}

fn main() {
    let mut out : Vec<String> = vec![];
    
    call!(rl!(u32)[0], || {
       let [n, m, l ,r]  = rl!(i32).try_into().unwrap();
        let mut l1 = 0;
        let mut r1 = 0;
        let mut is_l = false;

        for _ in 0..m {
            if is_l {
                if l1 - 1 < l {
                    r1 += 1;
                } else {
                    l1 -= 1;
                }
            } else {
                if r1 + 1 > r {
                    l1 -= 1;
                } else {
                    r1 += 1;
                }
            }
            is_l = !is_l;
        }
        out.push(format!("{} {}", l1, r1)); 
    });
    out.iter().for_each(|x| println!("{}", x));
}

//IMPORTANT!! Submit Code Region End(Do not remove this line)