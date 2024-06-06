use p22::song::Song;

pub fn main() {
    let full_song = Song::new().compute_all();

    for day in full_song {
        println!("{day}")
    }
}
