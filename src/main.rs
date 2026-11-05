#[path = "Problema2.rs"]
mod problema2;
#[path = "Problema3.rs"]
mod problema3;
#[path = "Problema4.rs"]
mod problema4;
fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [p] if matches!(p.as_str(), "2" | "Problema2") => {
            problema2::ejecutar();
            std::process::ExitCode::SUCCESS
        }
        [p] if matches!(p.as_str(), "3" | "Problema3") => {
            problema3::ejecutar();
            std::process::ExitCode::SUCCESS
        }
        [p] if matches!(p.as_str(), "4" | "Problema4") => {
            problema4::ejecutar();
            std::process::ExitCode::SUCCESS
        }  
        [] => {
            println!(
                "Uso: cargo run -- Problema2 | Problema3 | Problema4 (también acepta 2 | 3 | 4)"
            );
            std::process::ExitCode::SUCCESS
        }
        _ => {
            eprintln!("Problema inválido o todavía no implementado.");
            std::process::ExitCode::FAILURE
        }
    }
}
