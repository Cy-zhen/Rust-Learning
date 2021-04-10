//元组可以充当函数的参数以及返回值
fn reverse(pair: (i32, bool)) -> (bool, i32){
    //可以使`let`把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;
    
    (boolean, integer)//返回值
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main(){
    //包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);
    
    //通过元组下标来访问具体的值
    println!("long tuple frist value: {}", long_tuple.0);
    //long tuple frist value: 1
    println!("long tuple second value: {}", long_tuple.1);
    //long tuple second value: 2

    //元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    //元组也可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    //tuple of tuples: ((1, 2, 3), (4, -1), -2)

    // 但很长的元组无法打印
    /*let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
     println!("too long tuple: {:?}", too_long_tuple);*/
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。
    //error: aborting due to previous error

    let pair = (1, true);
    println!("pair is {:?}", pair);
    //pair is (1, true)
    println!("the reversed pair is {:?}", reverse(pair));
    //the reversed pair is (true, 1)
    
    // 创建单元素元组需要一个额外的逗号， 这是为了和被括号包含的字面量做区分。
    println!("one elemnt tuple: {:?}", (5u32,));
    //one elemnt tuple: (5,)
    println!("just an integer: {:?}", (5u32));
    //just an integer: 5

    //元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    //1, "hello", 4.5, true

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    //Matrix(1.1, 1.2, 2.1, 2.2)

}
