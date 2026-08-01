use validador as vd;
use std::io;


fn main() {
    println!("Digite seu CPF: ");
    let mut cpf = String::new();
    

    match io::stdin().read_line(&mut cpf) {
        Ok(_) => {
            println!("Voce digitou: {}", cpf.trim());
        },
        Err(e) => {
            println!("Erro ao ler: {}", e);
        }
    }
    let validado: bool = vd::validar_cpf(cpf.as_str());

    if validado {
        println!("Cpf valido")
    } else {
        println!("Cpf inválido")
    }
}
