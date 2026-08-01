// ============================================================
// AULA: GENERICS EM RUST
// Exemplos usando contexto de peças/estoque (tipo HSA)
// Rode com: rustc aula_generics.rs && ./aula_generics
// ou cole num main.rs de um projeto cargo
// ============================================================

use std::fmt::Debug;
use std::collections::HashMap;

fn main() {
    aula_1_funcao_generica();
    aula_2_struct_generica();
    aula_3_multiplos_parametros();
    aula_4_bounds_e_where();
    aula_5_blanket_impl();
    aula_6_generics_vs_trait_objects();
}

// ------------------------------------------------------------
// 1. FUNÇÃO GENÉRICA BÁSICA
// ------------------------------------------------------------
// T é um placeholder de tipo. O compilador gera uma versão
// concreta da função para cada tipo usado (monomorphization).
// Isso significa: ZERO custo em runtime, mas o binário fica maior
// se você usar a função com muitos tipos diferentes.

fn maior<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn aula_1_funcao_generica() {
    println!("--- Aula 1: função genérica ---");

    let qtd_maior = maior(120, 85); // T = i32
    println!("Maior quantidade: {}", qtd_maior);

    let preco_maior = maior(45.90, 39.99); // T = f64
    println!("Maior preço: {}", preco_maior);

    // maior("A", "B") também funciona: T = &str, porque &str
    // implementa PartialOrd. O bound é o que decide o que É
    // permitido entrar na função, não o tipo em si.
}

// ------------------------------------------------------------
// 2. STRUCT GENÉRICA
// ------------------------------------------------------------
// Aqui T não é "qualquer coisa mágica". É um parâmetro fixado
// no momento da instância. Uma Peca<i32> e uma Peca<String> são
// tipos DIFERENTES em tempo de compilação, mesmo vindo do
// mesmo struct genérico.

#[derive(Debug)]
struct Peca<T> {
    codigo: String,
    valor: T, // pode ser quantidade (i32), custo (f64), etc.
}

impl<T: Debug> Peca<T> {
    fn new(codigo: &str, valor: T) -> Self {
        Peca { codigo: codigo.to_string(), valor }
    }

    fn mostrar(&self) {
        println!("Peça {}: {:?}", self.codigo, self.valor);
    }
}

fn aula_2_struct_generica() {
    println!("\n--- Aula 2: struct genérica ---");

    let peca_qtd = Peca::new("PN-1001", 350); // Peca<i32>
    let peca_custo = Peca::new("PN-1001", 12.75); // Peca<f64>

    peca_qtd.mostrar();
    peca_custo.mostrar();

    // ERRO se descomentar: tipos diferentes, mesmo struct genérico
    // let vetor_misto = vec![peca_qtd, peca_custo]; // não compila
}

// ------------------------------------------------------------
// 3. MÚLTIPLOS PARÂMETROS DE TIPO
// ------------------------------------------------------------
// Nada impede usar mais de uma letra. Convenção: T, U, V...
// mas nomes descritivos também são aceitos e às vezes mais claros.

struct MovimentoEstoque<Codigo, Quantidade> {
    peca: Codigo,
    delta: Quantidade,
}

impl<Codigo: Debug, Quantidade: Debug> MovimentoEstoque<Codigo, Quantidade> {
    fn log(&self) {
        println!("Movimento: peça={:?}, delta={:?}", self.peca, self.delta);
    }
}

fn aula_3_multiplos_parametros() {
    println!("\n--- Aula 3: múltiplos parâmetros ---");

    let mov = MovimentoEstoque { peca: "PN-2002", delta: -15 };
    mov.log();
}

// ------------------------------------------------------------
// 4. TRAIT BOUNDS E WHERE CLAUSES
// ------------------------------------------------------------
// Bound = restrição. "T tem que implementar tal trait para
// entrar aqui". Sem isso, o compilador não sabe que operações
// T suporta (nem +, nem >, nem nada).
//
// `where` é a mesma coisa que bound inline, só que mais legível
// quando os bounds começam a empilhar.

// Bound inline: fica ilegível rápido com mais de 1-2 bounds
fn total_generico_inline<T: std::iter::Sum + Copy>(itens: &[T]) -> T {
    itens.iter().copied().sum()
}

// Mesma função, com where: mais fácil de ler quando cresce
fn total_generico<T>(itens: &[T]) -> T
where
    T: std::iter::Sum + Copy,
{
    itens.iter().copied().sum()
}

fn aula_4_bounds_e_where() {
    println!("\n--- Aula 4: bounds e where ---");

    let quantidades = [10, 25, 40, 5];
    let soma: i32 = total_generico(&quantidades);
    println!("Total em estoque: {}", soma);

    let _ = total_generico_inline(&quantidades); // mesma coisa
}

// ------------------------------------------------------------
// 5. BLANKET IMPLEMENTATION
// ------------------------------------------------------------
// Implementar um trait para QUALQUER tipo que satisfaça um bound.
// É como dizer: "todo mundo que sabe fazer X automaticamente
// também sabe fazer Y".
//
// Exemplo clássico da std: impl<T: Display> ToString for T {}
// Aqui: qualquer tipo com Debug ganha de graça um relatório
// formatado, sem precisar implementar nada manualmente.

trait RelatorioEstoque {
    fn relatorio(&self) -> String;
}

impl<T: Debug> RelatorioEstoque for T {
    fn relatorio(&self) -> String {
        format!("[RELATÓRIO] {:?}", self)
    }
}

fn aula_5_blanket_impl() {
    println!("\n--- Aula 5: blanket implementation ---");

    let peca = Peca::new("PN-3003", 99);
    println!("{}", peca.relatorio()); // Peca<i32> não implementou nada disso à mão

    let numero = 42;
    println!("{}", numero.relatorio()); // i32 também ganhou de graça
}

// ------------------------------------------------------------
// 6. GENERICS (MONOMORPHIZATION) vs TRAIT OBJECTS (dyn)
// ------------------------------------------------------------
// Isso conecta com o que você já viu de impl Trait vs dyn Trait.
//
// - Generic <T: Trait>: compilador gera uma cópia especializada
//   da função/struct para cada T usado. Dispatch estático,
//   zero custo em runtime, mas SEM heterogeneidade: um
//   Vec<Peca<T>> só aceita um T por vez.
//
// - dyn Trait: dispatch dinâmico via vtable, permite misturar
//   tipos diferentes na mesma coleção, mas paga o custo de uma
//   indireção (ponteiro) em cada chamada.

trait Movimentavel {
    fn aplicar(&self) -> i32;
}

struct Entrada(i32);
struct Saida(i32);

impl Movimentavel for Entrada {
    fn aplicar(&self) -> i32 { self.0 }
}
impl Movimentavel for Saida {
    fn aplicar(&self) -> i32 { -self.0 }
}

fn aula_6_generics_vs_trait_objects() {
    println!("\n--- Aula 6: generics vs dyn ---");

    // Generic: só funciona com UM tipo concreto por chamada
    fn processa_generico<T: Movimentavel>(item: &T) -> i32 {
        item.aplicar()
    }
    println!("Generic (Entrada): {}", processa_generico(&Entrada(50)));

    // dyn: mistura tipos diferentes na mesma coleção
    let movimentos: Vec<Box<dyn Movimentavel>> = vec![
        Box::new(Entrada(50)),
        Box::new(Saida(20)),
        Box::new(Entrada(10)),
    ];

    let saldo: i32 = movimentos.iter().map(|m| m.aplicar()).sum();
    println!("Saldo via dyn Trait (tipos mistos): {}", saldo);

    // Pergunta pra fixar: por que Vec<Entrada> ou Vec<Saida>
    // separados NÃO precisariam de dyn, mas misturar os dois
    // no mesmo Vec obriga a usar Box<dyn Movimentavel>?
    // Resposta: Vec precisa que todo elemento tenha o MESMO
    // tamanho conhecido em compile time. Entrada e Saida têm
    // tamanhos possivelmente diferentes; Box<dyn ...> normaliza
    // isso pra um ponteiro de tamanho fixo.
}