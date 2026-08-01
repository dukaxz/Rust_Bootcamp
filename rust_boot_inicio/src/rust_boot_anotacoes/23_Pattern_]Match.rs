// Exemplo de Pattern Matching (Combinação de padroes)

// enum Comando {
//     Iniciar(String), // variante com dado associado (String)
//     Parar,           // variante sem dado
// }

// fn executar_comando(comando: Comando) {
//     match comando {
//         Comando::Iniciar(mensagem) => println!("Iniciando: {}", mensagem), // extrai o valor de dentro da variante
//         Comando::Parar => println!("Parando"), // variante simples, sem extração
//     }
// }

// fn main() {
//     let comando_iniciar = Comando::Iniciar(String::from("Motor"));
//     let comando_parar = Comando::Parar;

//     executar_comando(comando_iniciar);
//     executar_comando(comando_parar);
// }

//================================================
// enum Tipo {
//     Juridica(String), // guarda uma descrição (ex: "matriz")
//     Fisica(String),   // guarda uma descrição (ex: "funcionario")
// }

// struct Pessoa {
//     nome: String,
//     documento: String,
//     tipo: Tipo, // campo que usa o enum acima
// }

// fn main() {
//     let daniel = Pessoa {
//         nome: String::from("Daniel"),
//         documento: String::from("333-222-222-32"),
//         tipo: Tipo::Fisica(String::from("funcionario")),
//     };

//     match daniel.tipo {
//         // "ref" evita mover o valor de dentro do enum, só pega uma referência
//         Tipo::Fisica(ref descricao) => {
//             println!("{} é uma pessoa fisica({})", daniel.nome, descricao)
//         },
//         Tipo::Juridica(ref descricao) => {
//             println!("{} é uma pessoa juridica, {}", daniel.nome, descricao)
//         },
//     }
    
//================================================

enum Tipo {
    Juridica(String, i16), // agora com dois dados: texto e número inteiro pequeno
    Fisica(i32, f64),      // dois dados: número inteiro e número decimal
}

struct Pessoa {
    nome: String,
    documento: String,
    tipo: Tipo,
}

fn main() {
    let daniel = Pessoa {
        nome: String::from("Daniel"),
        documento: String::from("3332.33322332.232.323"),
        tipo: Tipo::Fisica(1, 6.5), // cria a variante Fisica já com os dois valores
    };

    match daniel.tipo {
        // desestrutura os dois valores da variante em uma única linha
        Tipo::Fisica(ref valor, ref valor_f) => {
            println!("{} é uma pessoa fisica({}), {}", daniel.nome, valor, valor_f)
        },
        Tipo::Juridica(ref valor, ref valor_f) => {
            println!("{} é uma pessoa juridica, {}, {}", daniel.nome, valor, valor_f)
        },
    }
}

// Pattern matching: usar "match" para verificar qual variante de um enum (ou formato de dado) se encaixa, extraindo os valores internos de cada caso.