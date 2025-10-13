# 📖 API Documentation

Documentação completa da API do Sentiment Analysis Trading System.

## 📋 Sumário

- [Módulos](#módulos)
- [Tipos de Dados](#tipos-de-dados)
- [Funções Principais](#funções-principais)
- [Exemplos de Uso](#exemplos-de-uso)

---

## 🗂️ Módulos

### `nlp` - Natural Language Processing

Módulo responsável por processamento de linguagem natural e análise de sentimento.

#### Funções

##### `analyze_sentiment`

Analisa o sentimento de um artigo e retorna scores.

```rust
pub fn analyze_sentiment(article: &Article) -> Result<SentimentScore>
```

**Parâmetros:**
- `article: &Article` - O artigo a ser analisado

**Retorna:**
- `Result<SentimentScore>` - Scores de sentimento (positivo, negativo, neutro)

**Exemplo:**
```rust
let article = Article {
    title: "Bitcoin surges to new highs".to_string(),
    content: "Market shows strong gains".to_string(),
    source: "CryptoNews".to_string(),
    timestamp: 1696435200,
};

let sentiment = analyze_sentiment(&article)?;
println!("Positive: {}", sentiment.positive);
```

##### `preprocess_text`

Preprocessa texto removendo caracteres especiais e normalizando.

```rust
pub fn preprocess_text(text: &str) -> String
```

##### `extract_entities`

Extrai menções de entidades (símbolos como BTC, ETH) do texto.

```rust
pub fn extract_entities(text: &str) -> Vec<String>
```

---

### `signals` - Trading Signal Generation

Módulo para geração de sinais de trading baseados em sentimento.

#### Enums

##### `SignalType`

```rust
pub enum SignalType {
    Buy,    // Sinal de compra
    Sell,   // Sinal de venda
    Hold,   // Sinal de manutenção
}
```

#### Funções

##### `generate_signal`

Gera um sinal de trading baseado em scores de sentimento.

```rust
pub fn generate_signal(sentiment: &SentimentScore, symbol: &str) -> Result<Signal>
```

**Regras:**
- **BUY**: `positive > 0.65` AND `positive > negative + 0.3`
- **SELL**: `negative > 0.65` AND `negative > positive + 0.3`
- **HOLD**: outros casos

**Exemplo:**
```rust
let sentiment = SentimentScore {
    positive: dec!(0.85),
    negative: dec!(0.05),
    neutral: dec!(0.10),
};

let signal = generate_signal(&sentiment, "BTC")?;
```

##### `generate_signal_with_type`

Gera um sinal incluindo o tipo explícito.

```rust
pub fn generate_signal_with_type(
    sentiment: &SentimentScore, 
    symbol: &str
) -> Result<(Signal, SignalType)>
```

##### `calculate_signal_strength`

Calcula a força do sinal em escala 0-100.

```rust
pub fn calculate_signal_strength(sentiment: &SentimentScore) -> u8
```

##### `is_signal_actionable`

Verifica se um sinal tem confiança suficiente para ser acionável.

```rust
pub fn is_signal_actionable(signal: &Signal, min_confidence: Decimal) -> bool
```

---

### `scrapers` - Data Collection

Módulo para coleta de dados de múltiplas fontes.

#### Structs

##### `NewsScraper`

Scraper de notícias (implementação mock para demo).

```rust
pub struct NewsScraper {
    source: String,
}
```

**Métodos:**
```rust
// Cria novo scraper
pub fn new(source: &str) -> Self

// Scrape artigos (mock)
pub async fn scrape(&self) -> Result<Vec<Article>>

// Cria artigo de exemplo
pub fn create_sample_article(&self, title: &str, content: &str) -> Article
```

##### `MockDataProvider`

Provedor de dados de exemplo para testes.

```rust
pub struct MockDataProvider;
```

**Métodos:**
```rust
// Retorna artigos de exemplo
pub fn get_sample_articles() -> Vec<Article>

// Retorna artigos positivos
pub fn get_positive_articles() -> Vec<Article>

// Retorna artigos negativos
pub fn get_negative_articles() -> Vec<Article>
```

##### `RateLimiter`

Limitador de taxa para requisições.

```rust
pub struct RateLimiter {
    requests_per_second: u32,
}
```

---

### `correlation` - Price Correlation Analysis

Módulo para análise de correlação entre sentimento e preço.

#### Structs

##### `PricePoint`

Representa um ponto de preço no tempo.

```rust
pub struct PricePoint {
    pub timestamp: i64,
    pub price: Decimal,
    pub volume: Option<Decimal>,
}
```

##### `CorrelationData`

Dados de correlação entre sentimento e preço.

```rust
pub struct CorrelationData {
    pub correlation_coefficient: Decimal,
    pub lag_hours: i32,
    pub sample_size: usize,
}
```

#### Enums

##### `PriceDirection`

```rust
pub enum PriceDirection {
    Up,      // Preço esperado subir
    Down,    // Preço esperado cair
    Neutral, // Sem direção clara
}
```

#### Funções

##### `calculate_price_change`

Calcula a mudança percentual de preço.

```rust
pub fn calculate_price_change(old_price: Decimal, new_price: Decimal) -> Decimal
```

**Exemplo:**
```rust
let change = calculate_price_change(dec!(50000.0), dec!(55000.0));
// Retorna: 10.0 (10% de aumento)
```

##### `predict_price_direction`

Prediz direção do preço baseado em sentimento.

```rust
pub fn predict_price_direction(sentiment: &SentimentScore) -> PriceDirection
```

##### `calculate_price_target`

Calcula preço-alvo baseado em sentimento e volatilidade.

```rust
pub fn calculate_price_target(
    current_price: Decimal,
    sentiment: &SentimentScore,
    volatility: Decimal,
) -> Decimal
```

---

### `dashboard` - Data Visualization

Módulo para visualização e formatação de dados.

#### Funções

##### `format_sentiment`

Formata scores de sentimento para exibição.

```rust
pub fn format_sentiment(sentiment: &SentimentScore) -> String
```

##### `format_signal`

Formata um sinal de trading para exibição.

```rust
pub fn format_signal(signal: &Signal, signal_type: &SignalType) -> String
```

##### `format_article`

Formata um artigo para exibição.

```rust
pub fn format_article(article: &Article) -> String
```

##### `create_dashboard`

Cria um dashboard completo em texto.

```rust
pub fn create_dashboard(
    articles: &[Article],
    sentiments: &[SentimentScore],
    signals: &[(Signal, SignalType)],
) -> String
```

##### `progress_bar`

Cria uma barra de progresso.

```rust
pub fn progress_bar(current: usize, total: usize, width: usize) -> String
```

---

## 📊 Tipos de Dados

### `Article`

Representa um artigo de notícia.

```rust
pub struct Article {
    pub title: String,      // Título do artigo
    pub content: String,    // Conteúdo do artigo
    pub source: String,     // Fonte (ex: "CryptoNews")
    pub timestamp: i64,     // Unix timestamp
}
```

### `SentimentScore`

Scores de sentimento que somam 1.0.

```rust
pub struct SentimentScore {
    pub positive: Decimal,  // Score positivo (0.0 - 1.0)
    pub negative: Decimal,  // Score negativo (0.0 - 1.0)
    pub neutral: Decimal,   // Score neutro (0.0 - 1.0)
}
```

### `Signal`

Sinal de trading gerado.

```rust
pub struct Signal {
    pub symbol: String,              // Símbolo (ex: "BTC")
    pub sentiment: SentimentScore,   // Sentimento associado
    pub confidence: Decimal,         // Confiança (0.0 - 1.0)
}
```

---

## 💡 Exemplos de Uso

### Exemplo Completo: Pipeline de Análise

```rust
use sentiment_analysis_trading::*;
use rust_decimal_macros::dec;

fn main() -> Result<()> {
    // 1. Criar artigo
    let article = Article {
        title: "Bitcoin Surges to New Highs".to_string(),
        content: "Bitcoin reaches unprecedented levels with strong gains".to_string(),
        source: "CryptoNews".to_string(),
        timestamp: 1696435200,
    };
    
    // 2. Analisar sentimento
    let sentiment = analyze_sentiment(&article)?;
    println!("Sentiment: +{:.0}% / -{:.0}%", 
        sentiment.positive * dec!(100),
        sentiment.negative * dec!(100)
    );
    
    // 3. Extrair entidades
    let entities = extract_entities(&article.content);
    println!("Entities: {:?}", entities);
    
    // 4. Gerar sinal
    let symbol = if !entities.is_empty() {
        &entities[0]
    } else {
        "MARKET"
    };
    
    let (signal, signal_type) = generate_signal_with_type(&sentiment, symbol)?;
    
    // 5. Verificar se é acionável
    if is_signal_actionable(&signal, dec!(0.7)) {
        println!("Signal: {:?} for {} (confidence: {:.0}%)",
            signal_type,
            signal.symbol,
            signal.confidence * dec!(100)
        );
    }
    
    Ok(())
}
```

### Exemplo: Análise de Múltiplos Artigos

```rust
use sentiment_analysis_trading::scrapers::MockDataProvider;

fn main() -> Result<()> {
    // Obter artigos de exemplo
    let articles = MockDataProvider::get_sample_articles();
    
    let mut buy_count = 0;
    let mut sell_count = 0;
    
    for article in articles {
        let sentiment = analyze_sentiment(&article)?;
        let (_, signal_type) = generate_signal_with_type(&sentiment, "BTC")?;
        
        match signal_type {
            SignalType::Buy => buy_count += 1,
            SignalType::Sell => sell_count += 1,
            _ => {}
        }
    }
    
    println!("Buy signals: {}", buy_count);
    println!("Sell signals: {}", sell_count);
    
    Ok(())
}
```

### Exemplo: Análise de Correlação

```rust
use sentiment_analysis_trading::correlation::*;
use rust_decimal_macros::dec;

fn main() -> Result<()> {
    let sentiment = SentimentScore {
        positive: dec!(0.85),
        negative: dec!(0.05),
        neutral: dec!(0.10),
    };
    
    // Predizer direção
    let direction = predict_price_direction(&sentiment);
    println!("Expected direction: {:?}", direction);
    
    // Calcular price target
    let current_price = dec!(50000.0);
    let volatility = dec!(0.05); // 5%
    let target = calculate_price_target(current_price, &sentiment, volatility);
    println!("Price target: ${:.2}", target);
    
    // Calcular mudança
    let new_price = dec!(52500.0);
    let change = calculate_price_change(current_price, new_price);
    println!("Price change: {:.2}%", change);
    
    Ok(())
}
```

---

## 🔗 Links Úteis

- [README Principal](../README.md)
- [Guia de Contribuição](../CONTRIBUTING.md)
- [Exemplos](../examples/)
- [Código Fonte](../src/)

---

<div align="center">

**Documentação gerada para v0.1.0**

Made with ❤️ and Rust 🦀

</div>
