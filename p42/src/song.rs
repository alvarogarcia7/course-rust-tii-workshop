struct SongIter {
    current: usize,
    storage: Vec<String>,
}

impl SongIter {
    fn new() -> Self {
        Self {
            current: 0,
            storage:
            vec![
                "On the first day of Christmas, my true love sent to me A partridge in a pear tree.".to_string(),
                "On the second day of Christmas, my true love sent to me Two turtle doves, And a partridge in a pear tree.".to_string(),
                "On the third day of Christmas, my true love sent to me Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "On the fourth day of Christmas, my true love sent to me Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "On the fifth day of Christmas, my true love sent to me Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "On the sixth day of Christmas, my true love sent to me Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "On the seventh day of Christmas, my true love sent to me Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "On the eighth day of Christmas, my true love sent to me Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "On the ninth day of Christmas, my true love sent to me Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "On the tenth day of Christmas, my true love sent to me Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "On the eleventh day of Christmas, my true love sent to me Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "On the twelfth day of Christmas, my true love sent to me Twelve drummers drumming, Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree!".to_string(),
            ],
        }
    }
}

impl Iterator for SongIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < (self.storage.len()) {
            let return_value = Some(self.storage[self.current].to_string());
            self.current += 1;
            return return_value;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_text() {
        let strs = vec![
            vec![
                "On the first day of Christmas,",
                "my true love sent to me",
                "A partridge in a pear tree.",
            ],
            vec![
                "On the second day of Christmas,",
                "my true love sent to me",
                "Two turtle doves,",
                "And a partridge in a pear tree.",
            ],
            vec![
                "On the third day of Christmas,",
                "my true love sent to me",
                "Three French hens,",
                "Two turtle doves,",
                "And a partridge in a pear tree.",
            ],
            vec![
                "On the fourth day of Christmas,",
                "my true love sent to me",
                "Four calling birds,",
                "Three French hens,",
                "Two turtle doves,",
                "And a partridge in a pear tree.",
            ],
            vec![
                "On the fifth day of Christmas,",
                "my true love sent to me",
                "Five gold rings,",
                "Four calling birds,",
                "Three French hens,",
                "Two turtle doves,",
                "And a partridge in a pear tree.",
            ],
            vec![
                "On the sixth day of Christmas,",
                "my true love sent to me",
                "Six geese a-laying,",
                "Five gold rings,",
                "Four calling birds,",
                "Three French hens,",
                "Two turtle doves,",
                "And a partridge in a pear tree.",
            ],
            vec![
                "On the seventh day of Christmas,",
                "my true love sent to me",
                "Seven swans a-swimming,",
                "Six geese a-laying,",
                "Five gold rings,",
                "Four calling birds,",
                "Three French hens,",
                "Two turtle doves,",
                "And a partridge in a pear tree.",
            ],
            vec![
                "On the eighth day of Christmas,",
                "my true love sent to me",
                "Eight maids a-milking,",
                "Seven swans a-swimming,",
                "Six geese a-laying,",
                "Five gold rings,",
                "Four calling birds,",
                "Three French hens,",
                "Two turtle doves,",
                "And a partridge in a pear tree.",
            ],
            vec![
                "On the ninth day of Christmas,",
                "my true love sent to me",
                "Nine ladies dancing,",
                "Eight maids a-milking,",
                "Seven swans a-swimming,",
                "Six geese a-laying,",
                "Five gold rings,",
                "Four calling birds,",
                "Three French hens,",
                "Two turtle doves,",
                "And a partridge in a pear tree.",
            ],
            vec![
                "On the tenth day of Christmas,",
                "my true love sent to me",
                "Ten lords a-leaping,",
                "Nine ladies dancing,",
                "Eight maids a-milking,",
                "Seven swans a-swimming,",
                "Six geese a-laying,",
                "Five gold rings,",
                "Four calling birds,",
                "Three French hens,",
                "Two turtle doves,",
                "And a partridge in a pear tree.",
            ],
            vec![
                "On the eleventh day of Christmas,",
                "my true love sent to me",
                "Eleven pipers piping,",
                "Ten lords a-leaping,",
                "Nine ladies dancing,",
                "Eight maids a-milking,",
                "Seven swans a-swimming,",
                "Six geese a-laying,",
                "Five gold rings,",
                "Four calling birds,",
                "Three French hens,",
                "Two turtle doves,",
                "And a partridge in a pear tree.",
            ],
            vec![
                "On the twelfth day of Christmas,",
                "my true love sent to me",
                "Twelve drummers drumming,",
                "Eleven pipers piping,",
                "Ten lords a-leaping,",
                "Nine ladies dancing,",
                "Eight maids a-milking,",
                "Seven swans a-swimming,",
                "Six geese a-laying,",
                "Five gold rings,",
                "Four calling birds,",
                "Three French hens,",
                "Two turtle doves,",
                "And a partridge in a pear tree!",
            ],
        ];
        assert_eq!(strs.len(), 12);
        let strsx: Vec<String> = strs.iter().map(|x| x.join(" ")).collect();
        assert_eq!(strsx.len(), 12);
        println!("{strsx:?}");

        let strsx_actual: Vec<String> = SongIter::new().collect();
        assert_eq!(strsx_actual, strsx);
    }
}
