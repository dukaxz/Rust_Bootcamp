// ==========================================
// EXEMPLO 1: Operacoes basicas de HashMap
// ==========================================

// use std::collections::HashMap;

// fn main() {
//     // Cria um HashMap vazio: chave &str, valor i32
//     let mut dados: HashMap<&str, i32> = HashMap::new();

//     // insert() adiciona um par chave-valor
//     // (aqui ha uma inconsistencia: o tipo declarado eh &str mas estamos passando String -
//     // isso so funciona se o tipo for HashMap<String, i32>)
//     dados.insert(String::from("Largura"), 10);
//     dados.insert(String::from("Altura"), 50);

//     // get() busca um valor pela chave e retorna um Option<&V>
//     // Option porque a chave pode nao existir (Some) ou nao (None)
//     let valor = dados.get(&String::from("Largura"));
//     let valor: Option<&i32> = dados.get("Largura");

//     // if let eh uma forma curta de tratar o Option sem usar match completo
//     if let Some(valor) = valor {
//         println!("{}", valor)
//     } else {
//         println!("Chave nao encontrada");
//     }

//     // Iterar sobre o HashMap com for retorna pares (chave, valor)
//     // OBS: a ordem NAO eh garantida, pode sair em qualquer sequencia
//     for (key, value) in &dados {
//         println!("{}: {}", key, value);
//     }
// }

// ==========================================
// EXEMPLO 2: entry() e or_insert() - valor padrao
// ==========================================

// use std::collections::HashMap;

// fn main() {
//     let mut dados = HashMap::new();

//     dados.insert(String::from("Largura"), 10);
//     dados.insert(String::from("Altura"), 50);

//     // entry() acessa uma chave; or_insert() insere um valor padrao
//     // SOMENTE se a chave ainda nao existir no HashMap
//     // (aqui tem um typo: o certo eh "or_insert", nao "orinsert")
//     dados.entry("Media").or_insert(30);

//     // Se a chave "Media" ja existir (por causa da linha acima), or_insert
//     // nao faz nada e so retorna uma referencia mutavel ao valor existente
//     let media = dados.entry("Media").or_insert(25);

//     // (aqui tambem tem um typo: o certo eh "{}", nao "{]")
//     println!("{}", media);

//     // {:?} formata o HashMap inteiro para debug/impressao
//     println!("{:?}", dados);
// }

// ==========================================
// EXEMPLO 3: Convertendo uma struct em HashMap
// ==========================================

use std::collections::HashMap;

struct Pessoa {
    nome: String,
    idade: u32,
    cidade: String,
}

fn main() {
    let pessoa = Pessoa {
        nome: "Joao".to_string(),
        idade: 19,
        cidade: "Diadema".to_string(),
    };

    // Monta um array de tuplas (chave, valor) e converte pra HashMap
    // Como o HashMap eh <String, String>, idade precisa virar String tambem
    let mut propriedades_pessoa: HashMap<String, String> = [
        ("nome".to_string(), pessoa.nome),
        ("idade".to_string(), pessoa.idade.to_string()), // u32 -> String
        ("cidade".to_string(), pessoa.cidade),
    ]
    .iter()      // gera um iterador sobre os pares (referencias)
    .cloned()    // clona cada par pra virar valores proprios (owned)
    .collect();  // "junta" o iterador num HashMap

    // insert() adiciona uma nova chave-valor
    // (o metodo certo eh insert, nao iter - iter so serve pra iterar, nao inserir)
    propriedades_pessoa.insert("Largura".to_string(), "xxx".to_string());

    // Imprime o HashMap inteiro no formato de debug
    println!("{:?}", propriedades_pessoa);
}