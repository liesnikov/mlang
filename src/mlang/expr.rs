// values

pub trait MVal<T> {
    fn val(&self) -> T;
}


// booleans

pub struct MBool {
    mbool : bool
}

impl MVal<bool> for MBool {
    fn val(&self) -> bool {
        self.mbool
    }
}

// integers

pub struct MInt {
    mint:isize
}

impl MVal<isize> for MInt {
    fn val(&self) -> isize {
        self.mint
    }
}

// expressions

pub trait MExpr<T> {
    fn eval(&self) -> Box<dyn MVal<T>>;
}

// int literals

pub struct MIntV {
    lit : isize
}

pub fn mint(i : isize) -> Box<MIntV> {
    Box::new(MIntV { lit : i})
}

impl MExpr<isize> for MIntV {
    fn eval(&self) -> Box<dyn MVal<isize>> {
        Box::new(MInt { mint: self.lit })
    }
}

// integer binary minus

pub struct MMinus{
    left: Box<dyn MExpr<isize>>,
    right: Box<dyn MExpr<isize>>
}

pub fn mminus(l : Box<dyn MExpr<isize>>, r : Box<dyn MExpr<isize>>) -> Box<dyn MExpr<isize>> {
    Box::new(MMinus { left : l, right : r})
}

impl MExpr<isize> for MMinus {
    fn eval(&self) -> Box<dyn MVal<isize>> {
        let lv = self.left.eval();
        let rv = self.right.eval();
        let lint = lv.val();
        let rint = rv.val();
        Box::new(MInt { mint : lint - rint })
    }
}

// integer unary minus

pub fn muminus(i : Box<dyn MExpr<isize>>) -> Box<dyn MExpr<isize>> {
    let zero = mint(0);
    Box::new(MMinus { left : zero, right : i})
}


// addition

pub struct MAdd{
    left: Box<dyn MExpr<isize>>,
    right: Box<dyn MExpr<isize>>
}

pub fn madd(l : Box<dyn MExpr<isize>>, r : Box<dyn MExpr<isize>>) -> Box<dyn MExpr<isize>> {
    Box::new(MAdd { left : l, right : r})
}

impl MExpr<isize> for MAdd {
    fn eval(&self) -> Box<dyn MVal<isize>> {
        let lv = self.left.eval();
        let rv = self.right.eval();
        let lint = lv.val();
        let rint = rv.val();
        Box::new(MInt { mint : lint + rint })
    }
}

// multiplication

pub struct MMul{
    left: Box<dyn MExpr<isize>>,
    right: Box<dyn MExpr<isize>>
}

pub fn mmul(l : Box<dyn MExpr<isize>>, r : Box<dyn MExpr<isize>>) -> Box<dyn MExpr<isize>> {
    Box::new(MMul { left : l, right : r})
}

impl MExpr<isize> for MMul {
    fn eval(&self) -> Box<dyn MVal<isize>> {
        let lv = self.left.eval();
        let rv = self.right.eval();
        let lint = lv.val();
        let rint = rv.val();
        Box::new(MInt { mint : lint * rint })
    }
}

// boolean literals

pub struct MBoolV {
    lit : bool
}

pub fn mbool(b : bool) -> Box<dyn MExpr<bool>> {
    Box::new(MBoolV { lit : b})
}

impl MExpr<bool> for MBoolV {
    fn eval(&self) -> Box<dyn MVal<bool>> {
        Box::new(MBool { mbool: self.lit })
    }
}

// boolean neg

pub struct MBoolNeg {
    inner : Box<dyn MExpr<bool>>
}

pub fn mboolneg(b : Box<dyn MExpr<bool>>) -> Box<dyn MExpr<bool>> {
    Box::new(MBoolNeg { inner : b})
}

impl MExpr<bool> for MBoolNeg {
    fn eval(&self) -> Box<dyn MVal<bool>> {
        let iv = self.inner.eval();
        let ibool = iv.val();
        Box::new(MBool { mbool: !ibool })
    }
}


// and

pub struct MAnd{
    left: Box<dyn MExpr<bool>>,
    right: Box<dyn MExpr<bool>>
}

pub fn mand(l : Box<dyn MExpr<bool>>, r : Box<dyn MExpr<bool>>) -> Box<dyn MExpr<bool>> {
    Box::new(MAnd { left : l, right : r})
}

impl MExpr<bool> for MAnd {
    fn eval(&self) -> Box<dyn MVal<bool>> {
        let lv = self.left.eval();
        let rv = self.right.eval();
        let lbool = lv.val();
        let rbool = rv.val();
        Box::new(MBool { mbool : lbool & rbool })
    }
}


// or

pub struct MOr{
    left: Box<dyn MExpr<bool>>,
    right: Box<dyn MExpr<bool>>
}

pub fn mor(l : Box<dyn MExpr<bool>>, r : Box<dyn MExpr<bool>>) -> Box<dyn MExpr<bool>> {
    Box::new(MOr { left : l, right : r})
}

impl MExpr<bool> for MOr {
    fn eval(&self) -> Box<dyn MVal<bool>> {
        let lv = self.left.eval();
        let rv = self.right.eval();
        let lbool = lv.val();
        let rbool = rv.val();
        Box::new(MBool { mbool : lbool | rbool })
    }
}


// if

pub struct MIf<T>{
    cond: Box<dyn MExpr<bool>>,
    left: Box<dyn MExpr<T>>,
    right: Box<dyn MExpr<T>>
}

pub fn mif<T>(c : Box<dyn MExpr<bool>>,
              l : Box<dyn MExpr<T>>,
              r : Box<dyn MExpr<T>>) -> Box<MIf<T>> {
    Box::new(MIf { cond : c, left : l, right : r})
}

impl<T> MExpr<T> for MIf<T> {
    fn eval(&self) -> Box<dyn MVal<T>> {
        let cv = self.cond.eval();
        let c = cv.val();
        if c {
            self.left.eval()
        } else {
            self.right.eval()
        }
    }
}
