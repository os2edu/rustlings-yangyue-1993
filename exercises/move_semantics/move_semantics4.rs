// move_semantics4.rs
// Refactor this code so that instead of passing `vec0` into the `fill_vec` function,
// the Vector gets created in the function itself and passed back to the main
// function.
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand for a hint.


fn main() {
    // 因为vec0没有用到所以必须给编译器一个确定的类型去推导他， 而下面的vec1在函数中有用到，所以就不虚一定要给一个类型
    let vec0: Vec<i32> = Vec::new();

    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
