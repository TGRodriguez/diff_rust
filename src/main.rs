use diff::lcs;

/// Flujo principal del programa.
///
/// Se espera que se reciban dos paths a dos archivos
/// de texto como parámetro.
fn main() -> Result<(), &'static str> { 
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        return Err("Error: cantidad inválida de argumentos. Uso: diff <un_archivo> <otro_archivo>");
    }
    if let Err(e) = lcs::diff(&args[1], &args[2]){
        println!("Error al generar diff: {}", e);
        return Err("Error en la lectura de archivos");
    }
    Ok(())
}

