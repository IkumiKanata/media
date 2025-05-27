use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn add_item(&mut self, item: Media) {
        self.items.push(item);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            return Some(&self.items[index]);
        } else {
            None
        }
    }
}
