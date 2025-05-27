mod content;

use content::catalog::Catalog;
use content::media::Media;

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
    catalog.add_item(Media::Podcast(1));
    catalog.add_item(Media::Placeholder);

    match catalog.get_by_index(0) {
        Option::Some(media) => media.describe(),
        Option::None => println!("No item found"),
    }
    match catalog.items.get(100) {
        Option::Some(media) => media.describe(),
        Option::None => println!("No item found"),
    }

    if let Some(media) = catalog.get_by_index(0) {
        media.describe();
    } else {
        println!("No item found");
    }

    if let Some(media) = catalog.get_by_index(100) {    
        media.describe();
    } else {
        println!("No item found");
    }
}
