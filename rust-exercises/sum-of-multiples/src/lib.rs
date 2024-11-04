use std::collections::HashSet; // Importa HashSet para almacenar múltiplos únicos.

/// Calcula la suma de todos los múltiplos únicos de los factores dados hasta el límite.
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new(); // Crea un HashSet para almacenar múltiplos sin duplicados.

    // Itera sobre cada factor en la lista de factores
    for &factor in factors {
        if factor != 0 { // Ignora los factores que sean 0 para evitar bucles infinitos.

            // Genera múltiplos del factor en pasos de 'factor', desde 'factor' hasta 'limit'
            for multiple in (factor..limit).step_by(factor as usize) {
                multiples.insert(multiple); // Agrega cada múltiplo del factor al HashSet.
            }
        }
    }

    // Suma todos los múltiplos únicos almacenados en el HashSet y devuelve el total
    multiples.iter().sum()
}
