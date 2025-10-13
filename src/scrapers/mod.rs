//! Scrapers Module - Web Scraping and Data Collection
//!
//! This module provides functionality for collecting data from various sources.
//! It includes example scrapers and utilities for web scraping.

use crate::types::Article;
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

/// A simple mock news scraper for demonstration purposes.
/// In production, this would connect to real news APIs or websites.
pub struct NewsScraper {
    source: String,
}

impl NewsScraper {
    /// Creates a new NewsScraper instance
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }
    
    /// Scrapes articles from the configured source.
    /// This is a mock implementation that returns example data.
    pub async fn scrape(&self) -> Result<Vec<Article>> {
        // In a real implementation, this would make HTTP requests
        // For now, return mock data
        Ok(vec![])
    }
    
    /// Creates a sample article for testing purposes
    pub fn create_sample_article(&self, title: &str, content: &str) -> Article {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        Article {
            title: title.to_string(),
            content: content.to_string(),
            source: self.source.clone(),
            timestamp,
        }
    }
}

/// Mock data provider for testing and examples
pub struct MockDataProvider;

impl MockDataProvider {
    /// Returns a set of sample articles for testing
    pub fn get_sample_articles() -> Vec<Article> {
        vec![
            Article {
                title: "Bitcoin Surges to New All-Time High".to_string(),
                content: "Bitcoin has reached unprecedented levels as institutional adoption continues to grow. Major companies announce BTC purchases.".to_string(),
                source: "CryptoNews".to_string(),
                timestamp: 1696435200,
            },
            Article {
                title: "Ethereum Upgrade Boosts Network Performance".to_string(),
                content: "The latest Ethereum upgrade shows promising results with improved transaction speeds and reduced gas fees.".to_string(),
                source: "BlockchainDaily".to_string(),
                timestamp: 1696435300,
            },
            Article {
                title: "Market Correction Expected Amid Regulatory Concerns".to_string(),
                content: "Analysts warn of potential market downturn as regulatory pressure increases. Investors show caution in recent trading.".to_string(),
                source: "FinanceTimes".to_string(),
                timestamp: 1696435400,
            },
            Article {
                title: "DeFi Protocols Report Strong Growth".to_string(),
                content: "Decentralized finance platforms continue to see increased adoption with total value locked reaching new highs.".to_string(),
                source: "DeFiWatch".to_string(),
                timestamp: 1696435500,
            },
        ]
    }
    
    /// Returns sample articles filtered by sentiment type
    pub fn get_positive_articles() -> Vec<Article> {
        Self::get_sample_articles()
            .into_iter()
            .filter(|a| {
                let text = format!("{} {}", a.title, a.content).to_lowercase();
                text.contains("surge") || text.contains("boost") || text.contains("growth")
            })
            .collect()
    }
    
    /// Returns sample articles filtered by sentiment type
    pub fn get_negative_articles() -> Vec<Article> {
        Self::get_sample_articles()
            .into_iter()
            .filter(|a| {
                let text = format!("{} {}", a.title, a.content).to_lowercase();
                text.contains("correction") || text.contains("concern") || text.contains("downturn")
            })
            .collect()
    }
}

/// Rate limiter for API requests
pub struct RateLimiter {
    requests_per_second: u32,
}

impl RateLimiter {
    /// Creates a new rate limiter
    pub fn new(requests_per_second: u32) -> Self {
        Self { requests_per_second }
    }
    
    /// Returns the configured rate limit
    pub fn get_rate(&self) -> u32 {
        self.requests_per_second
    }
    
    /// Checks if a request is allowed (mock implementation)
    pub fn is_allowed(&self) -> bool {
        true // In production, this would track actual request timing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_news_scraper_creation() {
        let scraper = NewsScraper::new("TestSource");
        assert_eq!(scraper.source, "TestSource");
    }
    
    #[test]
    fn test_create_sample_article() {
        let scraper = NewsScraper::new("TestSource");
        let article = scraper.create_sample_article("Test Title", "Test Content");
        
        assert_eq!(article.title, "Test Title");
        assert_eq!(article.content, "Test Content");
        assert_eq!(article.source, "TestSource");
        assert!(article.timestamp > 0);
    }
    
    #[test]
    fn test_mock_data_provider() {
        let articles = MockDataProvider::get_sample_articles();
        assert!(articles.len() >= 3);
        assert!(articles.iter().any(|a| a.title.contains("Bitcoin")));
    }
    
    #[test]
    fn test_positive_articles_filter() {
        let articles = MockDataProvider::get_positive_articles();
        assert!(articles.len() > 0);
        for article in articles {
            let text = format!("{} {}", article.title, article.content).to_lowercase();
            assert!(
                text.contains("surge") || 
                text.contains("boost") || 
                text.contains("growth")
            );
        }
    }
    
    #[test]
    fn test_negative_articles_filter() {
        let articles = MockDataProvider::get_negative_articles();
        assert!(articles.len() > 0);
        for article in articles {
            let text = format!("{} {}", article.title, article.content).to_lowercase();
            assert!(
                text.contains("correction") || 
                text.contains("concern") || 
                text.contains("downturn")
            );
        }
    }
    
    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(10);
        assert_eq!(limiter.get_rate(), 10);
        assert!(limiter.is_allowed());
    }
    
    #[tokio::test]
    async fn test_scraper_scrape() {
        let scraper = NewsScraper::new("TestSource");
        let result = scraper.scrape().await;
        assert!(result.is_ok());
    }
}

