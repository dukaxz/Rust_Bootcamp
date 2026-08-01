fn main() {
    let s: String = "ola".to_string(); // converte uma &str ("ola") em String
    let s = String::from("ola");       // outra forma de criar uma String
    let s = String::new();             // cria uma String vazia
    println!("{}", s);                 // imprime o valor de s (vazio)
}

//=============================================================================

fn main() {
    let s = "   Hello rust   ".trim();       // remove espaços dos dois lados
    let s = "   Hello rust   ".trim_start(); // remove espaços só do início
    let s = "   Hello rust   ".trim_end();   // remove espaços só do final

    println!("{}", s); // imprime o último valor de s (sem espaço no final)
}

//==============================================================================

fn main() {
    let s = "Hello rust".to_uppercase(); // transforma tudo em MAIÚSCULAS
    println!("{}", s);                   // imprime "HELLO RUST"

    let s = "HELLO RUST".to_lowercase(); // transforma tudo em minúsculas
    println!("{}", s);                   // imprime "hello rust"
}

//==============================================================================


fn main() {
    let original = "Hello world!";
    let replaced = original.replace("world", "rust"); // substitui "world" por "rust"
    println!("{}", replaced);                         // imprime "Hello rust!"

    let original = String::from("Hello, world!");
    let replaced = original.replace("world", "Rust"); // substitui "world" por "Rust"
    println!("{}", replaced);                         // imprime "Hello, Rust!"
}

//==============================================================================


fn main() {
    let s: String = "hello_world".to_camel_case();  // snake_case → camelCase (requer crate)
    println!("{}", s);                               // imprime "helloWorld"

    let s: String = "helloWorld".to_snake_case();   // camelCase → snake_case (requer crate)
    println!("{}", s);                               // imprime "hello_world"

    let s: String = "helloWorld".to_pascal_case();  // camelCase → PascalCase (requer crate)
    println!("{}", s);                               // imprime "HelloWorld"
}

//==============================================================================


fn main() {
    let s = "hello".chars()    // separa a string em caracteres individuais
                   .rev()      // inverte a ordem dos caracteres
                   .collect(); // junta tudo de volta em uma String
    println!("{}", s);         // imprime "olleh"
}

//==============================================================================


fn main() {
    let contains_substring = "hello, world".contains("world");  // verifica se "world" existe → true
    println!("{}", contains_substring);                         // imprime "true"

    let contains_substring = String::from("hello, world").contains("worlds"); // "worlds" não existe → false
    println!("{}", contains_substring);                                        // imprime "false"
}

//==============================================================================


fn main() {
    let texto = "Hello, world, rust, blabla bla";

    let palavras: Vec<&str> = texto.split(" ") // divide a string onde há espaço " "
                                   .collect(); // coleta os pedaços em um Vec
    println!("{:?}", palavras); // imprime ["Hello,", "world,", "rust,", "blabla", "bla"]
}

//==============================================================================


fn main() {
    let s = "hello";
    let substring = &s[0..2]; // pega os caracteres do índice 0 até 1 (não inclui o 2)

    println!("{}", substring); // imprime "he"
}

//==============================================================================


// regex — vale aprofundar o conhecimento nesse tema depois
use regex::Regex;

fn main() {
    let email_regex: Regex = Regex::new(r"^\w+@\w+\.\w+$").unwrap(); // cria o padrão regex para validar email
    let email = "example@example.com";                                 // string a ser testada

    if email_regex.is_match(email) {           // verifica se o email bate com o padrão
        println!("{} é um email válido.", email);   // imprime se for válido
    } else {
        println!("{} é um email invalido.", email); // imprime se for inválido
    }
}