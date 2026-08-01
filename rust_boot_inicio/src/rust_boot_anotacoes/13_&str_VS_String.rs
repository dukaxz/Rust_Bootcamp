// String vs &str

// &str (String Slice)
// - Referência para um texto.
// - Não possui ownership.
// - Geralmente aponta para dados já existentes.
// - Mais leve e performático.
// - Ideal quando só precisamos ler o texto.

// String
// - Possui ownership.
// - Armazena os dados na Heap.
// - Pode crescer e ser modificada.
// - Ideal quando precisamos alterar o texto.

// Regra prática:
// Ler texto  -> &str
// Modificar  -> String

fn main() {
    // Exemplo com String
    let s1 = String::from("Ola");   // s1 = String alocada na memoria heap
    let s2 = s1.clone();    // s2 = clone forçado de s1 para ser printado

    println!("String s1: {} - referencia: {:p}", s1, &s1);
    println!("String s2: {} - referencia: {:p}", s2, &s2);

    // Exemplo com &str
    let s3: &str = "Ola";   // s3 = slice &str
    let s4: &str = s3;  // s4 é uma referencia de s3

    println!("String s1: {} - referencia: {:p}", s3, &s3);
    println!("String s2: {} - referencia: {:p}", s4, &s4);

}

// fn main2() {
//     String é uma caixa que realmente possui o texto e pode aumentar ou diminuir seu conteúdo.
//     let mut s1: String = String::from("Ola");
//     s1 += " - teste";
//     println!("s1: {}", s1);
    
//     // &str é como um endereço apontando para um texto já existente.
//     // let s2: &str = "Ola";
//     // s2 += "- teste"; erro

// }

//================================================================================================

// Como mudar uma &str slice

fn main() {
    // Exemplo com String
    let s1 = String::from("Ola");   // s1 = String alocada na memoria heap
    s1 += "- teste"; 
    let s2 = s1.clone();    // s2 = clone forçado de s1 para ser printado

    println!("String s1: {} - referencia: {:p}", s1, &s1);
    println!("String s2: {} - referencia: {:p}", s2, &s2);

    // Exemplo com &str
    let s3: &str = "Ola";   // s3 = slice &str
    let s4: String = format!("{}- teste", s3);  // Cria um novo s3 conectando o primero com o segundo texto

    
    println!("String s3: {} - referencia: {:p}", s3, &s3);
    println!("String s4: {} - referencia: {:p}", s4, &s4);
    
    // kd um tendo uma referencia diferente

}

//==============================================================================================

// Manipulação de Substrings 

fn main() {
    let original_string: String = String::from("Boot_Camp_Rust!");

    // Criando substring usando slice e pegando os primeiros 2 chars
    let substring = &original_string[0..4];

    println!("String original: {} - referencia: {:p}", original_string, &original_string);
    println!("Substring: {} - referencia {:p}", substring, substring);
}

//==============================================================================================

// Convertendo uma String em &str slice

fn main() {
    // Convertendo String em &str usando '.as_str()'
    let s1: String = String::from("Rust_boot!");
    let reference_to_s1: &str = s1.as_str();

    println!("s1 (String): {} - referencia {:p}", s1, &s1);
    println!("s1 (Referencia &str): {} - referencia {:p}", reference_to_s1, reference_to_s1);

    // Convertendo String em &str fazendo uma referencia
    let s2: String = String::from("Rust_boot2!");
    let reference_to_s2: &str = &s2;

    println!("s1 (String): {} - referencia {:p}", s2, &s2);
    println!("s1 (Referencia &str): {} - referencia {:p}", reference_to_s2, reference_to_s2);
}