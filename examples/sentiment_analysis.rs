use anyhow::Result;
use sentiment_analysis_trading::*;
use rust_decimal_macros::dec;

fn main() -> Result<()> {
    println!("=== Sentiment Analysis Trading - Example ===\n");

    // Create sample articles
    let articles = vec![
        Article {
            title: "Bitcoin Surges to New Highs".to_string(),
            content: "Bitcoin reaches unprecedented levels as institutional adoption grows.".to_string(),
            source: "CryptoNews".to_string(),
            timestamp: 1696435200,
        },
        Article {
            title: "Market Correction Expected".to_string(),
            content: "Analysts warn of potential market downturn amid regulatory concerns.".to_string(),
            source: "FinanceTimes".to_string(),
            timestamp: 1696435300,
        },
    ];

    println!("📰 Analyzing {} articles...\n", articles.len());

    for (i, article) in articles.iter().enumerate() {
        println!("Article {}:", i + 1);
        println!("  Title: {}", article.title);
        println!("  Source: {}", article.source);

        // Simulate sentiment analysis
        let sentiment = if article.content.contains("surges") || article.content.contains("grows") {
            SentimentScore {
                positive: dec!(0.85),
                negative: dec!(0.05),
                neutral: dec!(0.10),
            }
        } else {
            SentimentScore {
                positive: dec!(0.15),
                negative: dec!(0.70),
                neutral: dec!(0.15),
            }
        };

        println!("  Sentiment:");
        println!("    Positive: {}", sentiment.positive);
        println!("    Negative: {}", sentiment.negative);
        println!("    Neutral: {}", sentiment.neutral);
        println!();
    }

    println!("=== Analysis Complete ===");
    Ok(())
}
