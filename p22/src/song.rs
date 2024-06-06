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

pub struct Song {}

impl Default for Song {
    fn default() -> Self {
        Self::new()
    }
}

impl Song {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compute_all(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::with_capacity(12);
        for day in 0..SONG_DAYS.len() {
            result.push(self.compute(day))
        }
        assert_eq!(result.len(), SONG_DAYS.len());
        result
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

        let mut initial: String =
            // https://doc.rust-lang.org/beta/alloc/string/struct.String.html#method.with_capacity
            // s.push(char)
            // s.push(&String)
                selected_days
                    .rev()
                    .map(|day| SONG_PARTS[day].to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
        initial.push(',');
        let mut final_part = SONG_PARTS[0].to_string();
        self.make_first_lowercase(&mut final_part);
        format!("{} And {}", initial, final_part)
    }
}

#[cfg(test)]
pub mod tests {
    // use crate::song::Song;
    use super::*;

    fn base_data() -> Vec<String> {
        let strs: Vec<Vec<String>> = vec![];
        include_str!("song.in");
        assert_eq!(strs.len(), 12);
        let strsx: Vec<String> = strs.iter().map(|x| x.join(" ")).collect();
        assert_eq!(strsx.len(), 12);
        strsx
    }

    #[test]
    fn full_song() {
        let expected = base_data();

        let iter_actual: Vec<String> = Song::new().compute_all();

        assert_eq!(iter_actual, expected);
    }
}
