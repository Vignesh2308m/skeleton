use crate::webcrawl::Error;
use crate::webcrawl::ParserMetadata;
use crate::webcrawl::WebParser;

pub struct StaticPage {
    pub url: String,
    pub content: Vec<u8>,
}

impl StaticPage {
    pub fn new(url: String, content: Vec<u8>) -> Self {
        Self { url, content }
    }
}

impl WebParser for StaticPage {
    fn new(url: &str) -> Result<Self, Error> {
        Ok(Self::new(url.to_string(), Vec::new()))
    }

    fn read(&mut self) -> Result<&[u8], Error> {
        let content = std::str::from_utf8(&self.content).map_err(Error::from)?;
        Ok(content.as_bytes())
    }

    fn metadata(&self) -> Result<ParserMetadata, Error> {
        Ok(ParserMetadata::default())
    }
}

mod test {
    #[test]
    fn test_static_page_new() {
        let url = "http://example.com";
        let content = b"<!DOCTYPE html><html><body><h1>Hello, World!</h1></body></html>";
        let page = StaticPage::new(url, content);
        assert_eq!(page.url, url);
        assert_eq!(page.content, content);
    }

    #[test]
    fn test_static_page_read() {
        let url = "http://example.com";
        let content = b"<!DOCTYPE html><html><body><h1>Hello, World!</h1></body></html>";
        let mut page = StaticPage::new(url, content);
        let expected_content = content.to_vec();
        assert_eq!(page.read().unwrap(), expected_content.as_ref());
    }
}
