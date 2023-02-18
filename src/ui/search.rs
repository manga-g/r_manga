use std::io;

use crate::api::Manga;
use crate::ui::{input, output};

pub fn search() -> io::Result<Vec<Manga>> {
    let mut term = crate::ui::create_terminal()?;
    let query = input(&mut term)?;
    let manga_list = crate::api::search_manga(query.trim())?;
    output(&mut term, &manga_list)?;
    Ok(manga_list)
}

