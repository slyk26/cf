use std::io::stdin;

#[allow(duplicate_macro_attributes)]
#[macro_export]
macro_rules! rl {
    ( $cnt:literal, $t:ty ) => {
        {
            let _ :u32 = $cnt;
            let mut ret = Vec::new();
            for _ in 0..$cnt {
                let mut input = String::new();
                let _ = stdin().read_line(&mut input);
                ret.push(input.trim().parse::<$t>().unwrap());
            }
            ret
        }
    };
}

fn main() {
}