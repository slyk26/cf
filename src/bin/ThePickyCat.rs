//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
use std::io::stdin;

#[allow(duplicate_macro_attributes)]
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
        let n = rl!(usize)[0];
        let a = rl!(i32).iter().map(|x| x.abs()).collect::<Vec<i32>>();
        
        if a[0] <= median(&a) {
            out.push("YES".to_string());
        } else {
            out.push("NO".to_string());
        }
    });
    out.iter().for_each(|x| println!("{}", x));
}

fn median(a: &Vec<i32>) -> i32 {
    let mut b = a.clone();
    b.sort();
    let x = a.len() as f64 / 2.0;
    b[x.floor() as usize]
}

//IMPORTANT!! Submit Code Region End(Do not remove this line)