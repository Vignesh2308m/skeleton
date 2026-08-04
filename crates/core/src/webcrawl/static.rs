use crate::webcrawl::{Error, ParserMetadata, WebParser}; // Correct import for our custom Error
use reqwest; // Required for making HTTP requests
// Removed 'use http::Response;' as it conflicts/is not needed for reqwest

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

    // `read` must be async to use `reqwest` and `await`
    // #[tokio::main] is used here for a quick fix, but a proper async application
    // would manage the runtime from its main function or a higher-level async block.
    #[tokio::main]
    async fn read(&mut self) -> Result<&[u8], Error> {
        let url = self.url.clone();
        let response = reqwest::get(&url).await?; // Await the HTTP GET request
        let bytes = response.bytes().await?; // Await getting the response body as bytes
        self.content = bytes.to_vec();
        Ok(self.content.as_slice())
    }

    fn metadata(&self) -> Result<ParserMetadata, Error> {
        Ok(ParserMetadata {
            path: self.url.clone(),
            kind: "web",
            file_name: self.url.split('/').last().unwrap_or_default().to_string(),
            extension: "html".to_string(), // Default to html, could be more dynamic
            size_bytes: self.content.len() as u64,
            last_modified: None, // Not available directly from a static fetch
            language: None,      // Should be determined from content/headers
            details: super::ParserMetadataDetails::Web,
            checksum: String::new(), // Compute if needed
            tags: std::collections::HashMap::new(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::StaticPage;
    use crate::webcrawl::Error; // Use our custom Error enum

    // All tests using async code must be marked with #[tokio::test]
    #[tokio::test]
    async fn test_static_page_new() {
        let url = String::from("http://example.com");
        let buffer = Vec::new();
        // The 'content' literal is not used for initialization here.
        // StaticPage::new initializes with the provided buffer (which is empty).
        let page = StaticPage::new(url.clone(), buffer.clone());
        assert_eq!(page.url, url);
        // This assertion is incorrect because page.content is empty after StaticPage::new
        // If you intended to test that content is passed in, you should pass 'content' to new.
        // assert_eq!(page.content, content);
    }

    #[tokio::test]
    async fn test_static_page_read() {
        let url = String::from("http://example.com"); // This URL will be fetched
        let buffer = Vec::new();
        let mut page = StaticPage::new(url, buffer);

        let content_from_read = page.read().await.expect("failed to read content");

        // Assert that some content was fetched. It's not guaranteed to be "Hello, World!"
        // For precise content testing, you would need to mock the HTTP request.
        assert!(
            !content_from_read.is_empty(),
            "Content should not be empty after reading"
        );
    }

    #[tokio::test]
    async fn test_static_page_read_http_error() {
        let url = String::from("http://invalid_url"); // This will cause an HTTP error
        let buffer = Vec::new();
        let mut page = StaticPage::new(url, buffer);

        let result = page.read().await; // Await the read operation
        assert!(
            result.is_err(),
            "Reading from an invalid URL should result in an error"
        );
        // Do not call .unwrap() here, as it would panic if the result is Err.
    }
}
