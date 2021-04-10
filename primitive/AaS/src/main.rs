use std::mem;

//此函数借用一个slice
fn analyze_slice(slice: &[i32]) {
    println!("frist element of the slice: {}", slice[0]);
    //first element of the array: 1
    println!("the slice has {} elements", slice.len());
}

fn main(){
    //定长数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    //所有元素都可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    //从下标0开始
    println!("frist element of the array: {}", xs[0]);
    //first element of the array: 1
    println!("secong element of the array: {}", xs[1]);
    //second element of the array: 2

    //`len` 返回数组的大小
    println!("array size: {}", xs.len());
    //array size: 5

    //数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    //array occupies 20 bytes

    //数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    //borrow the whole array as a slice
    //first element of the slice: 1

    //slice 可以指向数组的一部分
    println!("borrow a section of array as a slice");
    analyze_slice(&ys[1 .. 4]);
    //borrow a section of the array as a slice
    //first element of the slice: 0
    //the slice has 3 elements

    //越界的下标会引发错误（panic）
    //println!("{}", xs[5]);
    //error: aborting due to previous error
    //error: could not compile `playground`
    println!("{}", xs[4]);
    //5
}
