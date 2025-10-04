//! Sentiment Analysis Trading System
pub mod scrapers;
pub mod nlp;
pub mod signals;
pub mod correlation;
pub mod dashboard;
pub mod types;

pub use types::{Article, SentimentScore, Signal};
