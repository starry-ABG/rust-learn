# 所有权规则和借用规则

两个规则是Rust最基础的东西，对它们理解的越深，越能掌握Rust。

Ownership Rules
所有权规则：

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

The Rules of References
借用规则：

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

生命周期标注是如何来解决问题的？

借用检查器具体是如何通过生命周期标注来解决问题的？
生命周期标注的核心目的是为借用检查器提供足够信息，以便在编译时验证所有引用都是有效的。

```rust
fn first<'a>(x: &'a str, y: &str) -> &'a str {
   x
}

fn main() {
   // 可以编译:
   let s1 = String::from("long");
   let s2 = String::from("short");
   let result = first(&s1, &s2);
   println!("{}", result); // s1活得和result一样久

   // 不能编译:
   let result;
   {
       let s1 = String::from("temp");
       result = first(&s1, "hello"); // s1生命周期太短
   }
   println!("{}", result); // s1已经失效

   // 不能编译:
   let s1;
   let result; 
   {
       let s2 = String::from("hello");
       s1 = String::from("world"); 
       result = first(&s2, &s1); // s2生命周期太短
   }
   println!("{}", result);
}
```
