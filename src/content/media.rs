#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder,
}

#[derive(Debug)]
pub enum Genre {
    Fiction,
    NonFiction,
    Science,
    History,
    Biography,
    Mystery,
    Romance,
    Horror,
    Comedy,
    Drama,
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

impl Genre {
    pub fn description(&self) -> &str {
        match self {
            Genre::Fiction => "Fiction",
            Genre::NonFiction => "Non-Fiction",
            Genre::Science => "Science",
            Genre::History => "History",
            Genre::Biography => "Biography",
            Genre::Mystery => "Mystery",
            Genre::Romance => "Romance",
            Genre::Horror => "Horror",
            Genre::Comedy => "Comedy",
            Genre::Drama => "Drama",
        }
    }
}