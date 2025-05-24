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
        let s = rl!(String)[0].chars().collect::<Vec<char>>();
        let mut result = vec![vec!['0';n];n];

        for i in 0..n {
            let mut x = s.clone();
            if x[i] == '1' {
                x[i] = '0';
            } else {
                x[i] = '1';
            }
            result[i] = x;
        }
        out.push(format!("{}", result.iter().flatten().filter(|x| **x == '1').count()));
    });
    out.iter().for_each(|x| println!("{}", x));
}

//IMPORTANT!! Submit Code Region End(Do not remove this line)