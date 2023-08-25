use std::io;
use std::io::Write;

fn main() {

    let mut op: i32 = 4;
    let mut vect: Vec<String> = Vec::new();

    while op != 0{
        
        println!("--- Sudent Register ---");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Exit");

        println!();

        print!("Select an option: "); 
        //io::stdout().flush().unwrap();

        let mut str = String::new();

        io::stdin().read_line(&mut str).expect("Failed to read line");
        op  = str.trim().parse().expect("Invalid input");

        //println!();


        match op{
            1 => {
                let mut name = String::new();

                //ask user to enter name
                print!("Enter name: ");
                io::stdout().flush().unwrap();

                io::stdin().read_line(&mut name).expect("Cannot read line");
                println!();

                add_students(name,&mut vect);
            },
            2 => {
                view_students(&vect);
            },
            3 => {
                op = 0;
            }
            _ => println!(),
           
        }
    }
}


// //function to add students
// fn add_students(name: String, input_vec: &Vec<String>) -> Vec<String>{
//     //let mut new_vec = input_vec.clone(); // Clone the vector
//     input_vec.push(name);
//     input_vec
// }

// //function to add students
fn add_students(name: String, input_vec: &mut Vec<String>) {
    //let mut new_vec = input_vec.clone(); // Clone the vector
    input_vec.push(name);
    //input_vec

}

//function to view students
fn view_students(input_vec: &Vec<String>){
    println!("--- Student List ---");
    for value in input_vec.iter(){
        print!("\r{}",value);
        //io::stdout().flush().unwrap();

        //std::io::stdout().flush();
    }
    println!();
    println!();

}
