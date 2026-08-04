#[derive(Debug)]

enum ErroTransacao {
    SaldoInsulficiente {saldo_atual: f64, tentativa_saque: f64 },
    ErroDeAutenticacao,
    ErroDeRede,
}

impl std::fmt::Display for ErroDeTransacao {
    fn imt
}