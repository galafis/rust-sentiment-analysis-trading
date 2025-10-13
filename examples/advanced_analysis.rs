use anyhow::Result;
use sentiment_analysis_trading::*;
use sentiment_analysis_trading::scrapers::MockDataProvider;
use sentiment_analysis_trading::dashboard;
use sentiment_analysis_trading::correlation::calculate_price_change;
use rust_decimal_macros::dec;

fn main() -> Result<()> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║   Advanced Sentiment Analysis & Signal Generation Demo    ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // Part 1: Demonstrate NLP capabilities
    demo_nlp_analysis()?;
    
    // Part 2: Demonstrate signal generation
    demo_signal_generation()?;
    
    // Part 3: Demonstrate correlation analysis
    demo_correlation_analysis()?;
    
    // Part 4: Demonstrate dashboard
    demo_dashboard()?;
    
    println!("\n✅ All demonstrations completed successfully!\n");
    
    Ok(())
}

fn demo_nlp_analysis() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("📊 Part 1: NLP Sentiment Analysis");
    println!("═══════════════════════════════════════════════════════════\n");
    
    let test_articles = vec![
        ("Bitcoin surges to record high with strong gains", "Very Positive"),
        ("Market crashes amid fears and concerns", "Very Negative"),
        ("Trading volume remains steady today", "Neutral"),
        ("Ethereum shows excellent performance and growth", "Positive"),
    ];
    
    for (i, (text, expected)) in test_articles.iter().enumerate() {
        println!("Test #{}: {}", i + 1, expected);
        println!("Text: \"{}\"", text);
        
        let article = Article {
            title: text.to_string(),
            content: String::new(),
            source: "Test".to_string(),
            timestamp: 0,
        };
        
        let sentiment = analyze_sentiment(&article)?;
        println!("{}", dashboard::format_sentiment(&sentiment));
        
        // Extract entities
        let entities = extract_entities(text);
        if !entities.is_empty() {
            println!("🏷️  Entities: {}", entities.join(", "));
        }
        println!();
    }
    
    Ok(())
}

fn demo_signal_generation() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("🎯 Part 2: Trading Signal Generation");
    println!("═══════════════════════════════════════════════════════════\n");
    
    let test_cases = vec![
        (dec!(0.90), dec!(0.05), dec!(0.05), "BTC", "Strong Buy"),
        (dec!(0.05), dec!(0.90), dec!(0.05), "ETH", "Strong Sell"),
        (dec!(0.35), dec!(0.35), dec!(0.30), "BNB", "Hold"),
        (dec!(0.70), dec!(0.20), dec!(0.10), "ADA", "Moderate Buy"),
    ];
    
    for (pos, neg, neu, symbol, description) in test_cases {
        println!("Test: {}", description);
        
        let sentiment = SentimentScore {
            positive: pos,
            negative: neg,
            neutral: neu,
        };
        
        let (signal, signal_type) = generate_signal_with_type(&sentiment, &symbol)?;
        
        println!("{}", dashboard::format_signal(&signal, &signal_type));
        println!("Signal Strength: {}/100", calculate_signal_strength(&sentiment));
        
        let actionable = is_signal_actionable(&signal, dec!(0.7));
        println!("Actionable (>70% confidence): {}", if actionable { "✅ Yes" } else { "❌ No" });
        println!();
    }
    
    Ok(())
}

fn demo_correlation_analysis() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("📈 Part 3: Price Correlation Analysis");
    println!("═══════════════════════════════════════════════════════════\n");
    
    // Simulate price movements
    let price_scenarios = vec![
        (dec!(50000.0), dec!(55000.0), "Bull Run"),
        (dec!(50000.0), dec!(45000.0), "Bear Market"),
        (dec!(50000.0), dec!(50500.0), "Sideways"),
    ];
    
    for (old_price, new_price, scenario) in price_scenarios {
        println!("Scenario: {}", scenario);
        println!("Old Price: ${}", old_price);
        println!("New Price: ${}", new_price);
        
        let change = calculate_price_change(old_price, new_price);
        let emoji = if change > dec!(0.0) {
            "📈"
        } else if change < dec!(0.0) {
            "📉"
        } else {
            "➡️"
        };
        
        println!("{} Price Change: {:.2}%", emoji, change);
        
        // Predict direction based on sentiment
        let sentiment = if change > dec!(3.0) {
            SentimentScore {
                positive: dec!(0.85),
                negative: dec!(0.05),
                neutral: dec!(0.10),
            }
        } else if change < dec!(-3.0) {
            SentimentScore {
                positive: dec!(0.05),
                negative: dec!(0.85),
                neutral: dec!(0.10),
            }
        } else {
            SentimentScore {
                positive: dec!(0.3),
                negative: dec!(0.3),
                neutral: dec!(0.4),
            }
        };
        
        let direction = predict_price_direction(&sentiment);
        println!("Predicted Direction: {:?}", direction);
        
        let target = calculate_price_target(new_price, &sentiment, dec!(0.05));
        println!("Price Target: ${:.2}", target);
        println!();
    }
    
    Ok(())
}

fn demo_dashboard() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("📊 Part 4: Complete Dashboard Demo");
    println!("═══════════════════════════════════════════════════════════\n");
    
    // Get sample data
    let articles = MockDataProvider::get_sample_articles();
    let mut sentiments = Vec::new();
    let mut signals = Vec::new();
    
    println!("Processing {} articles...\n", articles.len());
    
    for (i, article) in articles.iter().enumerate() {
        print!("{}", dashboard::progress_bar(i + 1, articles.len(), 30));
        println!(" Processing article {} of {}", i + 1, articles.len());
        
        let sentiment = analyze_sentiment(article)?;
        let entities = extract_entities(&article.content);
        let symbol = if !entities.is_empty() {
            entities[0].clone()
        } else {
            "MARKET".to_string()
        };
        
        let (signal, signal_type) = generate_signal_with_type(&sentiment, &symbol)?;
        
        sentiments.push(sentiment);
        signals.push((signal, signal_type));
    }
    
    println!("\n");
    
    // Display comprehensive dashboard
    println!("{}", dashboard::create_dashboard(&articles, &sentiments, &signals));
    
    // Additional analytics
    println!("📊 Detailed Analytics:\n");
    
    let avg_confidence = signals.iter()
        .map(|(s, _)| s.confidence)
        .sum::<rust_decimal::Decimal>() / rust_decimal::Decimal::from(signals.len());
    
    println!("  Average Signal Confidence: {:.1}%", avg_confidence * dec!(100));
    
    let high_confidence_count = signals.iter()
        .filter(|(s, _)| s.confidence > dec!(0.8))
        .count();
    
    println!("  High Confidence Signals (>80%): {}", high_confidence_count);
    
    println!();
    
    Ok(())
}
