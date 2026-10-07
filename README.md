# Generador de contraseñas (Rust)

Genera de forma automatizada contraseñas con aleatoriedad criptográfica del sistema operativo (`OsRng`), de 32 caracteres por defecto, mezclando minúsculas, mayúsculas, números y símbolos. Genera simultáneamente el hash **Argon2id** con sal aleatoria independiente (64 MiB, 3 pasadas, 1 hilo).

Optimizado para **ejecución única y automatizada**, sin interacción manual ni dependencias superfluas.

## Ejecución automatizada

El script `./generar-contrasena` detecta si el binario de release ya está compilado y actualizado. Si no existe o se modificó el código, compila automáticamente en segundo plano y ejecuta de forma directa el binario nativo:

```sh
./generar-contrasena
```

Salida típica:
```text
Contraseña: {f(K9%mE4n_R?zT[Ip&d2e]3mZ2_q&I!
Hash Argon2id: $argon2id$v=19$m=65536,t=3,p=1$u3...
```

### Opciones admitidas

```sh
# Personalizar la longitud (16 a 256 caracteres)
./generar-contrasena --longitud 40

# Generar solo la contraseña sin calcular el hash Argon2id
./generar-contrasena --sin-hash

# Ver la ayuda
./generar-contrasena --ayuda
```

## Pruebas

Para ejecutar la suite de pruebas unitarias:

```sh
cargo test
```
