// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // 核心修复1：定义迭代器变量（mut 必须加，因为next()会修改迭代器状态）
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();

    // 核心修复2：按迭代器自然遍历顺序修正assert（迭代器按vec顺序取元素）
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    // 可选：补充最后一个next()返回None，符合迭代器遍历逻辑
    assert_eq!(my_iterable_fav_fruits.next(), None);
}