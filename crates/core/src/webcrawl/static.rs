use crate::webcrawl::ParserMetadata;
use crate::webcrawl::WebParser;
use http::Response;
use std::io::Error;

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
        let url = self.url.clone();
        let mut response = reqwest::get(&url).map_err(|e| Error::HttpError(e))?;
        let content = response
            .body_mut()
            .collect()
            .map_err(|e| Error::HttpError(e))?;
        self.content = content.as_bytes().to_vec();
        Ok(self.content.as_slice())
    }

    fn metadata(&self) -> Result<ParserMetadata, Error> {
        Ok(ParserMetadata::default())
    }
}

mod test {
    use super::StaticPage;
    use crate::webcrawl::Error;

    #[test]
    fn test_static_page_new() {
        let url = String::from("http://example.com");
        let buffer = Vec::new();
        let content = b"<!DOCTYPE html><html><body><h1>Hello, World!</h1></body></html>";
        let page = StaticPage::new(url, buffer);
        assert_eq!(page.url, url);
        assert_eq!(page.content, content);
    }

    #[test]
    fn test_static_page_read() {
        let url = String::from("http://example.com");
        let buffer = Vec::new();
        let content = b"<!DOCTYPE html><html><body><h1>Hello, World!</h1></body></html>";
        let mut page = StaticPage::new(url, buffer);
        let expected_content = content.to_vec();
        assert_eq!(page.read().unwrap(), expected_content.as_ref());
    }

    #[test]
    fn test_static_page_read_http_error() {
        let url = String::from("http://invalid_url");
        let buffer = Vec::new();
        let content = b"<!DOCTYPE html><html><body><h1>Hello, World!</h1></body></html>";
        let mut page = StaticPage::new(url, buffer);
        let expected_content = content.to_vec();
        assert_eq!(page.read().unwrap(), expected_content.as_ref());
    }
}
