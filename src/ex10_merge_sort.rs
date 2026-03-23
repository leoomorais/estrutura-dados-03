pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista;
    }

    let meio = lista.len() / 2;
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());

    merge(esquerda, direita)
}

fn merge(esq: Vec<i32>, dir: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::with_capacity(esq.len() + dir.len());
    let mut i = 0;
    let mut j = 0;

    while i < esq.len() && j < dir.len() {
        if esq[i] <= dir[j] {
            resultado.push(esq[i]);
            i += 1;
        } else {
            resultado.push(dir[j]);
            j += 1;
        }
    }

    resultado.extend_from_slice(&esq[i..]);
    resultado.extend_from_slice(&dir[j..]);

    resultado
}
