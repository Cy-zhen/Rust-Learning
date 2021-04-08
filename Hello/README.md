# Hello World
这是每一只程序猿👨‍💻的接触的第一个程序的源码 :computer:。

```Rust
// 这是注释内容，将会被编译器忽略掉
// 可以单击那边的按钮 "Run" 来测试这段代码 ->
// 若想用键盘操作，可以使用快捷键 "Ctrl + Enter" 来运行

// 这段代码支持编辑，你可以自由地修改代码！
// 通过单击 "Reset" 按钮可以使代码恢复到初始状态 ->

// 这是主函数
fn main() {
    // 调用编译生成的可执行文件时，这里的语句将被运行。

    // 将文本打印到控制台
    println!("Hello World!");
}

```
`println!` 是一个[宏]()（macros），可以将文本输出到控制台（console）。

使用 Rust 的编译器 `rustc` 可以从源程序生成可执行文件：
```Rust
$ rustc hello.rs
```
使用 `rustc` 编译后将得到可执行文件 `hello`。
```Rust
$ ./hello
Hello World!
```
