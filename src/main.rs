#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
}

fn print_media(media: Media) {
    println!("{:?}", media);
}

fn main() {
    let book = Media::Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
    };
    print_media(book);
}
