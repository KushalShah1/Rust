struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active:bool
}

fn main() {

    let user1=User{
        email:String::from("exy@d.com"),
        username:String::from("dog"),
        sign_in_count:1,
        active:false
    };

    println!("{}",user1.email);
    let user2=User{
        email:String::from("ssss"),
        username:String::from("ddd"),
        ..user1
    };
}

fn build_user(email:String,username:String)->User{
    User{
        email,
        username,
        active:true,
        sign_in_count:1
    }
}
