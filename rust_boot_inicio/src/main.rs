use std::error::Error;
use std::fs::File;
use std::io::{self, Read};


fn ler_conteudo_arquivo(nome_arquivo: &str) -> Result<String, io::Error> {
    let mut f = File::open(nome_arquivo)?;  // Dispara o erro ao inves de tratar com match
    let mut conteudo = String::new();
    f.read_to_string(&mut conteudo)?;
    
    Ok(conteudo)
}


#[derive(Debug)]

enum ErroTransacao {
    SaldoInsulficiente {saldo_atual: f64, tentativa_saque: f64 },
    ErroDeAutenticacao,
    ErroDeRede,
    ErroComFonte { mensagem: String, fonte: Box<dyn Error> },
}


impl std::fmt::Display for ErroTransacao {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErroTransacao::SaldoInsulficiente { saldo_atual, tentativa_saque } => write!(f, "Saldo insulficiente: saldo atual R${}, tentativa de saque: R${}", saldo_atual, tentativa_saque),
            
            ErroTransacao::ErroDeAutenticacao => write!(f, "Usuario não autenticado"),
            
            ErroTransacao::ErroDeRede => write!(f, "Falha na conexão com o servidor."),

            ErroTransacao::ErroComFonte { mensagem , fonte } => write!(f, "{}, {}", mensagem, fonte),
        }
    }
}


// Implementação da trait Error
impl std::error::Error for ErroTransacao {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErroTransacao::ErroComFonte { fonte, .. } => Some(fonte.as_ref()),
            _ => None,   
        }
    }
}


fn processar_transacao(valor: f64, autenticado: bool, falha_na_rede: bool, com_fonte: bool) -> Result<(), ErroTransacao> {
    let saldo = 1000.0; // Calculo ficticio
   
    if valor > saldo {
        return Err(ErroTransacao::SaldoInsulficiente { saldo_atual: saldo, tentativa_saque: valor })
    }

    // Simulando erro de autenticação    
    if autenticado {
        return Err(ErroTransacao::ErroDeAutenticacao);
    }

    // Simulando erro de rede
    if falha_na_rede {
        return Err(ErroTransacao::ErroDeRede);
    }

    if com_fonte {
        let result_arquivo: Result<String, io::Error> = ler_conteudo_arquivo("arquivo.txt");
        match result_arquivo {
            Ok(_) => { },
            Err(erro) => {
                return Err(ErroTransacao::ErroComFonte { mensagem: "Erro ao abrir arquivo".to_string(), fonte: Box::new(erro), 
            });
            }
        }

    }

    // Se chegar no ok a transação "foi" bemsucedida
    Ok(())

}


fn main() {
    match processar_transacao(30.0, false, false, false) {
        Ok(_) => println!("Transação processada."),
        Err(e) => {
            println!("Falha ao processar transação: {}", e);

            if let Some(source) = e.source() {
                println!("Causado por: {}", source);
            }
        }
    }
}