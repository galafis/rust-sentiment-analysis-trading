//! Dashboard Module - Data Visualization and Reporting
//!
//! This module provides utilities for displaying sentiment analysis results,
//! signals, and analytics in a readable format.

use crate::types::{Article, SentimentScore, Signal};
use crate::signals::SignalType;

/// Formats sentiment score for display
pub fn format_sentiment(sentiment: &SentimentScore) -> String {
    format!(
        "📊 Sentiment Scores:\n  🟢 Positive: {:.2}%\n  🔴 Negative: {:.2}%\n  ⚪ Neutral: {:.2}%",
        sentiment.positive * rust_decimal_macros::dec!(100),
        sentiment.negative * rust_decimal_macros::dec!(100),
        sentiment.neutral * rust_decimal_macros::dec!(100)
    )
}

/// Formats a signal for display
pub fn format_signal(signal: &Signal, signal_type: &SignalType) -> String {
    let emoji = match signal_type {
        SignalType::Buy => "🟢",
        SignalType::Sell => "🔴",
        SignalType::Hold => "🟡",
    };
    
    let action = match signal_type {
        SignalType::Buy => "BUY",
        SignalType::Sell => "SELL",
        SignalType::Hold => "HOLD",
    };
    
    format!(
        "{} {} Signal for {} (Confidence: {:.1}%)",
        emoji,
        action,
        signal.symbol,
        signal.confidence * rust_decimal_macros::dec!(100)
    )
}

/// Formats an article summary for display
pub fn format_article(article: &Article) -> String {
    format!(
        "📰 Article\n  Title: {}\n  Source: {}\n  Timestamp: {}",
        article.title,
        article.source,
        article.timestamp
    )
}

/// Creates a simple text-based dashboard
pub fn create_dashboard(
    articles: &[Article],
    sentiments: &[SentimentScore],
    signals: &[(Signal, SignalType)],
) -> String {
    let mut output = String::new();
    
    output.push_str("╔════════════════════════════════════════════════════════════╗\n");
    output.push_str("║     Sentiment Analysis Trading Dashboard                  ║\n");
    output.push_str("╚════════════════════════════════════════════════════════════╝\n\n");
    
    output.push_str(&format!("📊 Summary:\n"));
    output.push_str(&format!("  Articles Analyzed: {}\n", articles.len()));
    output.push_str(&format!("  Signals Generated: {}\n\n", signals.len()));
    
    // Calculate average sentiment
    if !sentiments.is_empty() {
        let avg_positive = sentiments.iter()
            .map(|s| s.positive)
            .sum::<rust_decimal::Decimal>() / rust_decimal::Decimal::from(sentiments.len());
        let avg_negative = sentiments.iter()
            .map(|s| s.negative)
            .sum::<rust_decimal::Decimal>() / rust_decimal::Decimal::from(sentiments.len());
        
        output.push_str("📈 Average Sentiment:\n");
        output.push_str(&format!("  Positive: {:.1}%\n", avg_positive * rust_decimal_macros::dec!(100)));
        output.push_str(&format!("  Negative: {:.1}%\n\n", avg_negative * rust_decimal_macros::dec!(100)));
    }
    
    // Show signals
    if !signals.is_empty() {
        output.push_str("🎯 Active Signals:\n");
        for (signal, signal_type) in signals {
            output.push_str(&format!("  {}\n", format_signal(signal, signal_type)));
        }
    }
    
    output.push_str("\n");
    output.push_str("════════════════════════════════════════════════════════════\n");
    
    output
}

/// Displays a progress bar for processing status
pub fn progress_bar(current: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return String::from("[          ] 0%");
    }
    
    let percentage = (current as f64 / total as f64 * 100.0) as usize;
    let filled = (current as f64 / total as f64 * width as f64) as usize;
    let empty = width.saturating_sub(filled);
    
    format!(
        "[{}{}] {}%",
        "=".repeat(filled),
        " ".repeat(empty),
        percentage
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_format_sentiment() {
        let sentiment = SentimentScore {
            positive: dec!(0.8),
            negative: dec!(0.1),
            neutral: dec!(0.1),
        };
        
        let formatted = format_sentiment(&sentiment);
        assert!(formatted.contains("Positive"));
        assert!(formatted.contains("80"));
    }
    
    #[test]
    fn test_format_signal() {
        let signal = Signal {
            symbol: "BTC".to_string(),
            sentiment: SentimentScore {
                positive: dec!(0.85),
                negative: dec!(0.05),
                neutral: dec!(0.10),
            },
            confidence: dec!(0.85),
        };
        
        let formatted = format_signal(&signal, &SignalType::Buy);
        assert!(formatted.contains("BUY"));
        assert!(formatted.contains("BTC"));
        assert!(formatted.contains("85"));
    }
    
    #[test]
    fn test_format_article() {
        let article = Article {
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
            source: "TestSource".to_string(),
            timestamp: 123456,
        };
        
        let formatted = format_article(&article);
        assert!(formatted.contains("Test Title"));
        assert!(formatted.contains("TestSource"));
    }
    
    #[test]
    fn test_create_dashboard() {
        let articles = vec![
            Article {
                title: "Test".to_string(),
                content: "Content".to_string(),
                source: "Source".to_string(),
                timestamp: 123,
            },
        ];
        
        let sentiments = vec![
            SentimentScore {
                positive: dec!(0.8),
                negative: dec!(0.1),
                neutral: dec!(0.1),
            },
        ];
        
        let signals = vec![
            (Signal {
                symbol: "BTC".to_string(),
                sentiment: sentiments[0].clone(),
                confidence: dec!(0.85),
            }, SignalType::Buy),
        ];
        
        let dashboard = create_dashboard(&articles, &sentiments, &signals);
        assert!(dashboard.contains("Dashboard"));
        assert!(dashboard.contains("Articles Analyzed: 1"));
    }
    
    #[test]
    fn test_progress_bar() {
        let bar = progress_bar(5, 10, 10);
        assert!(bar.contains("50%"));
        
        let bar_full = progress_bar(10, 10, 10);
        assert!(bar_full.contains("100%"));
        
        let bar_empty = progress_bar(0, 10, 10);
        assert!(bar_empty.contains("0%"));
    }
}

