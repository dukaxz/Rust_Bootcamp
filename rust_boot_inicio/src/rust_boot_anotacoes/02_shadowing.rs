// Função principal do programa - ponto de entrada da execução

fn main() {
    // Declara variável imutável 'x' com valor 5
    // Em Rust, variáveis são imutáveis por padrão
    let x = 5;
    // Imprime o valor de x e seu endereço de memória
    // {:p} formata o segundo argumento como ponteiro (endereço)
    println!("O valor de x e sua memoria: {}, {:p}", x, &x);

    // SHADOWING: redeclara 'x' com novo valor (x + 1 = 6)
    // Não é mutação! É uma nova variável que "sobrescreve" o nome 'x'
    // Por isso pode ter um endereço de memória diferente
    let x = x + 1;
    println!("O valor de x e sua memoria: {}, {:p}", x, &x);

    // SHADOWING novamente: redeclara 'x' com novo valor (x * 2 = 12)
    // Cada 'let x' cria uma nova ligação de variável na memória
    let x = x * 2;
    println!("O valor de x e sua memoria: {}, {:p}", x, &x);

    // Imprime o valor final de x, que é 12
    // Este 'x' se refere ao último shadowing (x * 2)
    println!("O valor de x é: {}", x) 

}