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
    //12 months in one year
    println!("{1:?} {0:?} is the {actor:?} name","Slater","Christain",actor = "actor's");
    //"Christain" "Slater" is the "actor\'s" name
    //Structure也可以输出
    println!("Now {:?} will print!",Structure(3));
    //Now Structure(3) will print!
    
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!",Deep(Structure(7)));
    //Now Deep(Structure(7)) will print!
 
    let name = "Peter";
    let age  = 27;
    let peter = Person{name,age};
    
    //美化打印
    println!("{:#?}",peter);
    /**Person {
    name: "Peter",
    age: 27,
    } */

}



