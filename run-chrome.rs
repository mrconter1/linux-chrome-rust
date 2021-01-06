use std::process::Command;                                     

fn main() {
    Command::new("google-chrome")
        .spawn()
        .expect("Oh, boy. Either Chrome isn't installed or reality doesn't exist.");
}

                                                               
                                                               
