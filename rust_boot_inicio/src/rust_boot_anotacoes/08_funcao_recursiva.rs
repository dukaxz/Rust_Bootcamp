// Exemplo de função recursiva

fn main() {
    let r: i32 = mostra_na_tela(1); // Inicia a recursão com o valor 1

    println!("O valor somado é: {}", r);
}

fn mostra_na_tela(i: i32) -> i32 {
    if i > 10 {
        return i; // Caso base: encerra a recursão
    }

    println!("O valor de i: {}", i);

    mostra_na_tela(i + 1) // Chama a si mesma com o próximo valor
}


