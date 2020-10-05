use std::{thread, time};
use std::io;
use notify_rust::Notification;

const ONE_SECOND: std::time::Duration = time::Duration::from_millis(1000);

fn main() {
    let total_secs: usize = user_input_as_secs();
    clock(total_secs);
    alarm(total_secs);
}

fn user_input_as_secs() -> usize {
    loop {
        let mut input = String::new();
        println!("Please write the number of seconds the clock should run for!");

        io::stdin().read_line(&mut input)
            .ok()
            .expect("Couldn't read line!");

        input = input.trim().to_string();
        if !validate_input(&input) { continue } else { return input.parse::<usize>().unwrap() }
    }

    fn validate_input(input: &String) -> bool {
        for c in input.chars() {
            // println!("The charcter: {} and is it a digit? {}", c, c.is_ascii_digit());
            if !c.is_ascii_digit() { 
                return false;
            }
        }
        true
    }
}

fn clock(total_secs: usize) {
    let mut current_secs = total_secs;

    loop {
        print_seconds(current_secs);

        current_secs -= 1;
        if current_secs == 0 {
            current_secs = total_secs; 
            alarm(total_secs);
        };

        thread::sleep(ONE_SECOND);
    }
}
 
fn alarm(total_secs: usize) {
    Notification::new()
    .summary("Timer Resetting!")
    .body(&format!("Your timer of {secs} is up! I'll automatically restart your {secs} timer now.", secs=total_secs))
    .icon("firefox")
    .show()
    .ok()
    .expect("Notification error (Non-critical)");
}

fn print_seconds(total_secs: usize) {
    println!("{:0width$}:{:0width$}", total_secs/60, total_secs%60, width = 2);
}

