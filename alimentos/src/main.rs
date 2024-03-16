use std::io;
use std::collections::HashSet;

fn main() {
    let mut alimentos: HashSet<String> = HashSet::new();

    println!("Bienvenido al sistema de gestión de alimentos");

    loop {
        println!("Por favor, introduzca un alimento (o 'salir' para terminar):");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Error al leer la entrada");

        let input = input.trim().to_lowercase();

        if input == "salir" {
            break;
        }

        alimentos.insert(input);
    }

    println!("Lista de alimentos guardados:");
    for alimento in alimentos.iter() {
        println!("{}", alimento);
    }
}
