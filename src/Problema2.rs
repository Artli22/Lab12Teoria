use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Valor {
    Texto(String),
    Numero(i64),
}

pub type Diccionario = BTreeMap<String, Valor>;

/// Devuelve una copia ordenada; exige que la clave exista y tenga un mismo tipo.
pub fn ordenar(lista: &[Diccionario], clave: &str) -> Result<Vec<Diccionario>, String> {
    let tipo = lista.first().and_then(|d| d.get(clave));
    for (indice, diccionario) in lista.iter().enumerate() {
        let valor = diccionario
            .get(clave)
            .ok_or_else(|| format!("Falta la clave '{clave}' en el elemento {indice}."))?;
        if tipo
            .is_some_and(|primero| std::mem::discriminant(primero) != std::mem::discriminant(valor))
        {
            return Err(format!("La clave '{clave}' contiene tipos diferentes."));
        }
    }
    let mut resultado = lista.to_vec();
    resultado.sort_by_key(|diccionario| diccionario.get(clave).cloned());
    Ok(resultado)
}

fn telefono(marca: &str, modelo: i64, color: &str) -> Diccionario {
    BTreeMap::from([
        ("make".into(), Valor::Texto(marca.into())),
        ("model".into(), Valor::Numero(modelo)),
        ("color".into(), Valor::Texto(color.into())),
    ])
}

pub fn ejecutar() {
    let lista = vec![
        telefono("Nokia", 216, "Black"),
        telefono("Mi Max", 2, "Gold"),
        telefono("Samsung", 7, "Blue"),
    ];
    let clave = "model";
    println!("Problema 2: ordenar diccionarios por '{clave}'");
    println!("Entrada: {lista:#?}");
    match ordenar(&lista, clave) {
        Ok(resultado) => println!("Salida: {resultado:#?}"),
        Err(error) => eprintln!("{error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_numerico_sin_modificar_original() {
        let lista = vec![
            telefono("Nokia", 216, "Black"),
            telefono("Mi Max", 2, "Gold"),
            telefono("Samsung", 7, "Blue"),
        ];
        assert_eq!(
            ordenar(&lista, "model").unwrap(),
            vec![lista[1].clone(), lista[2].clone(), lista[0].clone()]
        );
        assert_eq!(lista[0]["model"], Valor::Numero(216));
    }

    #[test]
    fn texto_empates_y_lista_vacia() {
        let lista = vec![
            telefono("Z", 1, "X"),
            telefono("A", 2, "Y"),
            telefono("A", 3, "Z"),
        ];
        assert_eq!(
            ordenar(&lista, "make").unwrap(),
            vec![lista[1].clone(), lista[2].clone(), lista[0].clone()]
        );
        assert_eq!(ordenar(&[], "model").unwrap(), vec![]);
    }

    #[test]
    fn rechaza_claves_ausentes_y_tipos_mezclados() {
        let a = telefono("A", 1, "X");
        assert!(ordenar(std::slice::from_ref(&a), "inexistente").is_err());
        let mut b = a.clone();
        b.insert("model".into(), Valor::Texto("uno".into()));
        assert!(ordenar(&[a, b], "model").is_err());
    }
}