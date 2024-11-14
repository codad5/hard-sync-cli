use colored::Colorize;

pub enum Step {
    Start(String),
    End(String),
    Syncing(String),
    Copying((String, String)),
    Completed(String),
}

impl Step {
    pub fn get_message(&self) -> String {
        match self {
            Step::Start(message) => format!("{}: {}", "Start".cyan(), message),
            Step::End(message) => format!("{}: {}", "End".cyan(), message),
            Step::Syncing(message) => format!("{}: {}", "Syncing".cyan(), message),
            Step::Copying((src, dest)) => format!("{}: {} to {}", "Copying".cyan(), src, dest),
            Step::Completed(message) => format!("{}: {}", "Completed".cyan(), message),
        }
    }

    pub fn print(&self) {
        print(&self.get_message());
    }
}

pub fn print(message: &str) {
    println!("{}", message);
}

pub fn print_error(message: &str) {
    print( format!("Error: {}", message.red()).as_str());
}

pub fn print_info(message: &str) {
    print(format!("Info: {}", message.blue()).as_str());
}

pub fn print_success(message: &str) {
    print(format!("Success: {}", message.green()).as_str());
}

pub fn print_warning(message: &str) {
    print( format!("Warning: {}", message.yellow()).as_str());
}

pub fn step_message(step: &str, message: &str) {
    println!("{}", format!("Step: {}", message).cyan());
}