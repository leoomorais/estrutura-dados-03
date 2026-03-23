pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut esq: isize = 0;
    let mut dir: isize = lista.len() as isize - 1;

    while esq <= dir {
        let meio = (esq + dir) / 2;
        let idx = meio as usize;

        if lista[idx] == alvo {
            return Some(idx);
        } else if lista[idx] < alvo {
            esq = meio + 1;
        } else {
            dir = meio - 1;
        }
    }
    None
}
