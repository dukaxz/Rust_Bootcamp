// match in case em rust

fn main () {
    let number: i8 = 3;

    match number {
        1 => println!("O numero é um"),
        2 => println!("O numero é dois"),
        _ => println!("Outro numero"),
    }
}

//====================================================

fn main () {
    loop {
        println!("novamente");
        break;                    // break; para parar
        continue;                 // continue; para avançar a ação do codigo
    
    }
}



fn main () {
    let mut x: i32 = 0;
    while x < 20 {
        
        if (x == 10) || (x == 5) {           // usar || para operaçoes separadas com () ( ou ) 
//        if x == 10 && x == 5 {      usar && para conjuntas por mais que essa nao va funcionar sendo um exemplo de sintaxe  ( e )               
            x += 1;
            continue;
        } // linha para pular um numero ou ação
        println!("{}", x);
        
        // if x > 9 { break; }
        
        x += 1;
    
    }
}

//  for

fn main() {
    for number in 1..5 {                // ou colocando ..= vc chega exatamente da mesma forma mas ja estou acostumado com python
        println!("Numero: {}", number);
    }
}