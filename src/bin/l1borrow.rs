fn main() {
    
}

fn b2() {
    let mut v = vec![String::from("hello")];
    for i in 0..2 {
        // 可变借用在这一行以下不再活跃了。
        v.push("world".to_string());    // 尝试创建可变借用

        let s = &v[0];        // 创建不可变借用
        println!("{}", s);
        println!("{}", v[1]);
        // 为什么这里可以编译通过？
        println!("{} {}", s, v[1]);
    }
}

fn b1() {
    let mut v = vec![String::from("hello")];
    for i in 0..2 {
        let s = &v[0];        // 创建不可变借用
        v.push(s.clone());    // 尝试创建可变借用
        // 这里有趣的地方在于：
        // 如果println的代码打开，则编译无法通过。
        // 因为rust中同一时刻只能存在一个活跃的可变借用或多个活跃的可变借用
        // 一个不活跃的不可变借用加上一个活跃的可变借用是允许的。
        // println!("{}", s);
    }
}