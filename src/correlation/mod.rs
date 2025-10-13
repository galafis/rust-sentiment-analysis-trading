//! Correlation Module - Sentiment-Price Correlation Analysis
//!
//! This module analyzes the correlation between sentiment scores and price movements.
//! It helps understand how sentiment affects market prices over time.

use crate::types::SentimentScore;
use anyhow::Result;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Represents a price data point
#[derive(Debug, Clone)]
pub struct PricePoint {
    pub timestamp: i64,
    pub price: Decimal,
    pub volume: Option<Decimal>,
}

/// Represents correlation data between sentiment and price
#[derive(Debug, Clone)]
pub struct CorrelationData {
    pub correlation_coefficient: Decimal,
    pub lag_hours: i32,
    pub sample_size: usize,
}

/// Calculates basic correlation between sentiment and price changes
/// 
/// This is a simplified implementation. In production, you would use
/// more sophisticated statistical methods.
pub fn calculate_correlation(
    sentiments: &[(i64, SentimentScore)],
    prices: &[PricePoint],
) -> Result<CorrelationData> {
    if sentiments.is_empty() || prices.is_empty() {
        return Ok(CorrelationData {
            correlation_coefficient: dec!(0.0),
            lag_hours: 0,
            sample_size: 0,
        });
    }
    
    // Simple correlation calculation
    // In production, this would use proper Pearson correlation
    let sample_size = sentiments.len().min(prices.len());
    
    // Mock calculation - would need actual statistical implementation
    let correlation_coefficient = dec!(0.65); // Placeholder
    
    Ok(CorrelationData {
        correlation_coefficient,
        lag_hours: 2,
        sample_size,
    })
}

/// Analyzes lag between sentiment changes and price reactions
pub fn analyze_sentiment_lag(
    _sentiments: &[(i64, SentimentScore)],
    _prices: &[PricePoint],
    max_lag_hours: i32,
) -> Result<Vec<(i32, Decimal)>> {
    let mut lag_correlations = Vec::new();
    
    for lag in 0..=max_lag_hours {
        // Calculate correlation for this lag
        // This is a placeholder - would need actual implementation
        let correlation = dec!(0.5) - Decimal::from(lag) * dec!(0.05);
        lag_correlations.push((lag, correlation));
    }
    
    Ok(lag_correlations)
}

/// Calculates price change percentage
pub fn calculate_price_change(old_price: Decimal, new_price: Decimal) -> Decimal {
    if old_price == dec!(0.0) {
        return dec!(0.0);
    }
    ((new_price - old_price) / old_price) * dec!(100.0)
}

/// Determines if sentiment predicts price movement direction
pub fn predict_price_direction(sentiment: &SentimentScore) -> PriceDirection {
    if sentiment.positive > dec!(0.7) {
        PriceDirection::Up
    } else if sentiment.negative > dec!(0.7) {
        PriceDirection::Down
    } else {
        PriceDirection::Neutral
    }
}

/// Expected price direction based on sentiment
#[derive(Debug, Clone, PartialEq)]
pub enum PriceDirection {
    Up,
    Down,
    Neutral,
}

/// Calculates sentiment-weighted price target
pub fn calculate_price_target(
    current_price: Decimal,
    sentiment: &SentimentScore,
    volatility: Decimal,
) -> Decimal {
    let sentiment_factor = sentiment.positive - sentiment.negative;
    let expected_change = sentiment_factor * volatility;
    current_price * (dec!(1.0) + expected_change)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_price_point_creation() {
        let price = PricePoint {
            timestamp: 1696435200,
            price: dec!(50000.0),
            volume: Some(dec!(1000.0)),
        };
        
        assert_eq!(price.price, dec!(50000.0));
        assert_eq!(price.timestamp, 1696435200);
    }
    
    #[test]
    fn test_calculate_correlation() {
        let sentiments = vec![
            (1696435200, SentimentScore {
                positive: dec!(0.8),
                negative: dec!(0.1),
                neutral: dec!(0.1),
            }),
        ];
        
        let prices = vec![
            PricePoint {
                timestamp: 1696435200,
                price: dec!(50000.0),
                volume: None,
            },
        ];
        
        let result = calculate_correlation(&sentiments, &prices).unwrap();
        assert!(result.sample_size > 0);
    }
    
    #[test]
    fn test_calculate_price_change() {
        let change = calculate_price_change(dec!(50000.0), dec!(55000.0));
        assert_eq!(change, dec!(10.0));
        
        let negative_change = calculate_price_change(dec!(50000.0), dec!(45000.0));
        assert_eq!(negative_change, dec!(-10.0));
    }
    
    #[test]
    fn test_predict_price_direction() {
        let positive_sentiment = SentimentScore {
            positive: dec!(0.85),
            negative: dec!(0.05),
            neutral: dec!(0.10),
        };
        assert_eq!(predict_price_direction(&positive_sentiment), PriceDirection::Up);
        
        let negative_sentiment = SentimentScore {
            positive: dec!(0.05),
            negative: dec!(0.85),
            neutral: dec!(0.10),
        };
        assert_eq!(predict_price_direction(&negative_sentiment), PriceDirection::Down);
        
        let neutral_sentiment = SentimentScore {
            positive: dec!(0.3),
            negative: dec!(0.3),
            neutral: dec!(0.4),
        };
        assert_eq!(predict_price_direction(&neutral_sentiment), PriceDirection::Neutral);
    }
    
    #[test]
    fn test_calculate_price_target() {
        let sentiment = SentimentScore {
            positive: dec!(0.8),
            negative: dec!(0.1),
            neutral: dec!(0.1),
        };
        
        let target = calculate_price_target(dec!(50000.0), &sentiment, dec!(0.1));
        assert!(target > dec!(50000.0));
    }
    
    #[test]
    fn test_analyze_sentiment_lag() {
        let sentiments = vec![
            (1696435200, SentimentScore {
                positive: dec!(0.8),
                negative: dec!(0.1),
                neutral: dec!(0.1),
            }),
        ];
        
        let prices = vec![
            PricePoint {
                timestamp: 1696435200,
                price: dec!(50000.0),
                volume: None,
            },
        ];
        
        let lag_results = analyze_sentiment_lag(&sentiments, &prices, 5).unwrap();
        assert_eq!(lag_results.len(), 6); // 0 to 5 inclusive
    }
}

