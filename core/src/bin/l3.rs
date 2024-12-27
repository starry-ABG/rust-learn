use std::ops::Deref;


struct Ref {}

impl Ref {

    fn r(self) {}
    fn s(&self) {}
    fn t(&mut self) {}
}

fn call1() {
    let rf = Ref{};
    // 
    // 编译器会自动来对rf进行借用，(&rf).s().
    // 那么编译器的具体流程是怎么做的呢？
    // 编译器首先尝试(rf).s()是否可行？因为Ref的定义中s(&self),不满足条件
    // 然后尝试(&rf).s()，rf是不可变绑定的，所以可以获得&rf,发现满足，所以直接调用了。
    rf.s();
    // 一下两行是等价的。rf.s()本质会转换为下面的全限定调用方式
    Ref::s(&rf);

    rf.r();
}

fn pp(c: &String) {
    println!("{}", c);
}

fn ppp(c: &str) {
    println!("{}", c);
}
fn auto_deref() {
    let c = Box::new(String::new());
    // 注意看，pp函数参数类型是&String
    // &c的类型是&Box<String>
    // 说明虽然是box的引用，但是通过自动解引用，成为了&String
    // 所以在函数值传递的时候也存在自动解引用
    pp(&c);
    // 下面的调用将不能编译通过，所以如果传递的值不是引用则不会发生
    // 所以没有自动借用。
    // pp(c);
    // ppp的参数类型是&str
    // 这里依然可以编译通过的原因在于String类型实现了Deref，指向str
    // 所以如果函数只是需要一个字符串的引用，那么用&str更方便
    // 因为它可以适应&String和&str两种情况
    ppp(&c);

}

fn main () {

}