const BLACK_BACKGROUND: &str = "\x1B[40m";
const RED_BACKGROUND: &str = "\x1B[41m";
const GREEN_BACKGROUND: &str = "\x1B[42m";
const YELLOW_BACKGROUND: &str = "\x1B[43m";
const BLUE_BACKGROUND: &str = "\x1B[44m";
const MAGENTA_BACKGROUND: &str = "\x1B[45m";
const CYAN_BACKGROUND: &str = "\x1B[46m";
const WHITE_BACKGROUND: &str = "\x1B[47m";

const RED_FOREGROUND: &str = "\x1B[31m";
const GREEN_FOREGROUND: &str = "\x1B[32m";
const BLUE_FOREGROUND: &str = "\x1B[96m";
const PURPLE_FOREGROUND: &str = "\x1B[01;95m";
const VIOLET_FOREGROUND: &str = "\x1B[01;94m";
const YELLOW_FOREGROUND: &str = "\x1B[01;93m";
const ORANGE_FOREGROUND: &str = "\x1B[01;91m";
const GREY_FOREGROUND: &str = "\x1B[01;90m";
const WHITE_FOREGROUND: &str = "\x1B[01;89m";
const RESET: &str = "\x1B[0m";

pub fn blue_background(value: &str) -> String {
    format!("{}{}{}", BLUE_BACKGROUND, value, RESET)
}

pub fn red_background(value: &str) -> String {
    format!("{}{}{}", RED_BACKGROUND, value, RESET)
}

pub fn green_background(value: &str) -> String {
    format!("{}{}{}", GREEN_BACKGROUND, value, RESET)
}

pub fn yellow_background(value: &str) -> String {
    format!("{}{}{}", YELLOW_BACKGROUND, value, RESET)
}

pub fn green_text(value: &str) -> String {
    format!("{}{}{}", GREEN_FOREGROUND, value, RESET)
}

pub fn blue_text(value: &str) -> String {
    format!("{}{}{}", BLUE_FOREGROUND, value, RESET)
}

pub fn yellow_text(value: &str) -> String {
    format!("{}{}{}", YELLOW_FOREGROUND, value, RESET)
}
