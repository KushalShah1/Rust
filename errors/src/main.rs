use std::fs::File;
fn main() {
    //panic!("crash");
    //let v =vec![1,2,3];
    //print!("{}",v[99]);
    let _f=File::open("hello.txt").expect("Dog");
    /*let foo= match f{
        Ok(file)=>file,
        Err(e)=>{
            panic!("File not Found");
        }
    };*/
}
