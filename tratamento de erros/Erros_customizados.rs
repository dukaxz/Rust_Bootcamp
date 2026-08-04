// Exemplo didático de erros customizados em Rust.
// Este arquivo mostra como definir um tipo de erro próprio (`NomeError`),
// implementar `Display` para mensagens amigáveis e usar esse erro em uma
// função de validação (`validar_nome`). O objetivo é demonstrar tratamento
// de erros explícito ao validar regras simples de entrada de texto.

// Importes necessários para implementar o trait `Error` e formatação.
use std::error::Error;
use std::fmt;

// Tipo de erro customizado usado na validação de nomes.
// - `Vazio`: quando a string está vazia após `trim()`.
// - `CaracterInvalido(c)`: quando aparece um caractere que não é letra ASCII.
#[derive(Debug, Clone, PartialEq)]
enum NomeError {
    Vazio,
    CaracterInvalido(char),
}

// Implementação de `Display` para apresentar mensagens legíveis ao usuário.
// Esta implementação traduz os variants do `NomeError` em frases em Português.
impl fmt::Display for NomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NomeError::Vazio => write!(f, "Erro: o nome não pode estar vazio."),
            NomeError::CaracterInvalido(c) => {
                write!(f, "Erro: caractere inválido no nome: '{}'. Use apenas letras.", c)
            }
        }
    }
}

// Marca `NomeError` como um erro compatível com a trait padrão `std::error::Error`.
impl Error for NomeError {}

/// Valida um nome de acordo com regras simples.
///
/// Contexto: função reutilizável que pode ser chamada por entrada do usuário,
/// parsing de arquivos ou APIs. Retorna `Ok(String)` com o nome "limpo" quando
/// válido, ou `Err(NomeError)` indicando o motivo da falha.
///
/// Regras aplicadas:
/// - o nome não pode estar vazio (após `trim()`);
/// - cada caractere deve ser uma letra ASCII (A-Z, a-z).
fn validar_nome(nome: &str) -> Result<String, NomeError> {
    // Remove espaços no início/fim antes de validar.
    let nome = nome.trim();

    // Falha se, após o trim, a string estiver vazia.
    if nome.is_empty() {
        return Err(NomeError::Vazio);
    }

    // Verifica cada caractere; aceita somente letras ASCII.
    for caractere in nome.chars() {
        if !caractere.is_ascii_alphabetic() {
            return Err(NomeError::CaracterInvalido(caractere));
        }
    }

    // Se passou em todas as checagens, retorna o nome válido como `String`.
    Ok(nome.to_string())
}

// `main` contém um pequeno conjunto de exemplos para demonstrar o comportamento
// da função `validar_nome`. Aqui não há interface de usuário — apenas um loop
// que imprime os resultados para fins didáticos.
fn main() {
    // Exemplos variados: nomes válidos, strings vazias e entradas com erros.
    let exemplos = [
        "Ana",
        "Bruno",
        "",
        "   ",
        "Joao123",
        "nome-completo",
        "Carlos",
    ];

    println!("Exemplo de validação de nomes customizados:\n");

    // Para cada exemplo, chamamos `validar_nome` e exibimos o resultado.
    for exemplo in exemplos {
        match validar_nome(exemplo) {
            Ok(nome_valido) => println!("'{}' -> válido", nome_valido),
            Err(erro) => println!("'{}' -> {}", exemplo, erro),
        }
    }
}


