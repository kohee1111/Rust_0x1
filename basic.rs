--simple example
//see the diff of the same veriable 
fn main(){
    let name = "Dibbo" ;
    {
        let name : &str = "kohee" ; 
        println!("Your name is :{:?}" , name) ;
    }
    println!("Your name is :{:?}" , name) ;
}
--simple example -----------------------------------------
//#![allow(unused)] 
#![allow(unused)] 
fn main(){

    let mut name = "Dibbo"  ; 
    //println!("Name is :{:?}" , name) ;
}
--simple example --------------------------------------
fn main(){
    
    let mut name : String = "dibbo".to_string() ;      //same as let mut name = String::from("dibbo")   
    name.push_str(", kohee")  ; 
    println!("Your name is :{:?}" , name) ;
}
--------------------------------simple example(learning return) 
fn ret_string() -> String{

    return "Dibbo".to_string()  ;
}
fn ret_str()-> &'static str{

    return "kohee"  ;
}
fn main(){
    let name2 : &'static str  = ret_str() ; 
    println!("Name2 is {:?} " , name2) ;
    let name1: String = ret_string() ; 
    println!("Your name is :{:?}" , name1) ;
}
----------------------------------------------------------------------------------------------------------------------
fn main(){

    let name1 = String::from("Dibbo") ; 
    let name2 = name1.clone() ; 
    {
        let name2 = name1.clone() ; 
        println!("Name 2 is {:?}" , name2) ;
        println!("Scope has ended") ;
    }
    println!("name 2 is {:?}" , name2)
}
