#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn describe(&self) {
        match self {
            Media::Book { title, author } => println!("Book: {} by {}", title, author),
            Media::Movie { title, director } => println!("Movie: {} by {}", title, director),
            Media::AudioBook { title } => println!("AudioBook: {}", title),
            Media::Podcast(episode) => println!("Podcast: {}", episode),
            Media::Placeholder => println!("Placeholder"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    fn add_item(&mut self, item: Media) {
        self.items.push(item);
    }
}
fn print_media(media: Media) {
    println!("{:?}", media);
}

fn main() {
    let mut catalog = Catalog::new();
    let book = Media::Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
    };
    let movie = Media::Movie {
        title: String::from("The Matrix"),
        director: String::from("The Wachowskis"),
    };
    let audio_book = Media::AudioBook {
        title: String::from("The Great Gatsby"),
    };
    catalog.add_item(book);
    catalog.add_item(movie);
    catalog.add_item(audio_book);
    println!("{:?}", catalog);
}
