# 🤝 Contribuindo para Sentiment Analysis Trading

Obrigado pelo seu interesse em contribuir! Este documento fornece diretrizes para contribuir com o projeto.

## 📋 Sumário

- [Código de Conduta](#código-de-conduta)
- [Como Contribuir](#como-contribuir)
- [Configuração do Ambiente](#configuração-do-ambiente)
- [Padrões de Código](#padrões-de-código)
- [Testes](#testes)
- [Processo de Pull Request](#processo-de-pull-request)
- [Reportando Bugs](#reportando-bugs)
- [Sugerindo Melhorias](#sugerindo-melhorias)

---

## 📜 Código de Conduta

Este projeto adere a um código de conduta que todos os contribuidores devem seguir:

- Seja respeitoso e inclusivo
- Aceite críticas construtivas
- Foque no que é melhor para a comunidade
- Demonstre empatia com outros membros da comunidade

## 🚀 Como Contribuir

### Tipos de Contribuições

Aceitamos vários tipos de contribuições:

1. **Correções de Bugs** 🐛
2. **Novas Funcionalidades** ✨
3. **Melhorias de Documentação** 📝
4. **Otimizações de Performance** ⚡
5. **Testes Adicionais** 🧪
6. **Melhorias de UI/UX** 🎨

## 🛠️ Configuração do Ambiente

### Pré-requisitos

- Rust 1.70 ou superior
- Git
- Editor de código (recomendado: VS Code com rust-analyzer)

### Setup Inicial

```bash
# 1. Fork o repositório no GitHub

# 2. Clone seu fork
git clone https://github.com/SEU_USUARIO/rust-sentiment-analysis-trading.git
cd rust-sentiment-analysis-trading

# 3. Adicione o repositório original como upstream
git remote add upstream https://github.com/galafis/rust-sentiment-analysis-trading.git

# 4. Compile o projeto
cargo build

# 5. Execute os testes
cargo test

# 6. Execute o projeto
cargo run --release
```

## 💻 Padrões de Código

### Estilo de Código

- Seguimos as convenções padrão do Rust
- Use `cargo fmt` antes de commitar
- Use `cargo clippy` para análise estática

```bash
# Formate o código
cargo fmt

# Execute o linter
cargo clippy -- -D warnings
```

### Nomenclatura

- **Variáveis e Funções**: `snake_case`
- **Tipos e Structs**: `PascalCase`
- **Constantes**: `SCREAMING_SNAKE_CASE`
- **Módulos**: `snake_case`

### Comentários e Documentação

```rust
/// Analisa o sentimento de um artigo.
///
/// # Arguments
///
/// * `article` - O artigo a ser analisado
///
/// # Returns
///
/// Um `SentimentScore` com os scores de sentimento
///
/// # Examples
///
/// ```
/// let article = Article { /* ... */ };
/// let sentiment = analyze_sentiment(&article)?;
/// ```
pub fn analyze_sentiment(article: &Article) -> Result<SentimentScore> {
    // Implementação
}
```

## 🧪 Testes

### Executando Testes

```bash
# Todos os testes
cargo test

# Testes específicos de um módulo
cargo test nlp::

# Testes com output verbose
cargo test -- --nocapture

# Testes de documentação
cargo test --doc
```

### Escrevendo Testes

Sempre adicione testes para novas funcionalidades:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nova_funcionalidade() {
        // Arrange
        let input = criar_input_teste();
        
        // Act
        let resultado = funcao_testada(input);
        
        // Assert
        assert_eq!(resultado, valor_esperado);
    }
}
```

### Cobertura de Testes

- Mantenha a cobertura de testes acima de 80%
- Teste casos de sucesso e falha
- Teste edge cases

## 🔄 Processo de Pull Request

### Antes de Abrir um PR

1. Certifique-se de que está na branch correta
2. Atualize sua branch com a main

```bash
git checkout main
git pull upstream main
git checkout sua-branch
git rebase main
```

3. Execute todos os testes
4. Execute o formatador e linter
5. Atualize a documentação se necessário

### Criando o PR

1. **Título Claro**: Use um título descritivo
   - ✅ "Adiciona análise de sentimento para tweets"
   - ❌ "Update code"

2. **Descrição Detalhada**:
   - O que foi mudado?
   - Por que foi mudado?
   - Como testar?
   - Screenshots (se aplicável)

3. **Template do PR**:

```markdown
## 📝 Descrição

Breve descrição das mudanças

## 🎯 Motivação e Contexto

Por que essa mudança é necessária? Que problema resolve?

## 🧪 Como foi testado?

Descreva os testes que você executou

## 📸 Screenshots (se aplicável)

Adicione screenshots se houver mudanças visuais

## ✅ Checklist

- [ ] Código segue os padrões do projeto
- [ ] Adicionei testes que provam que a correção/funcionalidade funciona
- [ ] Todos os testes passam localmente
- [ ] Atualizei a documentação
- [ ] Executei cargo fmt
- [ ] Executei cargo clippy
```

### Revisão de Código

- Seja paciente durante a revisão
- Responda aos comentários educadamente
- Faça as alterações solicitadas
- Marque comentários como resolvidos quando apropriado

## 🐛 Reportando Bugs

### Antes de Reportar

1. Verifique se o bug já foi reportado
2. Certifique-se de estar usando a versão mais recente
3. Tente reproduzir o bug de forma consistente

### Template de Bug Report

```markdown
**Descrição do Bug**
Descrição clara e concisa do bug

**Para Reproduzir**
Passos para reproduzir o comportamento:
1. Execute '...'
2. Com os parâmetros '...'
3. Veja o erro

**Comportamento Esperado**
Descrição do que deveria acontecer

**Comportamento Atual**
Descrição do que realmente acontece

**Screenshots**
Se aplicável, adicione screenshots

**Ambiente:**
 - OS: [ex: Ubuntu 22.04]
 - Versão do Rust: [ex: 1.70.0]
 - Versão do Projeto: [ex: 0.1.0]

**Contexto Adicional**
Qualquer informação adicional sobre o problema
```

## 💡 Sugerindo Melhorias

### Template de Feature Request

```markdown
**A feature está relacionada a um problema?**
Descrição clara do problema. Ex: "Sempre fico frustrado quando [...]"

**Descreva a solução que você gostaria**
Descrição clara e concisa do que você quer que aconteça

**Descreva alternativas consideradas**
Descrição de soluções alternativas que você considerou

**Contexto Adicional**
Qualquer outro contexto ou screenshots sobre a feature
```

## 📚 Recursos Adicionais

- [Documentação do Rust](https://doc.rust-lang.org/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rust Book](https://doc.rust-lang.org/book/)

## 🎉 Reconhecimento

Todos os contribuidores serão reconhecidos no README.md do projeto!

---

## 📞 Contato

Se tiver dúvidas sobre como contribuir, abra uma issue ou entre em contato:

- 💻 GitHub: [@galafis](https://github.com/galafis)
- 🔗 LinkedIn: [gabriel-demetrius](https://www.linkedin.com/in/gabriel-demetrius/)

---

<div align="center">

Made with Rust 🦀

</div>
