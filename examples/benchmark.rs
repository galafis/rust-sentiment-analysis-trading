use sentiment_analysis_trading::*;
use sentiment_analysis_trading::scrapers::MockDataProvider;
use std::time::Instant;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║             Sentiment Analysis - Benchmarks               ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // Benchmark 1: Sentiment Analysis
    benchmark_sentiment_analysis();
    
    // Benchmark 2: Signal Generation
    benchmark_signal_generation();
    
    // Benchmark 3: Entity Extraction
    benchmark_entity_extraction();
    
    // Benchmark 4: Complete Pipeline
    benchmark_complete_pipeline();
    
    println!("\n✅ All benchmarks completed!\n");
}

fn benchmark_sentiment_analysis() {
    println!("───────────────────────────────────────────────────────────");
    println!("📊 Benchmark 1: Sentiment Analysis");
    println!("───────────────────────────────────────────────────────────\n");
    
    let articles = MockDataProvider::get_sample_articles();
    let iterations = 1000;
    
    let start = Instant::now();
    for _ in 0..iterations {
        for article in &articles {
            let _ = analyze_sentiment(article);
        }
    }
    let duration = start.elapsed();
    
    let total_ops = iterations * articles.len();
    let ops_per_sec = total_ops as f64 / duration.as_secs_f64();
    let avg_time_us = duration.as_micros() as f64 / total_ops as f64;
    
    println!("  Total operations: {}", total_ops);
    println!("  Total time: {:.2?}", duration);
    println!("  Operations/sec: {:.2}", ops_per_sec);
    println!("  Avg time per analysis: {:.2} µs", avg_time_us);
    println!();
}

fn benchmark_signal_generation() {
    println!("───────────────────────────────────────────────────────────");
    println!("🎯 Benchmark 2: Signal Generation");
    println!("───────────────────────────────────────────────────────────\n");
    
    use rust_decimal_macros::dec;
    
    let sentiment = SentimentScore {
        positive: dec!(0.75),
        negative: dec!(0.15),
        neutral: dec!(0.10),
    };
    
    let iterations = 10000;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = generate_signal(&sentiment, "BTC");
    }
    let duration = start.elapsed();
    
    let ops_per_sec = iterations as f64 / duration.as_secs_f64();
    let avg_time_us = duration.as_micros() as f64 / iterations as f64;
    
    println!("  Total operations: {}", iterations);
    println!("  Total time: {:.2?}", duration);
    println!("  Operations/sec: {:.2}", ops_per_sec);
    println!("  Avg time per signal: {:.2} µs", avg_time_us);
    println!();
}

fn benchmark_entity_extraction() {
    println!("───────────────────────────────────────────────────────────");
    println!("🏷️  Benchmark 3: Entity Extraction");
    println!("───────────────────────────────────────────────────────────\n");
    
    let text = "Bitcoin and Ethereum dominate the market while BTC and ETH lead trading volume";
    let iterations = 10000;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = extract_entities(text);
    }
    let duration = start.elapsed();
    
    let ops_per_sec = iterations as f64 / duration.as_secs_f64();
    let avg_time_us = duration.as_micros() as f64 / iterations as f64;
    
    println!("  Total operations: {}", iterations);
    println!("  Total time: {:.2?}", duration);
    println!("  Operations/sec: {:.2}", ops_per_sec);
    println!("  Avg time per extraction: {:.2} µs", avg_time_us);
    println!();
}

fn benchmark_complete_pipeline() {
    println!("───────────────────────────────────────────────────────────");
    println!("⚡ Benchmark 4: Complete Analysis Pipeline");
    println!("───────────────────────────────────────────────────────────\n");
    
    let articles = MockDataProvider::get_sample_articles();
    let iterations = 100;
    
    let start = Instant::now();
    for _ in 0..iterations {
        for article in &articles {
            // Full pipeline: analyze -> extract -> generate signal
            if let Ok(sentiment) = analyze_sentiment(article) {
                let entities = extract_entities(&article.content);
                let symbol = if !entities.is_empty() {
                    &entities[0]
                } else {
                    "MARKET"
                };
                let _ = generate_signal(&sentiment, symbol);
            }
        }
    }
    let duration = start.elapsed();
    
    let total_ops = iterations * articles.len();
    let ops_per_sec = total_ops as f64 / duration.as_secs_f64();
    let avg_time_us = duration.as_micros() as f64 / total_ops as f64;
    
    println!("  Total operations: {}", total_ops);
    println!("  Total time: {:.2?}", duration);
    println!("  Operations/sec: {:.2}", ops_per_sec);
    println!("  Avg time per pipeline: {:.2} µs", avg_time_us);
    println!();
    
    println!("💡 Performance Summary:");
    println!("  The complete pipeline (analyze + extract + signal) can process");
    println!("  approximately {:.0} articles per second", ops_per_sec);
    println!();
}
