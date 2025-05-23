//IMPORTANT!! Submit Code Region Begin(Do not remove this line)

use std::io::stdin;
#[macro_export]
macro_rules! rl {
    ( $cnt:literal, $t:ty ) => {
        {
            let _ :u32 = $cnt;
            let mut ret = Vec::new();
            for _ in 0..$cnt {
                let mut input = String::new();
                let _ = stdin().read_line(&mut input);
                input.split(' ').for_each(|x| ret.push(x.trim().parse::<$t>().unwrap()) )
            }
            ret
        }
    };
}

fn main() {
    let t = rl!(1, u32)[0];
    let mut out : Vec<String> = vec![];
    
    for _ in 0..t {
        let [a, b, c] = rl!(1, String).try_into().unwrap();
        out.push(format!("{}{}{}", a.chars().next().unwrap(), b.chars().next().unwrap(), c.chars().next().unwrap()))
    }
    out.iter().for_each(|o| println!("{}", o));
}

//IMPORTANT!! Submit Code Region End(Do not remove this line)