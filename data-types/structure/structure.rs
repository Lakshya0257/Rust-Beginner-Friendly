struct User{                    //model/stucture of user
    username : String,
    email : String,
    sign_in_count : u64,
    active : bool,
}

fn main(){
    let mut user1=User{
        username : String::from("Lakshya"),
        email: String::from("lakshyabhati24@gmail.com"),
        sign_in_count: 25,
        active: true
    };

    let name: String = user1.username;
    user1.username=String::from("Lakshya0257");

    let user2: User=builduser(
        String::from("xyz@gmail.com"),
        String::from("User2")
    );

    let user3: User=User{
        username:String::from("James"),
        email:String::from("james@gmail.com"),
        ..user2   // rest of the values will be taken as same as user2
    };

    println!("{}",user3.active);
}


//new user
fn builduser(email: String, username: String) -> User{
    User{
        username: username,
        email: email,
        sign_in_count:1,
        active:true,
    }
}