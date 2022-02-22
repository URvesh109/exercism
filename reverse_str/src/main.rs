use unicode_segmentation::UnicodeSegmentation;

fn reverse(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true).collect::<String>()
    // input.chars().rev().collect::<String>()
}

fn main() {
    let s = "hello world";
    println!("{}", reverse(s));
    let s = "uüu";
    println!("{}", reverse(s));
}
