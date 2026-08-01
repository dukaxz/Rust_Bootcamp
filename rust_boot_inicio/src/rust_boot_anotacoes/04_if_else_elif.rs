fn main() {
    let numero = 3;

    if numero != 0 {
        println!("número era algo diferente de zero");
        return;             
    }
    println!("numero")
}
/* Tendo o return nao ira aparecer o print fora da 
    indentação. */

// ==================================================================

// verifica se o numero e divisivel por 4, 3 e 2

fn main() {
    let numero: i8 = 6;                         

    if numero % 4 == 0 {
        println!("número é divisível por 4");
    } 
    else if numero % 3 == 0 {
        println!("número é divisível por 3");
    } 
    else if numero % 2 == 0 {
        println!("número é divisível por 2");
    } 
    else {
        println!("número não é divisível por 4, 3 ou 2");
    }
}

// ===================================================================

/* Codigo exemplo para modificação em blocos representados pelas chaves
   é necessario seguir a identação nos dois casos */

fn main () {
    let numero: i32 = 3;

    let resultado: i32 = if numero > 3 { 4 } else { 5 };
    println!("o resultado é {}", resultado)
}
//
fn main () {
    let numero: i32 = 3;

    let resultado: i32 = if numero > 3 {
        4
    } else {
        5
    };
    println!("o resultado é {}", resultado)
}

// =======================================================

// Caulculador de idade sem input

fn main() {
    let nome: &str = "Eduardo";
    let ano_nascimento: u16 = 2007;
    let ano_atual: u16 = 2026;
    
    let mes_nascimento: u16 = 12;                           
    let mes_atual: u16 = 6;

    let dia_atual: u16 = 2;
    let dia_nascimento: u16 = 27;
    
    let mut idade: u16 = ano_atual - ano_nascimento;
    if mes_nascimento > mes_atual {
        idade -= 1;
    } else if dia_nascimento -= dia_atual {
        idade -= 1;
    }
    println!("A idade do {} é {} anos", nome, idade);
}