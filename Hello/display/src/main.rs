/*
#![allow(unused_variables)]
fn main() {
    use std::fmt; // （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
    // 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
    // `Structure`，包含一个 `i32` 元素。
    struct  Structure(i32);

    // 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait
    impl fmt::Display for Structure{
        // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
        fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
            // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
            // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
            write!(f,"{}",self.0)
        }
    } 
   //println!("Hello, world!");
}
*/


use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    } 
}

#[derive(Debug)]
struct Comlpex { real: f64, imag: f64, }

impl fmt::Display for Comlpex{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "real: {}, imag: {}", self.real, self.imag)
    }
}

fn main(){
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and  the small is {small}",
            small = small_range,
            big   = big_range);
    
    let point = Point2D{ x: 3.3, y: 7.2 };

    let complex = Comlpex{ real: 3.3, imag: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Display: {} + {} i", point.x, point.y);
    println!("Debug: {:?}", complex);
}

/*use std::fmt; // 导入 `fmt`

// 带有两个数字的结构体。推导出 `Debug`，以便与 `Display` 的输出进行比较。
#[derive(Debug)]
struct MinMax(i64, i64);

// 实现 `MinMax` 的 `Display`。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用 `self.number` 来表示各个数据。
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 为了比较，定义一个含有具名字段的结构体。
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 类似地对 `Point2D` 实现 `Display`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 `x` 和 `y` 的值。
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
    // 得到实现。这语句不能运行。
    // println!("What does Point2D look like in binary: {:b}?", point);
}
*/