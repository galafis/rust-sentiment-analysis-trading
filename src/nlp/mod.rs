//! NLP Module - Natural Language Processing and Sentiment Analysis
//!
//! This module provides sentiment analysis functionality for text content.
//! It analyzes articles and generates sentiment scores (positive, negative, neutral).

use crate::types::{Article, SentimentScore};
use anyhow::Result;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Analyzes the sentiment of an article and returns a sentiment score.
///
/// This is a basic rule-based sentiment analyzer that uses keyword matching.
/// In a production system, this would use more sophisticated NLP models.
///
/// # Arguments
///
/// * `article` - The article to analyze
///
/// # Returns
///
/// A `SentimentScore` with positive, negative, and neutral scores that sum to 1.0
///
/// # Examples
///
/// ```
/// use sentiment_analysis_trading::{Article, analyze_sentiment};
/// use rust_decimal_macros::dec;
///
/// let article = Article {
///     title: "Bitcoin surges to new highs".to_string(),
///     content: "Bitcoin reaches unprecedented levels".to_string(),
///     source: "CryptoNews".to_string(),
///     timestamp: 1696435200,
/// };
///
/// let sentiment = analyze_sentiment(&article).unwrap();
/// assert!(sentiment.positive > dec!(0.5));
/// ```
pub fn analyze_sentiment(article: &Article) -> Result<SentimentScore> {
    let text = format!("{} {}", article.title, article.content).to_lowercase();
    
    // Positive keywords
    let positive_keywords = vec![
        "surge", "surges", "bull", "bullish", "gain", "gains", "profit", 
        "profits", "high", "highs", "up", "rise", "rises", "growth", 
        "increase", "increases", "positive", "optimistic", "success", 
        "successful", "strong", "stronger", "breakthrough", "record",
        "adoption", "unprecedented", "excellent", "great", "good"
    ];
    
    // Negative keywords
    let negative_keywords = vec![
        "crash", "crashes", "bear", "bearish", "loss", "losses", "down", 
        "fall", "falls", "decline", "declines", "negative", "pessimistic", 
        "failure", "weak", "weaker", "concern", "concerns", "worry", "worries",
        "correction", "downturn", "plunge", "plunges", "drop", "drops",
        "risk", "risks", "fear", "fears", "warning", "warnings"
    ];
    
    let mut positive_count = 0;
    let mut negative_count = 0;
    
    // Count positive keywords
    for keyword in &positive_keywords {
        positive_count += text.matches(keyword).count();
    }
    
    // Count negative keywords
    for keyword in &negative_keywords {
        negative_count += text.matches(keyword).count();
    }
    
    let total_count = positive_count + negative_count;
    
    let (positive, negative, neutral) = if total_count == 0 {
        // No sentiment keywords found - neutral
        (dec!(0.1), dec!(0.1), dec!(0.8))
    } else {
        let pos_ratio = positive_count as f64 / total_count as f64;
        let neg_ratio = negative_count as f64 / total_count as f64;
        
        // Convert to Decimal with normalized values
        let pos_score = Decimal::from_f64_retain(pos_ratio * 0.85 + 0.05).unwrap_or(dec!(0.05));
        let neg_score = Decimal::from_f64_retain(neg_ratio * 0.85 + 0.05).unwrap_or(dec!(0.05));
        let neu_score = dec!(1.0) - pos_score - neg_score;
        
        (pos_score, neg_score, neu_score.max(dec!(0.0)))
    };
    
    Ok(SentimentScore {
        positive,
        negative,
        neutral,
    })
}

/// Preprocesses text by removing special characters and normalizing whitespace.
pub fn preprocess_text(text: &str) -> String {
    text.trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Extracts potential entity mentions (symbols like BTC, ETH) from text.
pub fn extract_entities(text: &str) -> Vec<String> {
    let common_symbols = vec!["BTC", "ETH", "USDT", "BNB", "XRP", "ADA", "DOGE", "SOL"];
    let upper_text = text.to_uppercase();
    
    common_symbols
        .into_iter()
        .filter(|symbol| upper_text.contains(symbol))
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_positive_sentiment() {
        let article = Article {
            title: "Bitcoin surges to record high".to_string(),
            content: "Great gains as bullish trend continues".to_string(),
            source: "Test".to_string(),
            timestamp: 123,
        };
        
        let sentiment = analyze_sentiment(&article).unwrap();
        assert!(sentiment.positive > sentiment.negative);
        assert!(sentiment.positive > dec!(0.5));
    }
    
    #[test]
    fn test_negative_sentiment() {
        let article = Article {
            title: "Market crashes amid fears".to_string(),
            content: "Bearish concerns as losses mount".to_string(),
            source: "Test".to_string(),
            timestamp: 123,
        };
        
        let sentiment = analyze_sentiment(&article).unwrap();
        assert!(sentiment.negative > sentiment.positive);
        assert!(sentiment.negative > dec!(0.5));
    }
    
    #[test]
    fn test_neutral_sentiment() {
        let article = Article {
            title: "Market analysis report".to_string(),
            content: "The market shows mixed signals today".to_string(),
            source: "Test".to_string(),
            timestamp: 123,
        };
        
        let sentiment = analyze_sentiment(&article).unwrap();
        assert!(sentiment.neutral > dec!(0.5));
    }
    
    #[test]
    fn test_preprocess_text() {
        let text = "  Bitcoin  SURGES   to  new  highs!  ";
        let processed = preprocess_text(text);
        assert_eq!(processed, "bitcoin surges to new highs!");
    }
    
    #[test]
    fn test_extract_entities() {
        let text = "BTC and ETH are leading the market";
        let entities = extract_entities(text);
        assert!(entities.contains(&"BTC".to_string()));
        assert!(entities.contains(&"ETH".to_string()));
        assert_eq!(entities.len(), 2);
    }
    
    #[test]
    fn test_sentiment_scores_sum_to_one() {
        let article = Article {
            title: "Test".to_string(),
            content: "Content".to_string(),
            source: "Test".to_string(),
            timestamp: 123,
        };
        
        let sentiment = analyze_sentiment(&article).unwrap();
        let sum = sentiment.positive + sentiment.negative + sentiment.neutral;
        // Allow small rounding error
        assert!((sum - dec!(1.0)).abs() < dec!(0.01));
    }
}

