pub fn validar_cpf(cpf: &str) -> bool {
    let digits: String = cpf
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .collect();

    if digits.len() != 11 {
        return false;
    }

    if digits.chars().all(|ch| ch == digits.chars().next().unwrap()) {
        return false;
    }

    let mut soma = 0;
    for (index, ch) in digits[..9].chars().enumerate() {
        soma += (ch.to_digit(10).unwrap() as i32) * (10 - index as i32);
    }

    let primeiro_digito = 11 - (soma % 11);
    let primeiro_digito = if primeiro_digito >= 10 { 0 } else { primeiro_digito };

    let mut soma = 0;
    for (index, ch) in digits[..10].chars().enumerate() {
        soma += (ch.to_digit(10).unwrap() as i32) * (11 - index as i32);
    }

    let segundo_digito = 11 - (soma % 11);
    let segundo_digito = if segundo_digito >= 10 { 0 } else { segundo_digito };

    let expected = format!("{}{}", primeiro_digito, segundo_digito);
    let actual = digits[9..].to_string();

    expected == actual
}

#[cfg(test)]
mod tests {
    use super::validar_cpf;

    #[test]
    fn aceita_cpf_valido() {
        assert!(validar_cpf("529.982.247-25"));
    }

    #[test]
    fn rejeita_cpf_invalido() {
        assert!(!validar_cpf("123.456.789-00"));
    }

    #[test]
    fn rejeita_cpf_com_todos_os_digitos_iguais() {
        assert!(!validar_cpf("111.111.111-11"));
    }
}
