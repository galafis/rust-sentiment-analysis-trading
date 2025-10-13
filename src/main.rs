use anyhow::Result;
use sentiment_analysis_trading::*;
use sentiment_analysis_trading::scrapers::MockDataProvider;
use sentiment_analysis_trading::dashboard;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     Sentiment Analysis Trading System - Demo              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    println!("🔍 Collecting sample articles...\n");
    
    // Get sample articles
    let articles = MockDataProvider::get_sample_articles();
    println!("📰 Found {} articles\n", articles.len());
    
    // Analyze sentiment for each article
    let mut sentiments = Vec::new();
    let mut signals_with_types = Vec::new();
    
    for (i, article) in articles.iter().enumerate() {
        println!("─────────────────────────────────────────────────────────────");
        println!("Article #{}: {}", i + 1, article.title);
        println!("Source: {}", article.source);
        println!();
        
        // Analyze sentiment
        let sentiment = analyze_sentiment(article)?;
        println!("{}", dashboard::format_sentiment(&sentiment));
        
        // Extract entities
        let entities = extract_entities(&article.content);
        if !entities.is_empty() {
            println!("\n🏷️  Detected symbols: {}", entities.join(", "));
        }
        
        // Generate trading signal
        let symbol = if !entities.is_empty() {
            entities[0].clone()
        } else {
            "MARKET".to_string()
        };
        
        let (signal, signal_type) = generate_signal_with_type(&sentiment, &symbol)?;
        println!("\n{}", dashboard::format_signal(&signal, &signal_type));
        println!("Signal Strength: {}/100", calculate_signal_strength(&sentiment));
        
        sentiments.push(sentiment);
        signals_with_types.push((signal, signal_type));
        
        println!();
    }
    
    // Display dashboard summary
    println!("\n");
    println!("{}", dashboard::create_dashboard(&articles, &sentiments, &signals_with_types));
    
    // Show correlation insights
    println!("💡 Insights:");
    let buy_signals = signals_with_types.iter()
        .filter(|(_, t)| *t == SignalType::Buy)
        .count();
    let sell_signals = signals_with_types.iter()
        .filter(|(_, t)| *t == SignalType::Sell)
        .count();
    let hold_signals = signals_with_types.iter()
        .filter(|(_, t)| *t == SignalType::Hold)
        .count();
    
    println!("  🟢 Buy Signals: {}", buy_signals);
    println!("  🔴 Sell Signals: {}", sell_signals);
    println!("  🟡 Hold Signals: {}", hold_signals);
    
    if buy_signals > sell_signals {
        println!("\n  📈 Overall market sentiment: BULLISH");
    } else if sell_signals > buy_signals {
        println!("\n  📉 Overall market sentiment: BEARISH");
    } else {
        println!("\n  ➡️  Overall market sentiment: NEUTRAL");
    }
    
    println!("\n✅ Analysis complete!");
    println!("\nRun 'cargo run --example sentiment_analysis' for more examples");
    
    Ok(())
}

