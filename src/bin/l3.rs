
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


fn main () {

}