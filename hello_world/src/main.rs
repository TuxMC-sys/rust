fn main(){
    let c =  |b, c| b + c;
    let a =  |d: &str, e: &str| d.to_owned() + e;
    println!("{}",c(1,2));
    println!("{}",a("test"," test2"));
}