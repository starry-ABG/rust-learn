struct Data {
    val: i32,
}
impl Data {
    fn print_val(&self) -> i32 {
        self.val
    }
}

fn reference_and_dot() {
    let data = Data {val: 5};
    let v = (&data).val;
    // dot的优先级高于&
    let v = &data.val;
    let r = &data;
    // dot的优先级也高于*
    let a = (*r).val;

}

fn main() {}
