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
       let [a, b, c] = rl!(String).try_into().unwrap();
        out.push(format!("{}{}{}", a.chars().next().unwrap(), b.chars().next().unwrap(), c.chars().next().unwrap())) 
    });
    out.iter().for_each(|o| println!("{}", o));
}

//IMPORTANT!! Submit Code Region End(Do not remove this line)