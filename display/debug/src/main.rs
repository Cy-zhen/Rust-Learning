/*
#![allow(unused_variables)]
fn main() {
// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);
}

/*fn main() {
    println!("Hello, world!");
}*/
*/

//所有 std 库类型都天生可以使用 {:?} 来打印：

// 推导 `Structure` 的 `fmt::Debug` 实现。
// `Structure` 是一个包含单个 `i32` 的结构体

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age: u8
}

fn main(){
    //println!("hello");
    println!("{:?} months in one year",12);
    println!("{1:?} {0:?} is the {actor:?} name","Slater","Christain",actor = "actor's");
    //Structure也可以输出
    println!("Now {:?} will print!",Structure(3));
    
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!",Deep(Structure(7)));

    //
    let name = "Peter";
    let age  = 27;
    let peter = Person{name,age};
    
    //美化打印
    println!("{:#?}",peter);

}



