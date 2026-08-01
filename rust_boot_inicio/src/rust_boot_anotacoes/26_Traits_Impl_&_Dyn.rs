// =====================================================================
// 06_traits_e_polimorfismo.rs
// Estudo: Traits | Outros exemplos de Trait | Polimorfismo (impl vs dyn)
// =====================================================================
//


use std::fmt;

fn main() {
    println!("=== 1. TRAITS: o básico ===");
    secao_traits_basico();

    println!("\n=== 2. OUTROS EXEMPLOS DE TRAIT ===");
    secao_outros_exemplos();

    println!("\n=== 3. POLIMORFISMO: impl Trait vs dyn Trait ===");
    secao_polimorfismo();
}

// ---------------------------------------------------------------------
// 1) TRAITS: o básico
// ---------------------------------------------------------------------
// Trait = "interface" do Rust. Define um CONTRATO: um conjunto de métodos
// que um tipo promete implementar. Não guarda dados, só comportamento.
//
// Diferença chave pra quem vem de outras linguagens: você pode implementar
// um trait em QUALQUER tipo (inclusive tipos que não são seus, respeitando
// a "orphan rule": trait ou tipo precisa ser seu no crate atual).

trait Resumo {
    // Método sem corpo = obrigatório implementar
    fn titulo(&self) -> String;

    // Método com corpo padrão = implementação default, pode ser sobrescrita
    fn resumo(&self) -> String {
        format!("(sem resumo disponível para \"{}\")", self.titulo())
    }
}

struct Artigo {
    titulo: String,
    conteudo: String,
}

struct Tweet {
    usuario: String,
    texto: String,
}

// Implementando o trait pra cada tipo
impl Resumo for Artigo {
    fn titulo(&self) -> String {
        self.titulo.clone()
    }

    fn resumo(&self) -> String {
        // sobrescrevendo o default
        format!("{}: {}...", self.titulo, &self.conteudo[..self.conteudo.len().min(20)])
    }
}

impl Resumo for Tweet {
    fn titulo(&self) -> String {
        format!("@{}", self.usuario)
    }
    // não sobrescreve resumo() -> usa o default do trait
}

fn secao_traits_basico() {
    let artigo = Artigo {
        titulo: String::from("Ownership em Rust"),
        conteudo: String::from("Ownership é o sistema que Rust usa pra gerenciar memória sem GC"),
    };

    let tweet = Tweet {
        usuario: String::from("clc_dev"),
        texto: String::from("estudando traits hoje"),
    };

    println!("{}", artigo.resumo()); // usa a versão sobrescrita
    println!("{}", tweet.resumo());  // usa a versão default do trait
}

// ---------------------------------------------------------------------
// 2) OUTROS EXEMPLOS DE TRAIT
// ---------------------------------------------------------------------
// a) Traits com múltiplos métodos e "trait bounds" (restrições genéricas)
// b) Operator overloading via traits da std (ex: Add)
// c) Traits derivadas automaticamente (derive) vs implementadas na mão

// a) trait com mais de um método + um método que usa outro
trait Figura {
    fn area(&self) -> f64;
    fn perimetro(&self) -> f64;

    fn descricao(&self) -> String {
        format!("área = {:.2}, perímetro = {:.2}", self.area(), self.perimetro())
    }
}

struct Retangulo {
    largura: f64,
    altura: f64,
}

impl Figura for Retangulo {
    fn area(&self) -> f64 {
        self.largura * self.altura
    }
    fn perimetro(&self) -> f64 {
        2.0 * (self.largura + self.altura)
    }
}

struct Circulo {
    raio: f64,
}

impl Figura for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.raio * self.raio
    }
    fn perimetro(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.raio
    }
}

// b) Operator overloading: implementando std::ops::Add pra um tipo próprio
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Ponto {
    x: i32,
    y: i32,
}

impl Add for Ponto {
    type Output = Ponto;
    fn add(self, other: Ponto) -> Ponto {
        Ponto {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// c) Trait derivada (macro derive) vs implementada manualmente
// Debug, Clone, PartialEq, etc. podem ser "derivadas" automaticamente
// quando todos os campos do struct também implementam o trait.
#[derive(Debug, PartialEq)]
struct Peca {
    codigo: String,
    quantidade: u32,
}

// Já Display, por exemplo, NÃO tem derive -- precisa implementar na mão,
// porque o formato de exibição é uma decisão sua, não genérica.
impl fmt::Display for Peca {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Peça {} (qtd: {})", self.codigo, self.quantidade)
    }
}

fn secao_outros_exemplos() {
    let ret = Retangulo { largura: 4.0, altura: 5.0 };
    let circ = Circulo { raio: 3.0 };
    println!("Retângulo: {}", ret.descricao());
    println!("Círculo:   {}", circ.descricao());

    let p1 = Ponto { x: 1, y: 2 };
    let p2 = Ponto { x: 3, y: 4 };
    let p3 = p1 + p2; // funciona por causa do impl Add
    println!("p1 + p2 = {:?}", p3);

    let peca1 = Peca { codigo: "H0000000001".into(), quantidade: 10 };
    let peca2 = Peca { codigo: "H0000000001".into(), quantidade: 10 };
    println!("{}", peca1);                 // usa Display (manual)
    println!("são iguais? {}", peca1 == peca2); // usa PartialEq (derive)
}

// ---------------------------------------------------------------------
// 3) POLIMORFISMO: impl Trait vs dyn Trait
// ---------------------------------------------------------------------
// Rust não tem herança de classes. Polimorfismo é feito via traits,
// de duas formas principais:
//
//  - impl Trait (static dispatch / genéricos): o compilador GERA uma
//    versão especializada da função pra cada tipo concreto usado
//    (monomorphization). Mais rápido em runtime, mas o tipo precisa
//    ser conhecido em tempo de COMPILAÇÃO.
//
//  - dyn Trait (dynamic dispatch / trait objects): o tipo concreto só
//    é resolvido em RUNTIME, via uma vtable (ponteiro pra funções).
//    Mais flexível (ex: Vec<Box<dyn Trait>> com tipos diferentes),
//    mas tem um pequeno custo de indireção.

// --- impl Trait: usado como parâmetro (açúcar sintático pra genérico) ---
fn imprimir_com_impl(item: &impl Figura) {
    // Essa função é monomorphizada: o compilador cria uma cópia
    // especializada pra cada tipo concreto (Retangulo, Circulo, etc.)
    println!("[impl Trait] {}", item.descricao());
}

// equivalente explícito usando genéricos + trait bound:
fn imprimir_generico<T: Figura>(item: &T) {
    println!("[generico<T: Figura>] {}", item.descricao());
}

// --- impl Trait: usado como RETORNO ---
// Útil quando você quer devolver "algo que implementa Figura"
// sem expor o tipo concreto, mas o retorno ainda precisa ser
// de UM ÚNICO tipo concreto por chamada (não pode variar em runtime).
fn criar_figura_fixa() -> impl Figura {
    Retangulo { largura: 2.0, altura: 2.0 }
    // Se você tentasse retornar Retangulo OU Circulo dependendo de uma
    // condição, o compilador reclamaria: tipos de retorno diferentes.
}

// --- dyn Trait: quando o tipo concreto varia em runtime ---
// Aqui SIM podemos misturar tipos diferentes, porque dyn Trait
// vira um "trait object": um ponteiro + vtable, não o tipo em si.
fn imprimir_com_dyn(item: &dyn Figura) {
    println!("[dyn Trait] {}", item.descricao());
}

// Uma coleção heterogênea só é possível com dyn (via Box, pois
// trait objects não têm tamanho conhecido em compile-time: são "?Sized")
fn coletar_figuras() -> Vec<Box<dyn Figura>> {
    vec![
        Box::new(Retangulo { largura: 3.0, altura: 6.0 }),
        Box::new(Circulo { raio: 1.5 }),
        Box::new(Retangulo { largura: 1.0, altura: 1.0 }),
    ]
}

fn secao_polimorfismo() {
    let ret = Retangulo { largura: 4.0, altura: 5.0 };
    let circ = Circulo { raio: 2.0 };

    // impl Trait / genérico: cada chamada é resolvida em compile-time
    imprimir_com_impl(&ret);
    imprimir_generico(&circ);

    let figura_fixa = criar_figura_fixa();
    println!("figura fixa: {}", figura_fixa.descricao());

    // dyn Trait: mesma função aceita tipos concretos diferentes
    imprimir_com_dyn(&ret);
    imprimir_com_dyn(&circ);

    // O caso clássico que só dyn resolve bem: lista heterogênea
    let figuras = coletar_figuras();
    let area_total: f64 = figuras.iter().map(|f| f.area()).sum();
    for f in &figuras {
        println!(" - {}", f.descricao());
    }
    println!("área total da coleção: {:.2}", area_total);

    // Resumo mental:
    // impl Trait / genéricos -> "compilador escolhe e otimiza pra mim,
    //   mas todo mundo na lista tem que ser do mesmo tipo concreto"
    // dyn Trait               -> "decide em runtime, aceita mistura de
    //   tipos, mas paga um preço pequeno de indireção (vtable)"
}