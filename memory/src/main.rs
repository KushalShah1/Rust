fn main() {
    let mut x= String::from("Hello");
    x.push_str(", World");
    println!("{}",x);

    let s1=String::from("hello");
    let s2=s1;
    println!("{}",s2);
}
