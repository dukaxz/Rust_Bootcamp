
fn main() {
    // Criando a tupla
    let tupla: (i32, f64, String) = (500, 6.4, "Rust".to_string());

    // Forma de mudar a tupla
    // let mut tupla: (i32, f64, String) = (500, 6.4, "Rust".to_string());
    // let (600, 9.8, "Rust".to_string()) = tupla;
    
    // Acessando elementos da tupla
    let (x, y, z) = (&tupla.0, &tupla.1, &tupla.2);

    println!("{} = x", x);
    println!("{} = y", y);
    println!("{} = z", z);

    // Acessando o valor pelo lugar ocupado no index
    println!("primeiro index {} = x", tupla.0);
    println!("segundo index {} = y", tupla.1);
    println!("terceiro index{} = z", tupla.2);
}


// Funçao que retorna uma tupla
fn calcular_dimensoes() -> (i32, i32) {
    // suponha que os valores foram calculados
    let largura = 30;
    let altura = 50;
    (largura, altura) // retornando tupla
}
 
fn main() {
    let dimensoes = calcular_dimensoes();
    println!("largura: {}, altura: {}", dimensoes.0, dimensoes.1);
}


// Funçao com tupla e argumentos
fn soma_dimensoes(dimensao: (i32, i32)) -> i32 {
    dimensao.0 + dimensao.1
}

fn main() {
    let dimensao = (5, 10);
    let soma = soma_dimensoes(dimensao);
    println!("Soma das dimensoes {}", soma);
}