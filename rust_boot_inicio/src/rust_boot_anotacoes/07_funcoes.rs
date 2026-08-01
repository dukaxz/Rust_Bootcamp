// ========================================
// EXEMPLO 1 - FUNÇÃO QUE RECEBE PARÂMETROS
// E RETORNA UM VALOR
// ========================================

fn main() {

    // Variáveis criadas dentro da função main
    let n1 = 99;
    let n2 = 99;

    // Chama a função soma()
    // n1 é enviado para x
    // n2 é enviado para y
    // O valor retornado é armazenado em r
    let r = soma(n1, n2);

    // Exibe o resultado retornado pela função
    println!("O valor de resultado: {}", r);
}

// Declaração da função soma
//
// x: i16 -> primeiro parâmetro
// y: i16 -> segundo parâmetro
// -> i16 -> informa que a função retornará um i16
fn soma(x: i16, y: i16) -> i16 {

    // O último valor sem ';' é retornado automaticamente
    x + y

    // Equivalente a:
    // return x + y;
}


// ========================================
// EXEMPLO 2 - FUNÇÃO QUE RETORNA UM VALOR
// E ESCOPO DE VARIÁVEIS
// ========================================

fn main() {

    // Variável criada dentro de main()
    let x: i32 = 5;

    // Chama a função calculo()
    // O valor retornado será armazenado em y
    let y = calculo();

    // Mostra que x continua valendo 5
    println!("Valor de x original: {}", x);

    // Mostra o valor retornado pela função
    println!("Valor de y (ultima mudança de x): {}", y);
}

// Função sem parâmetros
// -> i32 significa que ela retorna um número inteiro
fn calculo() -> i32 {

    // Esta variável x existe SOMENTE dentro da função
    // Ela NÃO é a mesma variável x da main()
    let x: i32 = 3;

    println!("O valor da primeira mudança de x: {}", x);

    // Retorna 4
    x + 1
}




// ========================================
// EXEMPLO 3 - FUNÇÃO SEM RETORNO
// USANDO return PARA SAIR MAIS CEDO
// ========================================

fn main() {

    // Chama a função enviando o valor 10
    retorna_string(10);
}

// Função que recebe um parâmetro do tipo i32
//
// Não possui "-> tipo"
// Portanto NÃO retorna nenhum valor
//
// Retorno implícito: ()
fn retorna_string(param: i32) {

    // Verifica se o valor recebido é igual a 10
    if param == 10 {

        // return sozinho encerra a função imediatamente
        // Tudo que estiver abaixo não será executado
        return;
    }

    // Só será executado se param for diferente de 10
    println!("O valor é 10");
}