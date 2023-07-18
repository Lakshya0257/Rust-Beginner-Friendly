fn main(){
    #[derive(Debug)]
    enum Animals{
        Cats(i32),
        Dogs(String),
        Other(bool)
    }

    let row=vec![
        Animals::Cats(3),
        Animals::Other(true)
    ];

    println!("{:?}" , row);

    match &row[0]{
        Animals::Other(value)=>println!("{}",value),
        _=>println!("Invalid")
    };
}