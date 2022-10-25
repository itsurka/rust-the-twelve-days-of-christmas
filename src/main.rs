
const LINES: [&str; 12] = [
    "Twelve drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five golden rings",
    "Four calling birds",
    "Three French hens",
    "Two turtle doves",
    "partridge in a pear tree.",
];
const DAYS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];
const FIRST_LINE: &str = "On the {DAY} day of Christmas";
const SECOND_LINE: &str = "My true love gave to me";

fn main() {
    println!("\nThe Twelve Days of Christmas\n");

    let lines_len: usize = LINES.len();

    for block in 0..lines_len {
        println!("{}\n{}", FIRST_LINE.replace("{DAY}", DAYS[block]), SECOND_LINE);

        let line_from = lines_len - block - 1;
        let line_to = lines_len;

        for line in line_from..line_to {
            if line == line_to - 1 {
                if block == 0 {
                    print!("A ");
                } else {
                    print!("And a ");
                }
            }

            print!("{}", LINES[line]);
            if line < line_to - 2 {
                print!(",");
            }

            println!();
        }

        println!();
    }
}