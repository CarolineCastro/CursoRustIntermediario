//1. format the code

/*const pi:f32=3.14159265358979323846;
fn count_to_5()->i32{let mut foo=0;loop{if foo>pi as i32{if foo > 5{break;}}foo=foo+1;}return 5;}
fn main(){
println!("I can count to {}", count_to_5());
}
#[cfg(test)]
mod test{
use super::*;
#[test]
fn test_counting(){
assert_eq!(count_to_5() == 5, true);
}
}*/

//formatted code using cargo fmt
// run cargo clippy and fix the warnings

/*const pi: f32 = 3.14159265358979323846;

fn count_to_5() -> i32 {
    let mut foo = 0;
    loop {
        if foo > pi as i32 {
            if foo > 5 {
                break;
            }
        }
        foo = foo + 1;
    }
    return 5;
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
} */


//modifications after running cargo clippy
/*
//const PI: f32 = 3.141_592_7; //warning non upper case globals, replace for PI
                             //warning excessive precision, consider changing the type or truncation it to: '3.141_592_7'
                             
use std::f32::consts::PI;   //error approximate value of `f{32, 64}::consts::PI´ found


fn count_to_5() -> i32 {
    let mut number = 0; //warning blacklisted name, replace for other name.
    loop {
        if number > PI as i32 && number > 5 { //warning this if statement can be collapsed,  replace for if number > PI && number > 5
             number += 1; //warning assing op pattern, replace for foo += 1
        }
    }
    5 //warning needless return, replace for a tail expression
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}*/


//3. Challange: Cargo clippy can't see everything. 
                      
//use std::f32::consts::PI; we don't need the PI for a simple counting
 
fn count_to_5() -> i32 {
     let mut number = 0;

     while number < 5 { //generic loop unnecessary for count numbers, replace for while.
        //if number > PI as i32 && number > 5 { as we changed the loop so we don't need that if
        number += 1;
     }
     //5 the type of return is wrong, here has to be the variable not the number

     number
}
                             
fn main() {
  println!("I can count to {}", count_to_5());
}
                             
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
