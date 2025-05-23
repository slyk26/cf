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
        
        
        
        out.push(format!("{}", 42));
    });
    out.iter().for_each(|x| println!("{}", x));
}