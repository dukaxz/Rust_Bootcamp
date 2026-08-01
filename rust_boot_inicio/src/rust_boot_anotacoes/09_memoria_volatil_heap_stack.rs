
// Funcionamento da memoria volatil do computador (Não o disco rigido)


/* Variavel statica: quando usada no codigo é armazenada na memoria statica que permanece 
    fixa ate o final do codigo.  */

// Memoria Stack
/* A Stack é um tipo de memoria que normalmente começa pequena em "MB"" e aumenta conforme o codigo se desenrrola, ocupando e alocando mais espaço, tambem sendo mais rapido.
   Com a maioria dos compiladores trabalham com a memoria Stack armazenando tipos primitivos. */

// Memoria Heap
/* Uma memoria maior inicialmente, armazenando tipos dinamicos que tambem podem crescer durante a aplicação do codigo. Pondo uma referencia na Stack mas o valor continua na Heap.
   Util para dados dinamicos como arrays e str. */

// Statica == Usada no começo da aplicação
static xxx:i32 = 5;
fn main(){

// Stack == Referencia durante aplicação
let x: i32 = 5;

// Heap == Valor durante aplicação
let h: String = String::from("Eduardo...");
}