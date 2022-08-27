
fn main() {

    let num:[i32; 2] = [7, 3];
    let v = vec![7,3,5] ;
    println!("{:?}", sum(num));
    // let resp:bool = can_sum(num, 11);
    // println!("{}", resp);
    println!("{}", can_sum_input_vec(v, 2))
    // let sum:i32 = fibb(15) ;
    // println!("{}", sum);
    
}

fn sum(arr:[i32; 2]) -> i32 {

    return arr[0] + arr[1];

}

// fn can_sum(arr: [i32; 2], target_value:i32) -> bool {
//     let mut result = false  ;
//     if target_value < 0 {
//         result = false  ;
//     }
//     else if target_value == 0 {
//         result = true  ;
//     }
//     else{
//         for each_item in arr.iter(){
            
//             let new_target = target_value - each_item ;
            
//             let re:bool =   can_sum(arr, new_target);
//             if re  == true{
//                 result = true ;
//                 break;
//             }

//         }
//     }
//     result
// }

fn can_sum_input_vec(input_vector: Vec<i32>, target_value:i32)-> bool{
    let mut result = false;
    let copy_input_vector = &input_vector ;
    if target_value < 0{
        result = false;
    }
    else if target_value == 0{
        result =  true ;
    }
    else{
        for i in &input_vector{
            let new_target:i32 = target_value - i ;
            let this_res:bool = can_sum_input_vec((&copy_input_vector).to_vec(), new_target);
            if this_res == true{
                result = true ;
                break;
            }
        }
    }
    return result ;
}



// fn fibb(num: i32) -> i32{
//     if memo.contains_key( &num){
//         let value= memo.get(&num);
//         return value ;
//     }
//     if num <= 2{
//         return num;
//     }
//     else{
//         let val =  fibb(num-1) + fibb(num-2);
//         memo.insert(num, val);
//         return val ;
//     }
// }