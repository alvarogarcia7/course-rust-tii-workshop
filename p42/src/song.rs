struct SongIter {
    current: usize,
    parts: Vec<String>,
}

impl SongIter {
    fn new() -> Self {
        Self {
            current: 0,
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
            return self.parts[day].to_string();
        }
        let selected_days = 1..day + 1;

        let initial: String = if day > 1 {
            format!(
                "{},",
                selected_days
                    .rev()
                    .map(|day| self.parts[day].to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        } else {
            format!("{},", self.parts[day])
        };
        let mut final_part = self.parts[0].to_string();
        self.make_first_lowercase(&mut final_part);
        format!("{} And {}", initial, final_part)
    }
}

impl Iterator for SongIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.parts.len() {
            let return_value = self.compute(self.current);
            self.current += 1;
            return Some(return_value);
        }
        None
    }
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
        // let iter_actual: Vec<String> = prefix_with_line_number(1, &SongIter::new());

        let mut line_number = 1;
        let iter_actual = SongIter::new()
            .map(|line| {
                let return_value = format!("{:0>2}: {}", line_number, line);
                line_number += 1;
                return_value
            })
            .collect::<Vec<String>>();

        let prefixes = iter_actual
            .iter()
            .map(|line: &String| line.get(0..3).unwrap().to_string())
            .collect::<Vec<String>>();

        let prefixes_from_actual: Vec<String> =
            prefix_with_line_number::<SongIter>(1, SongIter::new())
                .iter()
                .map(|line: &String| line.get(0..3).unwrap().to_string())
                .collect::<Vec<String>>();

        assert_eq!(
            prefixes,
            vec![
                "01:", "02:", "03:", "04:", "05:", "06:", "07:", "08:", "09:", "10:", "11:", "12:",
            ]
        );

        assert_eq!(prefixes_from_actual, prefixes);
    }

    fn prefix_with_line_number<T: Iterator>(starting_number: i32, iter: T) -> Vec<String>
    where
        <T as std::iter::Iterator>::Item: std::fmt::Display,
    {
        let mut line_number = starting_number;
        iter.map(|line| {
            let return_value = format!("{:0>2}: {}", line_number, line);
            line_number += 1;
            return_value
        })
        .collect::<Vec<String>>()
    }
}
