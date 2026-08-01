// Steing memoria heap

fn main() {
    // s1 esta na memoria heap

    // tupla para memoria stack
    // array para memoria heap


    // nao é uma variavel by copy, se nao estoura a memoria heap com uma muito grande
    let s1: String = String::from("Olá"); // Possui a propriedade String
    let s2: String = s1;    // A propriedade é transferida

    // Causando erro por s1 nao ser mais valido após transferencia
    // println!("S1: {} - referencia {:p}", s1, &s1)

    // s2 é valido e pode ser usado
    println!("S2: {} - referencia {:p}", s2, &s2)
    
}

// Proposito do rust é ser rapido e seguro, tendo metodos rigidos para copias de variaveis 

fn main() {
  

    let s1: String = String::from("Olá"); 
    let s2: String = s1.clone();    // Recebe copia de s1 (clone)  

    println!("S1: {} - referencia {:p}", s1, &s1);
    println!("S2: {} - referencia {:p}", s2, &s2)  
     // pode ate ter o mesmo resultado mas o endereço de memoria sera diferente
    
}

// Muitos clones deixam o processamento devagar 