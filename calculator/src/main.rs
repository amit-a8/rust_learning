use std::env::{args, Args} ;

fn main() {
    let mut args:Args = args();
    //println!("{:?}", args);
    let first = args.nth(1).unwrap();  
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();
    //let second = args.nth(1);
    let first_number = first.parse::<f32>().unwrap();
    let sec_number = second.parse::<f32>().unwrap();
    let result = operate_based_on_pattern(operator, first_number, sec_number) ;
    let res = operate(operator, first_number, sec_number) ;
    println!("{}", output(first_number, operator, sec_number, result)) ;
    println!("{}", output(first_number, operator, sec_number, res))
}

fn operate_based_on_pattern(operator:char, first_number:f32, sec_number:f32)-> f32{
    match operator{
        '+' => first_number + sec_number,
        '-' => first_number - sec_number,
        '/' => first_number/ sec_number,
        '*'| 'x' | 'X' => first_number* sec_number,
        _ => panic!("invalid oprator used")

    }
}

fn operate(operator:char, first_number:f32, sec_number:f32)-> f32{
    if operator == '+'{
         first_number + sec_number 
    }
    else if operator == '-'{
         first_number - sec_number 
    }
    else if operator == '/' {
         first_number/ sec_number 
    }else if operator == '*' || operator == 'x' || operator == 'X' {
         first_number * sec_number 
    }else {
         0.0 
    }
}


fn output(f_num:f32,operator:char, s_num:f32, result:f32) -> String{
    format!("{} {} {} = {}", f_num, operator, s_num, result)
}
