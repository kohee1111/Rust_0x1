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
//
