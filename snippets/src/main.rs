pub fn main() {
    /*
    [1..10] 和 (1..10) 的区别
    */

    [1..10].iter().for_each(|x| println!("{:?}", x)); // output: 1..10
    (1..10).for_each(|x| print!("{:?},", x)); // output: 1,2,3,4,5,6,7,8,9,

    let a = [1..10]; // Variable a is a array of type [Range<i32>;1]
    let b = 1..2; // Variable b is a Range of type Range<i32>
}
