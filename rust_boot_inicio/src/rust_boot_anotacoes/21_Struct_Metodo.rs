#[derive(Debug)]

struct Cliente {
    id: u32,
    nome: String,
    cpf: String,
    salario: f32
}

impl Cliente {
    fn cpf_valido(&self) -> bool {
        if self.cpf.is_empty() {
            return false;
        }
        true
    } // Verifica se o CPF não está vazio, retorna true ou false

    fn add_sobrenome(&mut self) {
        self.nome += " Victor";
    } // Concatena " Victor" ao nome do cliente

    fn aumento_salario(&mut self, valor: f32) {
        self.salario += valor;
    } // Soma o valor recebido ao salário atual do cliente
}

fn main() {
    let mut cliente = Cliente {
        id: 1,
        nome: String::from("Jao"),
        cpf: String::from("111.222.333-45"),
        salario: 3000.0,
    };

    cliente.add_sobrenome();

    cliente.aumento_salario(200.0);

    let validar: &str = if cliente.cpf_valido() {
        "Valido"
    } else {
        "Invalido"
    };

    println!(
        "Cliente {} id {}, cpf {} e salario {}\nresultado: {}",
        cliente.nome,
        cliente.id,
        cliente.cpf,
        cliente.salario,
        validar
    );
}

// Exemplo hsa da 98
fn main() {
  
  let peca: Peca = Peca {
    id: 1,
    codigo_pi: String::from("15570080"),
    nome: String::from("31-2298 - Haste Curvada"),
    proxima_operacao: ProximaOperacao::Recalcar,
    data_entrada: String::from("01/07/2026"),
    lote: String::from("H0501001001"),
    data_saida: None,
  };
  
  let status = match &peca.data_saida {   
    Some(data) => format!("Saiu em: {}", data),
    None       => String::from("Em estoque"),
};

  println!("{:#?}", peca);
  println!("Status: {}", status);

}

// Exemplo de puadas de função

fn main() {
    let conn = Connection::open("pecas.db").expect("Erro ao abrir banco");

    iniciar_banco(&conn);

    let peca = Peca {
        id: 0, // o banco vai gerar o id real
        codigo_pi:        String::from("15570080"),
        nome:             String::from("HSA - Haste Curvada"),
        proxima_operacao: ProximaOperacao::Recalcar,
        lote:             String::from("H0501001001"),
        data_entrada:     String::from("01/07/2026"),
        data_saida:       None,
    };

    inserir_peca(&conn, &peca);
    
    // atualizar_peca(&conn, &peca_atualizada);

    // deletar_peca(&conn, );

    let lista = listar_pecas(&conn);
    for p in &lista {
        let status = match &p.data_saida {
            Some(data) => format!("Saiu em: {}", data),
            None       => String::from("Em estoque"),
        };
        println!("#{} | {} | {} | {} | {} | {}", p.id, p.codigo_pi, p.nome, p.lote, p.data_entrada, status);
    }

}