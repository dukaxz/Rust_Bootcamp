fn calcular_raiz_quadrada(numero: f64) -> Result <f64, String> {
    if numero >= 0.0 {
        Ok (numero.sqrt()) // retorna raiz do numero
    } else {
        Err(String::from("numero negativo nao possui raiz quadrada real"))
    }
}

fn main() {
    let numero = -4.0;
    match calcular_raiz_quadrada(numero) {
        Ok(raiz) => println!("A raiz de {} é {}", numero, raiz),
        Err(e) => println!("Erro: {}", e),
    }
}