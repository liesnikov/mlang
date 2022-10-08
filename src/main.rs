mod mlang;

use mlang::expr::*;

fn main() {
    let v = mmul(madd(mint(1),
                      mint(2)),
                 mint(3));
    let ev = v.eval();
    println!("{}", ev.val());
    println!("Hello, world!")
}
