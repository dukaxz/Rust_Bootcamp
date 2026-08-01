// enum Tipo {
//     Juridica,
//     Fisica,
// }

// struct Pessoa {
//     nome: String,
//     documento: String, 
//     tipo: Tipo, // campo do tipo enum
// }

// fn main() {
//     let Daniel = Pessoa {
//         nome: "Daniel".to_string(),
//         documento: String::from("333-333-333-11"),
//         tipo: Tipo::Fisica, // Enum::Variante
//     };

//     match Daniel.tipo {
//         Tipo::Fisica => { // trata a variante Fisica
//             println!("{} é uma pessoa fisica.", Daniel.nome)
//         },
//         _ => { // pega qualquer outra variante (aqui, Juridica)
//             println!("{} é uma pessoa juridica.", Daniel.nome)
//         },
//     }
// }



#[derive(PartialEq, Debug)]
// PartialEq: permite usar "==" entre variantes
// Debug: permite imprimir com {:?}
enum Tipo {
    Juridica,
    Fisica,
}

struct Pessoa {
    nome: String,
    documento: String,
    tipo: Tipo, // só aceita as variantes definidas no enum
}

fn main() {
    let daniel = Pessoa {
        nome: String::from("Daniel"),
        documento: String::from("222-333-444-11"),
        tipo: Tipo::Fisica, // Enum::Variante
    };

    // comparação possível graças ao PartialEq
    if daniel.tipo == Tipo::Fisica {
        println!("{} é uma pessoa fisica.", daniel.nome)
    } else {
        println!("{} é uma pessoa juridica", daniel.nome)
    };
}