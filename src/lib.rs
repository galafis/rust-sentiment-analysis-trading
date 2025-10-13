//! Sentiment Analysis Trading System
//! 
//! A system for analyzing sentiment from financial news and social media
//! to generate trading signals.
//! 
//! # Modules
//! 
//! - `nlp` - Natural language processing and sentiment analysis
//! - `signals` - Trading signal generation
//! - `scrapers` - Web scraping for data collection
//! - `correlation` - Price correlation analysis
//! - `dashboard` - Data visualization
//! - `types` - Core data types

pub mod scrapers;
pub mod nlp;
pub mod signals;
pub mod correlation;
pub mod dashboard;
pub mod types;

// Re-export commonly used types
pub use types::{Article, SentimentScore, Signal};

// Re-export NLP functions
pub use nlp::{analyze_sentiment, preprocess_text, extract_entities};

// Re-export signal functions
pub use signals::{
    generate_signal, 
    generate_signal_with_type, 
    SignalType, 
    calculate_signal_strength, 
    is_signal_actionable
};

// Re-export scraper utilities
pub use scrapers::{NewsScraper, MockDataProvider, RateLimiter};

// Re-export correlation types and functions
pub use correlation::{
    PricePoint, 
    CorrelationData, 
    PriceDirection,
    calculate_correlation,
    analyze_sentiment_lag,
    calculate_price_change,
    predict_price_direction,
    calculate_price_target,
};


