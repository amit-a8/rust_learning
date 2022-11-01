fn main() {
    //println!("Hello, world!");
    //let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    println!("{}",s);
    let mut updated_str = String::from("from amit") ;
    updated_str.push_str(" Hi") ;
    println!("{}", updated_str);
}
