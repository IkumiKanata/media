#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn describe(&self) {
        match self {
            Media::Book { title, author } => println!("Book: {} by {}", title, author),
            Media::Movie { title, director } => println!("Movie: {} by {}", title, director),
            Media::AudioBook { title } => println!("AudioBook: {}", title),
            Media::Podcast(episode) => println!("Podcast: {}", episode),
            Media::Placeholder => println!("Placeholder"),
        }
    }
}