use std::io;

fn main(){
    let  x = ip_word();
    println!("{}",x);
    let y = feedback();
    println!("{:?}",y);

    


}

fn ip_word() -> String {
    println!("five letter word");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("failed input");
    x
}

fn feedback() -> Vec<u16>{
    println!("Enter feedback");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("failed input");
    let code : Vec <u16> = x.trim().chars().map(|c| c.to_digit(10).expect("failed") as u16).collect();
    code
        
}

