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
       let mut number = rl!(String).remove(0).chars().collect::<Vec<char>>();

        for i in 0..number.len() {
            let x = number[i].to_digit(10).unwrap();
            let min = (10 - (i + 1)) as u32;
            if min == x { continue }

            let mut slice = &number[i+1..];
            let f = slice.iter().position(|n| n.to_digit(10).unwrap() == min);
            if  let Some(x) = f {
                let r = number[i];
                number[i] = slice [x];
                number[i+1+x] = r;
            } else {
                slice = &number[i..];
                if let Some((idx, _)) = min_by_key(slice, min) {
                    let tmp = number[i];
                    number[i] = slice[idx];
                    number[i + idx] = tmp;
                }
            }
        }
        out.push(format!("{}", number.iter().collect::<String>())); 
    });
    out.iter().for_each(|x| println!("{}", x));
}

fn min_by_key(s: &[char], min: u32) -> Option<(usize, char)> {
    s.iter()
        .enumerate()
        .filter_map(|(idx, &c)| {
            c.to_digit(10)
                .filter(|&num| num >= min)
                .map(|num| (idx, c, num)) 
        })
        .min_by_key(|&(_, _, num)| num) 
        .map(|(idx, c, _)| (idx, c))
}

//IMPORTANT!! Submit Code Region End(Do not remove this line)