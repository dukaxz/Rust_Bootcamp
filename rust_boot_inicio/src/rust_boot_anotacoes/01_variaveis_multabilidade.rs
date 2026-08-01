// Print em rust
fn main() {
    println!("Hello, world!");
}

// ====================================================

// Variavel imutavel

fn main() {
    let x = 5;                                // Primeira variavel atribuida
    println!("O valor de x é: {}", x);
    let x = 6;                               // Tentativa falha de mudar a primeira variavel
    println!("O valor de x é: {}", x);
}

fn main() {
    let mut x = 3;                          // Variavel mutavel | i32 bits podendo alternar para i64
    println!("O valor de x é: {}", x);
    x += 6;
    println!("O valor de x é: {}", x);
}

// ===================================================================================================
//  Variavel Constante e statica

const TIPO_DE_DADO: i8 = 2;                                     // Tipo constante | não mutável, definida em tempo de compilação
static mut VARIAVEL_STATICA: i8 = 3;                            // Tipo estática | variável global existente durante toda a execução do programa.
                                                                // É mutável, por isso exige `unsafe` para acesso e alteração
fn main() {
    unsafe {
        VARIAVEL_STATICA = 4;                                   // Alterando o valor da variável estática

        let valor = VARIAVEL_STATICA;                           // Copiando o valor para uma variável local antes de utilizar

        println!("O valor de TIPO_DE_DADO é: {}", TIPO_DE_DADO); // A constante pode ser usada em     qualquer lugar do programa
        println!("O valor de STATICA é: {}", valor);             // Exibindo o valor atual da variável estática
    }

    imprime();                                                  // Chamando outra função
}

fn imprime() {
    unsafe {
        VARIAVEL_STATICA = 4;                                   // Alterando novamente a variável estática

        let valor = VARIAVEL_STATICA;                           // Copiando o valor para uma variável local

        println!("Constante: {}", TIPO_DE_DADO);                // A constante continua acessível fora da main
        println!("O valor de STATICA é: {}", valor);            // Exibindo o valor atual da variável estática
    }
}  