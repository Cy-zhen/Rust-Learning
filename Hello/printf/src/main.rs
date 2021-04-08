/*
打印操作由 std::fmt 里面所定义的一系列宏来处理，包括：

    format!：将格式化文本写到字符串（String）。（译注：字符串是返 回值不是参数。）
    print!：与 format! 类似，但将文本输出到控制台（io::stdout）。
    println!: 与 print! 类似，但输出结果追加一个换行符。
    eprint!：与 format! 类似，但将文本输出到标准错误（io::stderr）。
    eprintln!：与 eprint! 类似，但输出结果追加一个换行符。
*/
fn main() {
    //'{}'在通常情况下会被任意变量内容所替换
    //变量内容会转化成字符串
    //println!("Hello, world!");
    println!("{} days",31);
    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型。

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0},this is {1}. {1},this is {0}","Alice","bob");
    
    //可以使用命名参数
    println!("{subject} {verb} {object}",
            object  = "the lazy dog",
            subject = "the quick brown fox",
            verb    = "jumps over" );
    
    //可以在'：' 后面指定特殊的格式   
    println!("{} of {:b} people know binary, the other half don't",1,2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1
    println!("{number:>width$}",number = 1 , width = 10);
    
    // 你可以在数字左边补 0。下面语句输出 "000001"
    println!("{number:>0width$}",number = 1 , width = 10);

    //println! 会检查使用到的参数数量是否正确
    println!("My name is {0} , {1} {0}" , "bond" ,"jame");
    // 改正 ^ 补上漏掉的参数："James"
}
