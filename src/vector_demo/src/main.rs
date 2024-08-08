#[derive(Debug)]
#[allow(dead_code)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
//consider changing the field to be of unit type to suppress this warning while preserving the field numbering, or remove the field: `()`rustc(dead_code)
// 用中文翻译一下就是：考虑将字段更改为单元类型以抑制此警告，同时保留字段编号，或删除字段：`（）`rustc(dead_code)
// 什么是单元类型
// 单元类型是一个空元组，它只有一个值，就是()，这个值在rust中叫做unit，它是一个类型，它的值是一个空元组
// 这个警告是因为我们定义了一个结构体，但是没有使用，所以rust会提示我们这个结构体没有使用，可以删除或者使用
// 这个enum不是已经在main函数中使用了吗？为什么还会提示这个警告呢？
// 如何消除这个警告
// 1. 可以在结构体前面加上#[allow(dead_code)]，这样就不会提示这个警告了


fn main() {
    // let v:Vec<i32> = Vec::new();
    // let v1 = vec![1,2,3];
    // let mut v = Vec::new();
    // v.push(1);
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[5];
    // println!("The third element is {}", third);

    // match v.get(5) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element"),
    // }
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 40;
    // }
    // for i in v {
    //     println!("{}", i);
    // }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.1),
        SpreadSheetCell::Text(String::from("ok")),
    ];

    for i in row {
        println!("{:?}", i);
    }
}
