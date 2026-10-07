use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::rngs::OsRng;
use rand::{seq::SliceRandom, Rng};
use std::env;

const MIN_LENGTH: usize = 16;
const MAX_LENGTH: usize = 256;
const DEFAULT_LENGTH: usize = 32;
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{}:,.?";

// 64 MiB, tres pasadas y un hilo. La sal se genera de forma independiente para cada hash.
fn argon2id() -> Argon2<'static> {
    let params = Params::new(64 * 1024, 3, 1, None).expect("parámetros Argon2id válidos");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

fn generar(longitud: usize) -> String {
    let mut rng = OsRng;
    let grupos = [LOWER, UPPER, DIGITS, SYMBOLS];
    let alfabeto: Vec<u8> = grupos.concat();
    let mut bytes = Vec::with_capacity(longitud);

    // Garantiza una letra minúscula, una mayúscula, un dígito y un símbolo.
    for grupo in grupos {
        bytes.push(grupo[rng.gen_range(0..grupo.len())]);
    }
    while bytes.len() < longitud {
        bytes.push(alfabeto[rng.gen_range(0..alfabeto.len())]);
    }
    bytes.shuffle(&mut rng);
    String::from_utf8(bytes).expect("el alfabeto solo contiene ASCII")
}

fn hash_contrasena(contrasena: &str) -> Result<String, String> {
    let sal = SaltString::generate(&mut OsRng);
    argon2id()
        .hash_password(contrasena.as_bytes(), &sal)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("no se pudo generar el hash: {e}"))
}

enum Accion {
    Generar {
        longitud: usize,
        con_hash: bool,
    },
    Ayuda,
}

fn numero(valor: Option<String>, nombre: &str, min: usize, max: usize) -> Result<usize, String> {
    let valor = valor.ok_or_else(|| format!("falta el valor de {nombre}"))?;
    let n = valor
        .parse::<usize>()
        .map_err(|_| format!("{nombre} debe ser un número entero"))?;
    if !(min..=max).contains(&n) {
        return Err(format!("{nombre} debe estar entre {min} y {max}"));
    }
    Ok(n)
}

fn argumentos() -> Result<Accion, String> {
    let mut args = env::args().skip(1);
    let mut longitud = DEFAULT_LENGTH;
    let mut con_hash = true;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--ayuda" | "-h" => return Ok(Accion::Ayuda),
            "--longitud" | "-l" => {
                longitud = numero(args.next(), "--longitud", MIN_LENGTH, MAX_LENGTH)?;
            }
            "--sin-hash" => {
                con_hash = false;
            }
            _ => return Err(format!("opción desconocida: {arg}. Usa --ayuda")),
        }
    }

    Ok(Accion::Generar {
        longitud,
        con_hash,
    })
}

fn ayuda() {
    println!("Generador de contraseñas seguras (Rust / Argon2id)\n");
    println!("Uso:");
    println!("  ./generar-contrasena [--longitud N] [--sin-hash]");
    println!("  ./generar-contrasena --ayuda\n");
    println!("Por defecto genera 1 contraseña de 32 caracteres y su hash Argon2id.");
    println!("Longitud permitida: {MIN_LENGTH}-{MAX_LENGTH}.");
}

fn ejecutar() -> Result<(), String> {
    match argumentos()? {
        Accion::Ayuda => ayuda(),
        Accion::Generar {
            longitud,
            con_hash,
        } => {
            let contrasena = generar(longitud);
            println!("Contraseña: {contrasena}");
            if con_hash {
                println!("Hash Argon2id: {}", hash_contrasena(&contrasena)?);
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = ejecutar() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::password_hash::{PasswordHash, PasswordVerifier};

    #[test]
    fn cumple_longitud_y_categorias() {
        for longitud in [MIN_LENGTH, DEFAULT_LENGTH, MAX_LENGTH] {
            let contrasena = generar(longitud);
            assert_eq!(contrasena.len(), longitud);
            assert!(contrasena.bytes().any(|c| LOWER.contains(&c)));
            assert!(contrasena.bytes().any(|c| UPPER.contains(&c)));
            assert!(contrasena.bytes().any(|c| DIGITS.contains(&c)));
            assert!(contrasena.bytes().any(|c| SYMBOLS.contains(&c)));
        }
    }

    #[test]
    fn hash_con_sal_y_verificacion() {
        let contrasena = generar(DEFAULT_LENGTH);
        let primero = hash_contrasena(&contrasena).unwrap();
        let segundo = hash_contrasena(&contrasena).unwrap();
        assert!(primero.starts_with("$argon2id$v=19$m=65536,t=3,p=1$"));
        assert_ne!(primero, segundo);
        let parsed = PasswordHash::new(&primero).unwrap();
        assert!(argon2id()
            .verify_password(contrasena.as_bytes(), &parsed)
            .is_ok());
        assert!(argon2id()
            .verify_password(b"contrasena-incorrecta", &parsed)
            .is_err());
    }
}
