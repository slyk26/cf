
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
        let size = rl!(usize)[0];
        let mut matrix = vec![vec![0; size]; size];
        
        for i in 0..size {
            matrix[i] = rl!(i32)
        }
        
        let mut result = vec![0; 2*size];
        
        for i in 0..size {
            for j in 0..size {
                result[i+j+1] = matrix[i][j]
            }
        }

        for i in 1..(2*size) +1 {
            if let None = result.iter().find(|x| **x == i as i32) {
                result[0] =  i as i32; 
            }
        }
        
        out.push(format!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")));
    });
    out.iter().for_each(|x| println!("{}", x));
}

//IMPORTANT!! Submit Code Region End(Do not remove this line)