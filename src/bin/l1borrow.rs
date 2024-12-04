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

// 题目1：以下代码能编译通过吗？
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("really long string");
    &result
 }
 
 // 题目2：以下代码能编译通过吗？
 fn mai() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}", r);
 }
 
 // 题目3：以下代码能编译通过吗？
 struct Config<'a> {
    name: &'a str,
    data: String
 }
 
 fn process_config<'a>(cfg: &'a mut Config<'a>) {
    let new_name = String::from("new");
    cfg.name = &new_name;
 }
 
 // 题目4：以下代码能编译通过吗？
 fn mai2() {
    let mut data = String::from("hello");
    let r = &data;
    {
        let mut_r = &mut data;
        *mut_r = String::from("world");
    }
    println!("{}", r);
 }