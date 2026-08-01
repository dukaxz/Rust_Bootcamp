// Imultabilidade nas referencias

fn main() {
    // memoria stack
    let mut x = 4;

    imprime_valor(&x);
    imprime_valor(&x);

}

fn imprime_valor(valor : &i32) {
    
    valor += 1; // Nao pode ser executado por nao ter multabilidade nas referencias
    
    println!("Endereço de memoria de x {:p}", valor); // Só printa se nao houver mudanças
}

//========================================================================================

// Reborrowing

fn main() {
    let mut x: i32 = 4;
    println!("Valor original x appós as modificações: {} - referencia: {:p}", x, &x);

    imprime_valor(&mut x);  // passa referencia mutavel para x
    println!("Valor original x appós as modificações: {} - referencia: {:p}", x, &x);

    imprime_valor(&mut x);  // passa referencia mutavel para x
    println!("Valor original x appós as modificações: {} - referencia: {:p}", x, &x);
}

fn imprime_valor(valor: &mut i32) {
    *valor += 1;    // modifica o valor usando reborrowing
    // o compilador move a variavel temporariamente para um lugar na memoria diferente executando e atualizando o valor de _ temporariamente
    // ultilizado para anti aliasing e garantir segurança
    println!("Valor [reborrowing] referenciado por valor: {} _ referencia: {:p}", valor, &valor)
}