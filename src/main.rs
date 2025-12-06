use std::env;
use std::fs::File;
use std::io::Read;
use std::str::Lines;
fn main() {
    // get the arguments ( string and the filename ) 
    let args : Vec<String> = env::args().collect();

    let s : String = args[1].clone();
    let file_name : String = args[2].clone();

    // open the file and read line by line
    let mut file : File = File::open(file_name).expect("File Could Not be Found !!!, Please check for the location");
    let mut file_str:String = String::new();
    let _ = file.read_to_string(&mut file_str);
    let list_of_lines: Lines<'_>  = file_str.lines();
    
    // structure to store the list of lines where the word exist 
    let mut list_of_lines_found : Vec<String> = Vec::new();
    
    // loop each line one by one and if foudn 
    for _s in list_of_lines{
        if _s.contains(&s) {
            list_of_lines_found.push(_s.to_owned());
        }
    }
    println!("YO , i found the word in the following : ");
    for _s in list_of_lines_found{
        println!("{}", _s);
    }

}
