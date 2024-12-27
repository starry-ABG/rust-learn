use std::sync::Arc;

// for 循环与作用域
// 知识点，一个变量进入一个新的作用域，不能认为它将一定发生了move或者copy
// 而要看新作用域中说如何使用它的，可能只是用了它的引用，那么其实是引用进入了新的
// 作用域
// 可以认为编译器会根据使用情况来传递引用或者自身进入新的作用域
struct Data {
    v: Vec<i32>,
}

impl Data {
    pub fn new() -> Self {
        Data { v: vec![] }
    }

    pub fn push(mut self, i: i32) {
        self.v.push(i);
    }
}

fn main() {
    let mut data = Data::new();
    let mut data_vec = vec![];


    for i in 0..=2 {
        // data.push(i);
        data_vec.push(i);
    }
}


// for循环本质上是对迭代器的一个语法糖
fn for_() {
    let mut data_vec = vec![];
    let mut data2 = vec![1];
    let mut data3 = vec![1];
    {
        // 创建迭代器
        let mut iter = (0..=2).into_iter();
        let data3 = data2;
        let ddd = &data3;
        println!("{:?}", ddd);

        // 不断调用 next
        loop {
            match iter.next() {
                None => break, // 如果迭代结束就退出循环
                Some(i) => {
                    // 循环体
                    data_vec.push(i);
                }
            }
        }
    }
    // 这里data_vec依然能使用，说明data_vec并没有在上面的作用域中被消耗
    // 但是在上面的作用域中确实用到了data_vec
    // 你说明进入作用域的不是data_vec而是&data_vec
    // 所以如果一个变量进入到一个作用域中，还要看是对其如何使用的才能知道
    // 使用的到底是它本身还是它的引用
    data_vec.push(1 as i32);

    // 这里将不能使用了，也说明了上面的道理。
    // data2.push(2);

    // 在上面作用域对data3只是使用了它的引用，所以并存在move的情况
    data3.push(3);
}

fn arc_clone() {
    let data = Arc::new(vec![1]);
    for i in 0..3 {
        // clone也是&self
        data.clone();
    }
}
