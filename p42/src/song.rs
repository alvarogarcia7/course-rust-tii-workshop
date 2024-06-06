struct SongIter {
    current: usize,
    storage: Vec<String>,
    computed_storage: Vec<String>,
    parts: Vec<String>,
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
            computed_storage:
            vec![
                "A partridge in a pear tree.".to_string(),
                "Two turtle doves, And a partridge in a pear tree.".to_string(),
                "Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.".to_string(),
                "Twelve drummers drumming, Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five gold rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree!".to_string(),
            ],
            parts: vec![
                "A partridge in a pear tree".to_string(),
                "Two turtle doves".to_string(),
                "Three French hens".to_string(),
                "Four calling birds".to_string(),
                "Five gold rings".to_string(),
                "Six geese a-laying".to_string(),
                "Seven swans a-swimming".to_string(),
                "Eight maids a-milking".to_string(),
                "Nine ladies dancing".to_string(),
                "Ten lords a-leaping".to_string(),
                "Eleven pipers piping".to_string(),
                "Twelve drummers drumming".to_string(),
            ],
        }
    }

    fn nth_day(&self, day: usize) -> String {
        let ordinal = match day + 1 {
            1 => "first".to_string(),
            2 => "second".to_string(),
            3 => "third".to_string(),
            4 => "fourth".to_string(),
            5 => "fifth".to_string(),
            6 => "sixth".to_string(),
            7 => "seventh".to_string(),
            8 => "eighth".to_string(),
            9 => "ninth".to_string(),
            10 => "tenth".to_string(),
            11 => "eleventh".to_string(),
            12 => "twelfth".to_string(),
            _ => todo!(),
        };

        format!(
            "On the {} day of Christmas, my true love sent to me",
            ordinal
        )
        .to_string()
    }

    pub fn compute(&self, day: usize) -> String {
        format!("{} {}.", self.nth_day(day), self.rest(day))
    }
    pub fn compute_intermediate(&self, day: usize) -> String {
        format!("{} {}", self.nth_day(day), self.computed_storage[day])
    }

    // https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
    #[allow(dead_code)]
    fn make_first_uppercase(&self, string: &mut str) {
        if let Some(r) = string.get_mut(0..1) {
            r.make_ascii_uppercase();
        }
    }
    // https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
    fn make_first_lowercase(&self, string: &mut str) {
        if let Some(r) = string.get_mut(0..1) {
            r.make_ascii_lowercase();
        }
    }
    fn rest(&self, day: usize) -> String {
        if day == 0 {
            return self.parts[day].to_string();
        }
        let selected_days = 1..day;
        // let mut selected_days_string = Vec::with_capacity(10);

        let initial: String = if day > 1 {
            selected_days
                .rev()
                .map(|day| self.parts[day].to_string())
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            format!("{},", self.parts[day])
        };
        let mut final_ = self.parts[0].to_string();
        self.make_first_lowercase(&mut final_);
        let maybe_separator = match day {
            0 => "", // dead
            _ => " And ",
        };
        format!("{}{}{}", initial, maybe_separator, final_)
    }
}

impl Iterator for SongIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < (self.storage.len()) {
            let hardcoded_value = self.storage[self.current].to_string();
            let computed = if [0usize, 1].contains(&self.current) {
                self.compute(self.current)
            } else {
                self.compute_intermediate(self.current)
            };
            assert_eq!(computed, hardcoded_value);
            self.current += 1;
            return Some(hardcoded_value);
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

    #[test]
    fn second() {
        let mut iter = SongIter::new();
        // skip first
        let _ = iter.next();
        let x = iter.next();
        println!("{x:?}")
    }
}
