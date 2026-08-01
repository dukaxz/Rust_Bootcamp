fn main() {
    // Tamanho fixo, definido em tempo de compilação
    let numeros: [i32; 3] = [10, 20, 30];

    // Acessando por índice
    println!("{}", numeros[0]); // 10

    // Array com valor repetido: [0, 0, 0, 0, 0]
    let zeros = [0; 5];

    // Tamanho do array
    println!("{}", numeros.len()); // 3

    // Iterando
    for n in numeros {
        println!("{}", n);
    }
}

fn main() {
    // Criando um Vec vazio e adicionando elementos
    let mut nomes: Vec<String> = Vec::new();
    nomes.push(String::from("Ana"));
    nomes.push(String::from("Bruno"));

    // Criando com a macro vec!
    let mut idades = vec![25, 30, 22];

    // Acessando elementos
    println!("{}", idades[0]);       // 25 (pode dar panic!)
    println!("{:?}", idades.get(1)); // Some(30) — seguro

    // Remover o último elemento
    idades.pop(); // remove 22

    // Tamanho e iteração
    println!("{}", idades.len()); // 2
    for i in &idades {
        println!("{}", i);
    }

}