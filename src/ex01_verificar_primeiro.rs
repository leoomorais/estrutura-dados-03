pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    lista.first().copied()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lista_vazia() {
        assert_eq!(verificar_primeiro(&[]), None);
    }
}
