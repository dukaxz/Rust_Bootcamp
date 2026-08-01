// Usando estrutura 
struct Usuario {
    id: u32,
    nome: String,
    saldo: f32,
}

fn main() {
    let usuario: Usuario = Usuario {
        id: 1,
        nome: String::from("Jao"),
        saldo: 1300.0,
    };

    println!("id: {}", usuario.id);
    println!("Nome: {}", usuario.nome);
    println!("Saldo: {}", usuario.saldo);

}

//============================================================================================

// // Usando Array de Array

// fn main() {
//     let usuarios: [[&str; 3]; 2] = [
//         ["1", "Jao", "1300.0"],
//         ["2", "Maria", "2500.0"],
//     ];

//     println!("ID: {}", usuarios[0][0]);
//     println!("Nome: {}", usuarios[0][1]);
//     println!("Saldo: {}", usuarios[0][2]);
// }

//============================================================================================


// // Usando Tupla

// fn main() {
//     let usuario: (u32, &str, f32) = (1, "Jao", 1300.0);

//     println!("ID: {}", usuario.0);
//     println!("Nome: {}", usuario.1);
//     println!("Saldo: {}", usuario.2);
// }


//============================================================================================


// // Usando Vec 

// fn main() {
//     let usuario: Vec<String> = vec![
//         String::from("1"),
//         String::from("Jao"),
//         String::from("1300.0"),
//     ];

//     println!("ID: {}", usuario[0]);
//     println!("Nome: {}", usuario[1]);
//     println!("Saldo: {}", usuario[2]);
// }

//============================================================================================


// // Usando Vec de tuplas

// fn main() {
//     let usuarios: Vec<(u32, &str, f32)> = vec![
//         (1, "Jao", 1300.0),
//         (2, "Maria", 2500.0),
//     ];

//     println!("ID: {}", usuarios[0].0);
//     println!("Nome: {}", usuarios[0].1);
//     println!("Saldo: {}", usuarios[0].2);
// }


//============================================================================================



// Struct padrão — campos com tipos primitivos diretos
struct Usuario {
    id: u32,
    nome: String,
    saldo: f32,
}


// Struct com Option — campos podem ser None (ausente) ou Some(valor)
// Útil quando os dados ainda não foram preenchidos
struct UsuarioNull {
    id: Option<u32>,
    nome: Option<String>,
    saldo: Option<f32>,
}


// Constrói um Usuario padrão com dados reais
fn construir_usuario(nome: String, saldo: f32) -> Usuario {
    Usuario {
        id: 0,         // id padrão inicial
        nome: nome,    // recebe o nome como parâmetro
        saldo: saldo,  // recebe o saldo como parâmetro
    }
}

// Constrói um UsuarioNull — todos os campos começam como None (vazios)
fn construir_usuario_null() -> UsuarioNull {
    UsuarioNull {
        id: None,    // ausente por enquanto
        nome: None,  // ausente por enquanto
        saldo: None, // ausente por enquanto
    }
}

fn main() {

    // --- Exemplo 1: Struct padrão com valores diretos ---
    // let mut usuario = Usuario {
    //     id: 1,
    //     nome: String::from("Jao"),
    //     saldo: 1300.0,
    // };

    // --- Exemplo 2: Usando a função construtora padrão ---
    // let mut usuario = construir_usuario(String::from("Jao"), 2400.0);

    // --- Exemplo 3: Struct padrão vazia (valores zerados) ---
    // let mut usuario = Usuario {
    //     id: 0,
    //     nome: String::new(), // String vazia ""
    //     saldo: 0.0,
    // };

    // --- Exemplo 4: Usando a função construtora Null (todos os campos como None) ---
    let mut usuario = construir_usuario_null();

    // Preenchendo os campos depois — atribuindo Some(valor)
    usuario.id = Some(5);
    usuario.nome = Some(String::from("Jao"));
    usuario.saldo = Some(2400.0);

    // Sobrescrevendo um campo já preenchido
    usuario.nome = Some(String::from("Fabio"));

    // --- Exibindo os valores com unwrap_or (fallback se for None) ---

    // unwrap_or(0) → retorna o u32 dentro do Some, ou 0 se for None
    println!("id: {}", usuario.id.unwrap_or(0));

    // unwrap_or retorna a String dentro do Some, ou "Usuario invalido" se for None
    // as_deref() converte Option<String> → Option<&str> para comparar com &str literal
    println!("Nome: {}", usuario.nome.as_deref().unwrap_or("Usuario invalido"));

    // unwrap_or(0.0) → retorna o f32 dentro do Some, ou 0.0 se for None
    println!("Saldo: {}", usuario.saldo.unwrap_or(0.0));
}

//====================================================================

// Inspecionando o objeto inteiro

#[derive(Debug)]
struct Cliente {
    id: u32,
    nome: String,
}

fn main() {
    let cliente1 = Cliente { id: 1, nome: String::from("Eduardo")};
    let cliente2 = Cliente { id: 2, nome: String::from("Jao")};

    println!("Mostrar conteudo total da struct {:?}", cliente1);
    println!("Mostrar conteudo total da struct formatando com # {:#?}", cliente2);

}