// Desestructuring
// struct Pessoa {
//     nome: String,
//     idade: u8,
// }

// fn main() {
//     let pessoa = Pessoa {
//         nome: String::from("Joao"),
//         idade: 19,
//     };

//     // Desestruturação de uma struct
//     let Pessoa { nome, idade } = pessoa;
//     println!("nome: {} , idade: {}", nome, idade);

// }

//
// Para arrays

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];

//     let [primeiro, segundo, ..] = arr;

//     println!("Primeiro: {}, Segundo: {}", primeiro, segundo);
// }

// 
// Para tuplas

// fn main() {
//     let tupla = ("Rust", 2021);

//     let (linguagem, ano) = tupla;

//     println!("Linguagem: {}, Ano: {}", linguagem, ano);        
// }

// 

// Destructing enum

// enum Mensagem {
//     Enviar { id: u32, texto: String },
// }
// fn main() {
//     let msg = Mensagem::Enviar {id: 1, texto: String::from("Ola Rust")};

//     match msg {
//         Mensagem::Enviar { id, texto} => {
//             println!("ID: {}, Texto: {}", id, texto)
//         }
//     }
// }

//

// Permite imprimir a struct de forma legível com {:?} ou {:#?}
#[derive(Debug)]
struct Pessoa {
    nome: String,
    sobrenome: String,
    idade: u8,
}

fn main() {
    // Criando uma instância da struct Pessoa
    let pessoa = Pessoa {
        nome: String::from("Joao"),
        sobrenome: String::from("Teste"),
        idade: 30,
    };

    // Destructuring da struct Pessoa
    // Aqui estamos pegando os campos da struct e criando variáveis com o mesmo nome
    
    // Como usamos &pessoa, não movemos os valores para fora da struct original.
    // Ou seja, nome, sobrenome e idade viram referências aos campos de pessoa.
    let Pessoa { nome, sobrenome, idade } = &pessoa;

    // Criando uma nova Pessoa usando os valores extraídos acima
    let pessoa2 = Pessoa {
        // nome é uma referência para String, então usamos to_owned() para criar uma nova String
        nome: nome.to_owned() + " Silva",

        // copiando o sobrenome para uma nova String
        sobrenome: sobrenome.to_owned(),

        // idade é uma referência para u8, então usamos * para pegar o valor
        idade: *idade,
    };

    // Imprime a nova pessoa formatada
    println!("Pessoa Nova: {:#?}", pessoa2);
}

