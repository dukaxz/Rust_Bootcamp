// ============================================================
// OPERADORES LÓGICOS E RELACIONADOS EM RUST
// ============================================================

// --- LÓGICOS BOOLEANOS ---
// &&   AND lógico         true && false  → false  (short-circuit)
// ||   OR lógico          true || false  → true   (short-circuit)
// !    NOT lógico         !true          → false

// --- COMPARAÇÃO (retornam bool) ---
// ==   Igual              5 == 5         → true
// !=   Diferente          5 != 3         → true
// <    Menor que          3 < 5          → true
// >    Maior que          5 > 3          → true
// <=   Menor ou igual     5 <= 5         → true
// >=   Maior ou igual     5 >= 3         → true

// --- BITWISE (bit a bit) ---
// &    AND bit a bit      0b1100 & 0b1010  → 0b1000
// |    OR bit a bit       0b1100 | 0b1010  → 0b1110
// ^    XOR bit a bit      0b1100 ^ 0b1010  → 0b0110
// !    NOT bit a bit      !0b00001100u8    → 0b11110011
// <<   Shift esquerda     1u8 << 3         → 8
// >>   Shift direita      8u8 >> 3         → 1

// --- ATRIBUIÇÃO COMPOSTA (bitwise) ---
// &=   x &= y   →  x = x & y
// |=   x |= y   →  x = x | y
// ^=   x ^= y   →  x = x ^ y
// <<= x <<= y   →  x = x << y
// >>= x >>= y   →  x = x >> y

// --- OUTROS OPERADORES COM COMPORTAMENTO LÓGICO ---

// ?    Propagação de erro (early return se Err ou None)
//      let val = alguma_funcao()?;
//      Equivale a:  match result { Ok(v) => v, Err(e) => return Err(e) }
//      Funciona com Result<T, E> e Option<T>

// ..   Range exclusivo    0..5   → 0, 1, 2, 3, 4
// ..=  Range inclusivo    0..=5  → 0, 1, 2, 3, 4, 5
//      Muito usado em match e for

// @    Bind em pattern matching (captura e testa ao mesmo tempo)
//      match numero {
//          n @ 1..=10 => println!("entre 1 e 10: {n}"),
//          _          => println!("fora do range"),
//      }

// |    Múltiplos padrões em match (diferente do OR bitwise)
//      match x {
//          1 | 2 | 3 => println!("um, dois ou três"),
//          _         => println!("outro"),
//      }

// _    Padrão curinga — ignora o valor
//      let _ = funcao_que_retorna_algo();
//      match x { _ => println!("qualquer coisa") }

// ============================================================