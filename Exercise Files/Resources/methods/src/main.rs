struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}",rect1.area());

    let name =String::from("ram");
    println!("{}",match name.chars().nth(2){
        Some(c)=>c.to_string(),
        None=>"No char found".to_string()
    });
}