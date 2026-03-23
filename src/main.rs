mod ex01_verificar_primeiro;
mod ex02_somar_lista;
mod ex03_busca_binaria;
mod ex04_pares_com_soma;
mod ex05_imprimir_pares_e_pares;
mod ex06_potencias_de_dois;
mod ex07_fibonacci_recursivo;
mod ex08_ordenacao_bolha;
mod ex09_produto_de_matrizes;
mod ex10_merge_sort;

fn main() {
    println!("--- Testes\n");

    // Ex01
    let lista = vec![10, 20, 30];
    println!("--- Ex01: {:?}", 
        ex01_verificar_primeiro::verificar_primeiro(&lista));

    // Ex02
    println!("--- Ex02: {}", 
        ex02_somar_lista::somar_lista(&lista));

    // Ex03
    println!("--- Ex03: {:?}", 
        ex03_busca_binaria::busca_binaria(&[1,2,3,4,5], 3));

    // Ex04
    println!("--- Ex04:");
    ex04_pares_com_soma::pares_com_soma(&[1,2,3,4], 5);

    // Ex05
    println!("--- Ex05:");
    ex05_imprimir_pares_e_pares::imprimir_pares_e_pares(&[1,2,3]);

    // Ex06
    println!("--- Ex06:");
    ex06_potencias_de_dois::potencias_de_dois(50);

    // Ex07
    println!("--- Ex07: {}", 
        ex07_fibonacci_recursivo::fibonacci_recursivo(6));

    // Ex08
    let mut v = vec![5, 3, 1, 4, 2];
    ex08_ordenacao_bolha::ordenacao_bolha(&mut v);
    println!("--- Ex08: {:?}", v);

    // Ex09
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];
    let c = ex09_produto_de_matrizes::produto_de_matrizes(&a, &b);
    println!("--- Ex09: {:?}", c);

    // Ex10
    let lista = vec![5, 2, 9, 1, 3];
    let ordenado = ex10_merge_sort::merge_sort(lista);
    println!("--- Ex10: {:?}", ordenado);
}