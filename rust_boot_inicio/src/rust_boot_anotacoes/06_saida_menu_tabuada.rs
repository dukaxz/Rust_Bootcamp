use std::io; // Importa o módulo de entrada e saída (teclado e terminal)

fn main() {
    println!("Digite o valor da tabuada desejada:"); // Solicita o número da tabuada

    let mut valor_tabuada: String = String::new(); // Cria uma String vazia

    io::stdin() // Acessa a entrada padrão (teclado)
        .read_line(&mut valor_tabuada) // Lê o valor digitado
        .expect("Falha ao ler a linha"); // Exibe erro caso a leitura falhe

    let valor_tabuada: i32 = valor_tabuada
        .trim() // Remove espaços e quebras de linha
        .parse() // Converte texto para número
        .expect("Digite um caracter valido"); // Trata erro de conversão

    for multiplicador in 1..=10 { // Loop de 1 até 10
        println!(
            "{} x {} = {}",
            valor_tabuada,                // Número escolhido
            multiplicador,                // Multiplicador atual
            multiplicador * valor_tabuada // Resultado da multiplicação
        );
    }
}

// =================================================================================

use std::io; // Importa o módulo de entrada e saída

fn main() {
    loop { // Loop infinito (encerra com break)

        println!("Escolha uma opção:");

        println!(
            r#"
Opção 1
Opção 2
Opção 3
Opção 4
"#
        ); // Exibe o menu (r#" "# permite múltiplas linhas)

        let mut opcao: String = String::new(); // Armazena a opção digitada

        io::stdin()
            .read_line(&mut opcao) // Lê a entrada do usuário
            .expect("Falha ao ler a linha"); // Trata erro de leitura

        let opcao: i8 = opcao
            .trim() // Remove espaços e quebras de linha
            .parse() // Converte para número inteiro (i8)
            .expect("Digite um caracter valido"); // Trata erro de conversão

        match opcao {
            1 => println!("Opção 1 escolhida."), // Caso 1
            2 => println!("Opção 2 escolhida."), // Caso 2
            3 => println!("Opção 3 escolhida."), // Caso 3
            4 => break, // Encerra o loop e o programa
            _ => println!("Opção invalida."), // Qualquer outro valor
        }
    }
}