fn main() {
    let func_ops: (u64, i32, [i32;3]) = (20,70, [7,69,420]);
    let s = "Hello,";
    let new_s = append_world(&s);
    println!("{}", new_s);
    println!("Fibonacci number {} is {}.",func_ops.0, fib(func_ops.0));
    println!("The sum of {:?} is {}.", func_ops.2, sum(&func_ops.2));
    println!("{} degrees fahrenheit is {} degrees celcius.", func_ops.1, ftc(func_ops.1));
}
fn sum(nums: &[i32]) -> i32 {nums.iter().sum()}
fn fib(n: u64) -> u64{
    match n {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
fn ftc(n: i32) -> i32{((f64::from(n)-32.0)*(5.0/9.0)).round() as i32}
fn append_world(s: &str)->String{
    String::from(s) + " world!"
}