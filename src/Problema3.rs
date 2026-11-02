pub fn transponer<T: Clone>(matriz: &[Vec<T>]) -> Result<Vec<Vec<T>>, String> {
    let columnas = matriz.first().map_or(0, |fila| fila.len());
    if matriz.iter().any(|fila| fila.len() != columnas) {
        return Err("La matriz debe ser rectangular: todas las filas deben medir lo mismo.".into());
    }
    Ok((0..columnas)
        .map(|columna| matriz.iter().map(|fila| fila[columna].clone()).collect())
        .collect())
}

fn imprimir_matriz(matriz: &[Vec<i32>]) {
    let ancho = matriz
        .iter()
        .flatten()
        .map(|valor| valor.to_string().len())
        .max()
        .unwrap_or(1);
    if matriz.is_empty() {
        println!("[]");
    }
    for fila in matriz {
        let valores = fila
            .iter()
            .map(|valor| format!("{valor:>ancho$}"))
            .collect::<Vec<_>>()
            .join(" ");
        println!("[ {valores} ]");
    }
}

pub fn ejecutar() {
    let matriz = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("Problema 3: matriz transpuesta");
    println!("\nEntrada:");
    imprimir_matriz(&matriz);
    match transponer(&matriz) {
        Ok(resultado) => {
            println!("\nSalida (transpuesta):");
            imprimir_matriz(&resultado);
        }
        Err(error) => eprintln!("{error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_y_matriz_rectangular() {
        assert_eq!(
            transponer(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).unwrap(),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
        let matriz = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let transpuesta = transponer(&matriz).unwrap();
        assert_eq!(transpuesta, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
        assert_eq!(transponer(&transpuesta).unwrap(), matriz);
    }

    #[test]
    fn vacias_y_filas_irregulares() {
        assert!(transponer::<i32>(&[]).unwrap().is_empty());
        assert!(transponer::<i32>(&[vec![], vec![]]).unwrap().is_empty());
        assert!(transponer(&[vec![1, 2], vec![3]]).is_err());
    }
}
