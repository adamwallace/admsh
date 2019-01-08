use std::io;
use std::io::Write;
use std::env;

fn run_cmd (cmd: String) { // what does this return? 
    println!("{}", cmd);
    //cmd.clear(); // use this to clear string when done with cmd
}

fn main() {
    let mut status = true;
    loop { // rust doesn't have do { while() } 
        let mut current_dir = match env::current_dir() {
            Err(e) => panic!("Error getting current dir: {}", e), 
            Ok(dir) => dir, 
        };
        print!("ash {} > ", current_dir); // TODO: remove quotes

        let mut cmd = String::new(); 
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut cmd)
            .expect("Failed to read command.");
        
        run_cmd(cmd);
        if !status { break }
    }
}
