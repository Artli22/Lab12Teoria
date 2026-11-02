pub fn eliminar<T: PartialEq + Clone>(lista: &[T], borrar: &[T]) -> Vec<T> {
    lista
        .iter()
        .filter(|elemento| !borrar.contains(elemento))
        .cloned()
        .collect()
}

pub fn ejecutar() {
    let lista = vec![
        "rojo", "verde", "azul", "amarillo", "gris", "blanco", "negro",
    ];
    let borrar = vec!["amarillo", "cafe", "blanco"];
    println!("Problema 4: eliminar elementos de una lista");
    println!("Entrada: {lista:?}");
    println!("Elementos a borrar: {borrar:?}");
    println!("Salida: {:?}", eliminar(&lista, &borrar));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_ignora_elementos_ausentes() {
        let lista = vec![
            "rojo", "verde", "azul", "amarillo", "gris", "blanco", "negro",
        ];
        assert_eq!(
            eliminar(&lista, &["amarillo", "cafe", "blanco"]),
            vec!["rojo", "verde", "azul", "gris", "negro"]
        );
        assert_eq!(lista.len(), 7);
    }

    #[test]
    fn repetidos_vacios_y_eliminacion_total() {
        assert_eq!(eliminar(&[1, 2, 1, 3, 2], &[1, 1, 9]), vec![2, 3, 2]);
        assert_eq!(eliminar(&[1, 2], &[]), vec![1, 2]);
        assert!(eliminar::<i32>(&[], &[1]).is_empty());
        assert!(eliminar(&[1, 1], &[1]).is_empty());
    }
}
