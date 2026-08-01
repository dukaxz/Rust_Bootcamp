fn encontrar_divisor(numero: u32) -> Option<u32> {
    let mut divisor = 2; // Começa testando pelo menor divisor possível.

    while divisor < numero {
        // Verifica se o número é divisível pelo divisor atual.
        if numero % divisor == 0 {
            return Some(divisor); // Encontrou um divisor.
        } else {
            divisor += 1; // Testa o próximo divisor.
        }
    }

    None // Nenhum divisor encontrado: o número é primo.
}

fn main() {
    let numero = 28;

    let resultado = encontrar_divisor(numero);

    // Verifica se a função retornou um divisor.
    if resultado.is_some() {
        println!("Divisor encontrado: {}", resultado.unwrap());
    } else {
        println!("{} é um número primo.", numero);
    }
}