use print_and_log::*;

fn main() {
    //Initializing
    let mut pal = PrintAndLog::new();
    
    //Changing and reading settings
    pal.set_log_to_file(true);
    let _we_are_writing_logs: bool = pal.get_log_to_file();

    let _ = pal.set_log_file_name("mycool.log");
    let _name_of_the_log_file: String = pal.get_log_file_name();

    let _ = pal.set_max_file_size(4000000);
    let _maximal_log_file_size: u32 = pal.get_max_file_size();

    let _ = pal.set_timestamp_format_for_print("%d %m");
    // Format can be found here: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    let _timestamp_format: String = pal.get_timestamp_format_for_print();

    // Using &str
    pal.print("Success!", "This is a success message.", &PALMessageType::Success);
    pal.print("Info", "This is an info message.", &PALMessageType::Info);
    pal.print("Warn", "This is a warning message.", &PALMessageType::Warn);
    pal.print("Trace", "This is a trace message.", &PALMessageType::Trace);
    pal.print("Debug", "This is a debug message.", &PALMessageType::Debug);
    pal.print("Error!", "This is an error message.", &PALMessageType::Error);
    pal.print("", "This message has no title.", &PALMessageType::Info);

    // Using String
    pal.print(String::from("Success!"), String::from("This is a success string message."), &PALMessageType::Success);

    // Using both
    pal.print("&str", String::from("String"),&PALMessageType::Info);

    //Logging
    pal.log("Test", "It's a message!", &PALMessageType::Info);

    //Printing and logging
    pal.print_and_log("You can see me","On your screen and in your files, if the logging is turned on.",&PALMessageType::Info);

    //Borrowing with & to avoid moving the value
    let title: String = String::from("Title 1");
    let message: String = String::from("Message 1");
    
    pal.print(&title,&message,&PALMessageType::Info);
    pal.print(title,message,&PALMessageType::Info);
}
