/// An _item_ in a document's Table of Contents
pub struct TocItem {
    level: u8,
    text: String,
    children: Vec<TocItem>,
}

impl TocItem {
    pub fn new(level: &u8, text: &str) -> Self {
        TocItem {
            level: level.clone(),
            text: text.to_string(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, text: &str) {
        let level = self.level + 1;
        self.children.push(TocItem::new(&level, text));
    }
}
