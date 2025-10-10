use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub source: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentScore {
    pub positive: Decimal,
    pub negative: Decimal,
    pub neutral: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub symbol: String,
    pub sentiment: SentimentScore,
    pub confidence: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_article_creation() {
        let article = Article {
            title: "Test Article".to_string(),
            content: "Test content".to_string(),
            source: "Test Source".to_string(),
            timestamp: 1234567890,
        };
        
        assert_eq!(article.title, "Test Article");
        assert_eq!(article.content, "Test content");
        assert_eq!(article.source, "Test Source");
        assert_eq!(article.timestamp, 1234567890);
    }

    #[test]
    fn test_sentiment_score_creation() {
        let sentiment = SentimentScore {
            positive: dec!(0.8),
            negative: dec!(0.1),
            neutral: dec!(0.1),
        };
        
        assert_eq!(sentiment.positive, dec!(0.8));
        assert_eq!(sentiment.negative, dec!(0.1));
        assert_eq!(sentiment.neutral, dec!(0.1));
    }

    #[test]
    fn test_signal_creation() {
        let sentiment = SentimentScore {
            positive: dec!(0.7),
            negative: dec!(0.2),
            neutral: dec!(0.1),
        };
        
        let signal = Signal {
            symbol: "BTC".to_string(),
            sentiment,
            confidence: dec!(0.85),
        };
        
        assert_eq!(signal.symbol, "BTC");
        assert_eq!(signal.confidence, dec!(0.85));
        assert_eq!(signal.sentiment.positive, dec!(0.7));
    }

    #[test]
    fn test_article_serialization() {
        let article = Article {
            title: "Test".to_string(),
            content: "Content".to_string(),
            source: "Source".to_string(),
            timestamp: 123,
        };
        
        let json = serde_json::to_string(&article).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("Content"));
    }

    #[test]
    fn test_sentiment_score_serialization() {
        let sentiment = SentimentScore {
            positive: dec!(0.5),
            negative: dec!(0.3),
            neutral: dec!(0.2),
        };
        
        let json = serde_json::to_string(&sentiment).unwrap();
        assert!(json.contains("positive"));
        assert!(json.contains("negative"));
        assert!(json.contains("neutral"));
    }
}

