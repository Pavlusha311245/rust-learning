fn main () {
    let twelve_days_count = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = [
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
    ];

    for (index, day) in twelve_days_count.iter().enumerate() {
        println!("On the {} day of Christmas", day);
        println!("My true love sent to me");

        const GIFT_COUNT: usize = 12;

        for gift in gifts.iter().skip((GIFT_COUNT - 1) - index ) {
            println!("{}", gift);
        }

        println!("{} a partridge in a pear tree. \n", if day == &"first" { "A" } else { "And" });
    }
}