// io::Error com ?

use std::fs::File;
use std::io;
use std::io::Read;

fn ler_arquivo(nome: &str) -> Result<String, io::Error> {
    let mut f = File::open(nome)?;  // Dispara o erro ao inves de tratar com match
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    match ler_arquivo("meu_arquivo.txt") {
        Ok(conteudo) => println!("Conteúdo do arquivo: {}", conteudo),
        Err(e) => println!("Falha ao abrir arquivo: {}", e),
    }
}

// io::Error sem ?

use std::fs::File;
use std::io;
use std::io::Read;

fn ler_arquivo(nome: &str) -> Result<String, io::Error> {
    let mut f = match File::open(nome) {
        Ok(file) => file,
        Err(e) => println!("Erro: {}", e.raw_os_error()),
    }
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    match ler_arquivo("meu_arquivo.txt") {
        Ok(conteudo) => println!("Conteúdo do arquivo: {}", conteudo),
        Err(e) => println!("Falha ao abrir arquivo: {}", e),
    }
}