# Generador de contraseñas (Rust)

Genera contraseñas con aleatoriedad criptográfica del sistema operativo (`OsRng`), de 32 caracteres por defecto, mezclando minúsculas, mayúsculas, números y símbolos. Para guardar y verificar contraseñas, produce hashes **Argon2id** con sal aleatoria independiente por contraseña (64 MiB, 3 pasadas, 1 hilo). El hash no es la contraseña ni permite recuperarla.

## Ejecutar en la terminal

Se necesita Rust y Cargo. Si no están instalados, obtén Rust en <https://rustup.rs/>. Desde esta carpeta:

```sh
./generar-contrasena
./generar-contrasena --longitud 40
./generar-contrasena --cantidad 3
./generar-contrasena --sin-hash
./generar-contrasena --ayuda
```

La primera ejecución compila el programa; las siguientes reutilizan la compilación. También se puede usar `cargo run --release -- [opciones]`. Se admiten longitudes de 16 a 256 y cantidades de 1 a 20.

Para comprobar una contraseña guardada frente a su hash (pega solo el hash como argumento):

```sh
./generar-contrasena --verificar '$argon2id$v=19$m=65536,t=3,p=1$...'
```

Se solicita la contraseña sin mostrarla en pantalla ni pasarla como argumento del shell. Si no coincide, el programa devuelve código de salida 1.

**Importante:** una contraseña no puede garantizarse «incrackeable». Su resistencia depende de su aleatoriedad, longitud y del cuidado al almacenarla. Guarda la contraseña en un gestor de contraseñas y el hash solo donde necesites verificarla; no almacenes ambos juntos. Mostrar contraseñas en la terminal puede dejarlas en el historial de desplazamiento de la terminal o en registros si rediriges la salida. No pases contraseñas como argumentos de comandos.

## Pruebas

```sh
cargo test
```
