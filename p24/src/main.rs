fn main() {
    let arr = [2, 4, 8, 16];

    let mut n = 2;
    let nth = nth_item(&arr, &n);
    let increased = increased_by_first_item(&arr, &mut n);

    let value = {
        let values = TwoValues::new(&arr[3], increased);

        assert_eq!(*values.get_first(), 16);

        values.get_second()
    };

    assert_eq!(*value, 4);
    assert_eq!(*nth, 8);
}

fn nth_item<'a, 'b>(data: &'a [usize], n: &'b usize) -> &'a usize {
    &data[*n]
}

fn increased_by_first_item<'a, 'b>(data: &'a [usize], n: &'b mut usize) -> &'b mut usize {
    *n += data[0];
    n
}

struct TwoValues<'a, 'b> {
    first: &'a usize,
    second: &'b usize,
}

impl<'a, 'b> TwoValues<'a, 'b> {
    pub fn new(first: &'a usize, second: &'b usize) -> Self {
        Self { first, second }
    }

    pub fn get_first(&self) -> &'a usize {
        self.first
    }

    pub fn get_second(&self) -> &'b usize {
        self.second
    }
}
