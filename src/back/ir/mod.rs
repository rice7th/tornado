// VSCode broke and I cannot paste wth
// https://bernsteinbear.com/blog/whats-in-an-egraph/ <- I had to write ts
// manually. It's 1:15am and I can't even type crying emojis
// because of this fucking piece of shit called election.

#[derive(Clone, Debug, PartialEq)] // PARTIAL EQ???
pub struct Expr {
    parent: Option<Box<Expr>>
}

impl Expr {
    // Less clones??
    pub fn find(&self) -> Box<Expr> {
        let mut expr = Box::new(self.clone());
        while expr.parent.is_some() {
            let next = self.parent.clone().unwrap().parent;
            if next.is_none() {
                return Box::new(self.clone());
            }
            expr = next.clone().unwrap();
        }
        return expr;
    }

    pub fn merge(&mut self, rhs: Box<Expr>) {
        let root = self.find();
        if root == rhs {
            self.parent = Some(rhs.clone());
        }
    }
}

pub enum Nodes<T> {
    Const(Const<T>),
    SimpleOps(Binary),

    //Gamma(Gamma),
    //Theta(Theta),
    //Omega(Omega),
    //Lambda(Lambda),
    //Phi(Phi),
    //Delta(Delta)
}

pub struct Const<T>(T);

pub struct Binary {
    left: Expr,
    right: Expr
}

pub struct Add {
    op: Binary
}