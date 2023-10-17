use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "lap.html")]
pub struct Lap {
    id: u32,
    time: u32,
}

impl Lap {
    pub fn new(id: u32, time: u32) -> Self {
        Self { id, time }
    }
}
