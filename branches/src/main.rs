fn main() {
    let a=15;
    if a<10{
        println!("dog");
    }else{
        println!("cat");
    }
    loop{
        println!("4");
        break;
    }
    let mut num=3;
    while num<10{
        println!("num={}",num);
        num=num+1;
    }

    let a=[1,2,3,4,5];
    for element in a.iter(){
        println!("{}",element);
    }
}
