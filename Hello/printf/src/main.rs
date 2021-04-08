fn main() {
    //'{}'在通常情况下会被任意变量内容所替换
    //变量内容会转化成字符串
    //println!("Hello, world!");
    println!("{} days",31);
    //31 days
    
    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型。

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0},this is {1}. {1},this is {0}","Alice","bob");
    //Alice,this is bob. bob,this is Alice

    //可以使用命名参数
    println!("{subject} {verb} {object}",
            object  = "the lazy dog",
            subject = "the quick brown fox",
            verb    = "jumps over" );
    //the quick brown fox jumps over the lazy dog

    //可以在'：' 后面指定特殊的格式   
    println!("{} of {:b} people know binary, the other half don't",1,2);
    //1 of 10 people know binary, the other half don't

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1
    println!("{number:>width$}",number = 1 , width = 10);
    //         1

    // 你可以在数字左边补 0。下面语句输出 "000001"
    println!("{number:>0width$}",number = 1 , width = 10);
    //0000000001

    //println! 会检查使用到的参数数量是否正确
    println!("My name is {0} , {1} {0}" , "bond" ,"jame");
    //My name is bond , jame bond
    // 改正 ^ 补上漏掉的参数："James"
}
