//! Signals Module - Trading Signal Generation
//!
//! This module generates trading signals (BUY, SELL, HOLD) based on sentiment analysis.
//! Signals are generated with confidence scores to help traders make informed decisions.

use crate::types::{SentimentScore, Signal};
use anyhow::Result;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Type of trading signal
#[derive(Debug, Clone, PartialEq)]
pub enum SignalType {
    /// Buy signal - indicates positive sentiment
    Buy,
    /// Sell signal - indicates negative sentiment
    Sell,
    /// Hold signal - indicates neutral or uncertain sentiment
    Hold,
}

/// Generates a trading signal based on sentiment scores.
///
/// Signal generation rules:
/// - BUY: positive > 0.65 and positive > negative + 0.3
/// - SELL: negative > 0.65 and negative > positive + 0.3
/// - HOLD: otherwise
///
/// # Arguments
///
/// * `sentiment` - The sentiment score to analyze
/// * `symbol` - The trading symbol (e.g., "BTC", "ETH")
///
/// # Returns
///
/// A `Signal` struct with the signal type and confidence score
///
/// # Examples
///
/// ```
/// use sentiment_analysis_trading::{SentimentScore, generate_signal};
/// use rust_decimal_macros::dec;
///
/// let sentiment = SentimentScore {
///     positive: dec!(0.85),
///     negative: dec!(0.05),
///     neutral: dec!(0.10),
/// };
///
/// let signal = generate_signal(&sentiment, "BTC").unwrap();
/// assert!(signal.confidence > dec!(0.7));
/// ```
pub fn generate_signal(sentiment: &SentimentScore, symbol: &str) -> Result<Signal> {
    let confidence: Decimal;
    
    // Determine signal type based on sentiment scores
    let signal_sentiment = if sentiment.positive > dec!(0.65) 
        && sentiment.positive > sentiment.negative + dec!(0.3) {
        // Strong positive sentiment -> BUY
        confidence = sentiment.positive;
        sentiment.clone()
    } else if sentiment.negative > dec!(0.65) 
        && sentiment.negative > sentiment.positive + dec!(0.3) {
        // Strong negative sentiment -> SELL
        confidence = sentiment.negative;
        sentiment.clone()
    } else {
        // Neutral or uncertain -> HOLD
        confidence = sentiment.neutral.max(dec!(0.5));
        sentiment.clone()
    };
    
    Ok(Signal {
        symbol: symbol.to_string(),
        sentiment: signal_sentiment,
        confidence,
    })
}

/// Generates a signal with additional context about the signal type.
pub fn generate_signal_with_type(sentiment: &SentimentScore, symbol: &str) -> Result<(Signal, SignalType)> {
    let signal = generate_signal(sentiment, symbol)?;
    
    let signal_type = if signal.sentiment.positive > dec!(0.65) 
        && signal.sentiment.positive > signal.sentiment.negative + dec!(0.3) {
        SignalType::Buy
    } else if signal.sentiment.negative > dec!(0.65) 
        && signal.sentiment.negative > signal.sentiment.positive + dec!(0.3) {
        SignalType::Sell
    } else {
        SignalType::Hold
    };
    
    Ok((signal, signal_type))
}

/// Calculates signal strength on a scale of 0-100
pub fn calculate_signal_strength(sentiment: &SentimentScore) -> u8 {
    let max_score = sentiment.positive.max(sentiment.negative).max(sentiment.neutral);
    let strength = (max_score * dec!(100)).to_string().parse::<f64>().unwrap_or(0.0);
    strength.min(100.0).max(0.0) as u8
}

/// Validates if a signal has sufficient confidence to be actionable
pub fn is_signal_actionable(signal: &Signal, min_confidence: Decimal) -> bool {
    signal.confidence >= min_confidence
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_buy_signal() {
        let sentiment = SentimentScore {
            positive: dec!(0.85),
            negative: dec!(0.05),
            neutral: dec!(0.10),
        };
        
        let (signal, signal_type) = generate_signal_with_type(&sentiment, "BTC").unwrap();
        assert_eq!(signal_type, SignalType::Buy);
        assert_eq!(signal.symbol, "BTC");
        assert!(signal.confidence > dec!(0.7));
    }
    
    #[test]
    fn test_sell_signal() {
        let sentiment = SentimentScore {
            positive: dec!(0.05),
            negative: dec!(0.85),
            neutral: dec!(0.10),
        };
        
        let (signal, signal_type) = generate_signal_with_type(&sentiment, "ETH").unwrap();
        assert_eq!(signal_type, SignalType::Sell);
        assert_eq!(signal.symbol, "ETH");
        assert!(signal.confidence > dec!(0.7));
    }
    
    #[test]
    fn test_hold_signal() {
        let sentiment = SentimentScore {
            positive: dec!(0.3),
            negative: dec!(0.3),
            neutral: dec!(0.4),
        };
        
        let (signal, signal_type) = generate_signal_with_type(&sentiment, "BTC").unwrap();
        assert_eq!(signal_type, SignalType::Hold);
    }
    
    #[test]
    fn test_signal_strength() {
        let sentiment = SentimentScore {
            positive: dec!(0.85),
            negative: dec!(0.05),
            neutral: dec!(0.10),
        };
        
        let strength = calculate_signal_strength(&sentiment);
        assert!(strength >= 85);
    }
    
    #[test]
    fn test_signal_actionable() {
        let signal = Signal {
            symbol: "BTC".to_string(),
            sentiment: SentimentScore {
                positive: dec!(0.85),
                negative: dec!(0.05),
                neutral: dec!(0.10),
            },
            confidence: dec!(0.85),
        };
        
        assert!(is_signal_actionable(&signal, dec!(0.7)));
        assert!(!is_signal_actionable(&signal, dec!(0.9)));
    }
    
    #[test]
    fn test_marginal_buy_signal() {
        let sentiment = SentimentScore {
            positive: dec!(0.66),
            negative: dec!(0.34),
            neutral: dec!(0.0),
        };
        
        let (_, signal_type) = generate_signal_with_type(&sentiment, "BTC").unwrap();
        assert_eq!(signal_type, SignalType::Buy);
    }
}

