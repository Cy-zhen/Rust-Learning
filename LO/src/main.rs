fn main() {
    println!("Hello, world!");

    //数组相加
    println!("1 + 2 = {}", 1u32 + 2);
    //如果将'i32',改为'u32'，将会出现错误
    println!("1 - 2 = {}", 1i32 - 2);

    //短路求值的bool逻辑
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    println!("NOT false is {}", !false);

    //位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b11u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性！
    println!("One million is written as {}", 1_000_000u32);
    
}
