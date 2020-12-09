use std::io;

#[cfg(not(target_os = "windows"))]
static NEWLINE: &str = "\n";
#[cfg(target_os = "windows")]
static NEWLINE: &str = "\r\n";

pub fn read_line() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop();
        }
        Err(error) => panic!("{}", error),
    }

    input
}

pub fn read_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input == NEWLINE {
                    break;
                }

                input.pop();

                lines.push(input.clone());
            }
            Err(error) => panic!("{}", error),
        }
    }

    lines
}
