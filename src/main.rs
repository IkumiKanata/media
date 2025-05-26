#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
}

impl Media {
    fn describe(&self) {
        match self {
            Media::Book { title, author } => println!("Book: {} by {}", title, author),
            Media::Movie { title, director } => println!("Movie: {} by {}", title, director),
            Media::AudioBook { title } => println!("AudioBook: {}", title),
        }
    }
}

fn print_media(media: Media) {
    println!("{:?}", media);
}

fn main() {
    let book = Media::Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
    };
    book.describe();
}
