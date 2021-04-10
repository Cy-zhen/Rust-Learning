# 字面量和运算符
整数 `1`、浮点数 `1.2`、字符 `'a'`、字符串` "abc"`、布尔值` true `和单元类型`() `可以用数字、文字或符号之类的 “字面量”（literal）来表示。

另外，通过加前缀 `0x`、`0o`、`0b`，数字可以用十六进制、八进制或二进制记法表示。

为了改善可读性，可以在数值字面量中插入下划线，比如：`1_000` 等同于` 1000`，` 0.000_001 `等同于` 0.000001`。

我们需要把字面量的类型告诉编译器。如前面学过的，我们使用` u32 `后缀来表明字面量 是一个 32 位无符号整数，`i32` 后缀表明字面量是一个 32 位有符号整数。

[Rust](https://doc.rust-lang.org/reference.html#operator-precedence) 提供了一系列的运算符（operator），它们的优先级 和[类 C 语言](https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages)的类似。（译注：类 C 语言包括 C/C++、Java、PHP 等语言。）