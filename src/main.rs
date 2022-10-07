mod mlang;

fn main() {
    let v = mlang::mmul(mlang::madd(mlang::mint(1),
                                    mlang::mint(2)),
                        mlang::mint(3));
    let ev = v.eval();
    println!("{}", ev.val());
    println!("Hello, world!")
}
