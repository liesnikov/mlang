mod mlang;

use mlang::expr::*;
use serde_yaml;

fn main() {
    let v = mmul(madd(mint(1),
                      mint(2)),
                 mint(3));
    let ev = v.eval();
    println!("{}", ev.val());
    println!("Hello, world!");
    let l : Result<serde_yaml::Value,_> = serde_yaml::from_str("[1,2,3]");
    println!("{:?}", l)
}
