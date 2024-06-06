static SONG_PARTS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

static SONG_DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

struct SongIter {
    current: usize,
}

impl SongIter {
    fn new() -> Self {
        Self { current: 0 }
    }

    fn nth_day(&self, day: usize) -> String {
        let ordinal = SONG_DAYS[day];

        format!(
            "On the {} day of Christmas, my true love sent to me",
            ordinal
        )
        .to_string()
    }

    pub fn compute(&self, day: usize) -> String {
        let terminator = if day == 11 { "!" } else { "." };
        format!("{} {}{}", self.nth_day(day), self.rest(day), terminator)
    }

    // https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
    fn make_first_lowercase(&self, string: &mut str) {
        if let Some(r) = string.get_mut(0..1) {
            r.make_ascii_lowercase();
        }
    }
    fn rest(&self, day: usize) -> String {
        if day == 0 {
            return SONG_PARTS[day].to_string();
        }
        let selected_days = 1..day + 1;

        let initial: String =
            // https://doc.rust-lang.org/beta/alloc/string/struct.String.html#method.with_capacity
            // s.push(char)
            // s.push(&String)
            format!(
                "{},",
                selected_days
                    .rev()
                    .map(|day| SONG_PARTS[day].to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        let mut final_part = SONG_PARTS[0].to_string();
        self.make_first_lowercase(&mut final_part);
        format!("{} And {}", initial, final_part)
    }
}

impl Iterator for SongIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < SONG_PARTS.len() {
            let return_value = self.compute(self.current);
            self.current += 1;
            return Some(return_value);
        }
        None
    }
}

fn prefix_with_line_number(
    mut line_num: i32,
    iter: impl Iterator<Item = String>,
) -> impl Iterator<Item = String> {
    iter.map(move |line| {
        let return_value = format!("{:0>2}: {}", line_num, line);
        line_num += 1;
        return_value
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_data() -> Vec<String> {
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
        strsx
    }

    #[test]
    fn full_text_without_decoration() {
        let expected = base_data();

        let iter_actual: Vec<String> = SongIter::new().collect();

        assert_eq!(iter_actual, expected);
    }

    #[test]
    fn full_text_with_decorated_lines() {
        let prefixes_from_actual: Vec<String> = prefix_with_line_number(1, SongIter::new())
            .map(|line| line.get(0..3).unwrap().to_string())
            .collect::<Vec<String>>();

        assert_eq!(
            prefixes_from_actual,
            vec![
                "01:", "02:", "03:", "04:", "05:", "06:", "07:", "08:", "09:", "10:", "11:", "12:",
            ]
        );
    }
}
