trait Lang {
    type Repr<T>;
    type Ret<T>;
    fn add(&self, a: Self::Repr<i32>, b: Self::Repr<i32>) -> Self::Ret<i32>;
}

struct Expression {}
struct PrettyPrint {}
impl Lang for Expression {
    type Repr<T> = T;
    type Ret<T> = T;
    fn add(&self, a: Self::Repr<i32>, b: Self::Repr<i32>) -> Self::Ret<i32> {
        a + b
    }
}

impl Lang for PrettyPrint {
    type Repr<T> = T;
    type Ret<T> = String;
    fn add(&self, a: Self::Repr<i32>, b: Self::Repr<i32>) -> Self::Ret<i32> {
        let ans = a + b;
        return format!("{} + {} = {}", a, b, ans);
    }
}

fn compute<T: Lang>(l: T, a: T::Repr<i32>, b: T::Repr<i32>) -> T::Ret<i32> {
    return l.add(a, b);
}

fn main() {
    let e = Expression {};
    let p = PrettyPrint {};
    println!("Ans: {}", compute(e, 1, 3));
    println!("Ans: {}", compute(p, 1, 3));
}
