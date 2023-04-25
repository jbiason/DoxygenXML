//! Doxygen XML produces the elements from the Doxygen XML

use std::fs::File;
use std::path::Path;

use elementtree::Element;

#[allow(non_snake_case)]
fn DoxygemXML(path: &Path) -> Result<DoxygenElementIterator, XMLError> {
    DoxygenElementIterator::new(path)
}

struct DoxygenElementIterator {
    position: Element,
}

pub enum XMLError {
    FileNotFound,
    InvalidXML,
}

impl From<std::io::Error> for XMLError {
    fn from(value: std::io::Error) -> Self {
        XMLError::FileNotFound
    }
}

impl From<elementtree::Error> for XMLError {
    fn from(value: elementtree::Error) -> Self {
        XMLError::InvalidXML
    }
}

impl DoxygenElementIterator {
    pub fn new(path: &Path) -> Result<Self, XMLError> {
        let mut file = File::open(&path.join("index.xml"))?;
        let root = Element::from_reader(&file)?;
    }
}

enum DoxygenElement {
    Namespace(String),
}

impl Iterator for DoxygenElementIterator  {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        // advance the elementtree iterator
        // read file pointed by said iterator
        // produce the parsed structure
        // Some(element)
        None
    }
}
