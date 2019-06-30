# BitGrin - Compilación, configuración y ejecución

## Plataformas soportadas

En un largo plazo, es probable que la mayoría de las plataformas sean compatibles en cierta medida.
El lenguaje de programación de BitGrin `rust` ha compilado metas para la mayoría de las plataformas.

¿Qué funciona hasta ahora?

* Linux x86\_64 y MacOS [bitgrin + mining + development]
* Todavía no funciona con windows 10 [bitgrin kind-of builds. No mining yet. Help wanted!]

## Requisitos

* rust 1.34+ (usa [rustup]((https://www.rustup.rs/))- por ejemplo, `curl https://sh.rustup.rs -sSf | sh; source $HOME/.cargo/env`)
  * Si rust está instalado, puede simplemente actualizar la versión con  `rustup update`
* clang
* ncurses y libs (ncurses, ncursesw5)
* zlib libs (zlib1g-dev or zlib-devel)
* pkg-config
* libssl-dev
* linux-headers (reportado como necesario en Alpine linux)
* llvm

Para las distribuciones basadas en Debian (Debian, Ubuntu, Mint, etc), todo en un comando (exceptuando Rust):

```sh
apt install build-essential cmake git libgit2-dev clang libncurses5-dev libncursesw5-dev zlib1g-dev pkg-config libssl-dev llvm
```

Para las Mac:

```sh
xcode-select --install
brew install --with-toolchain llvm
brew install pkg-config
brew install openssl
```

## Pasos para la compilación

```sh
git clone https://github.com/bitgrin/bitgrin.git
cd bitgrin
cargo build --release
```

BitGrin también puede compilarse en modo debug (sin la etiqueta `--release`, pero usando la etiqueta `--debug` o `--verbose`) esto hará que la sincronización rápida sea excesivamente lenta debido a la gran sobrecarga de las operaciones criptográficas.

## Errores de compilación

Vea [Solución de problemas](https://github.com/mimblewimble/docs/wiki/Troubleshooting)

## ¿Qué se ha compilado?

Con una compilación finalizada se obtiene:

* `target/release/bitgrin` - los binarios principales de bitgrin

Todos los datos, configuración y archivos de registro creados y utilizados por BitGrin se encuentran en el directorio oculto `~/.bitgrin` (bajo el directorio home del usuario) por defecto. Puede modificar toda la configuración editando el archivo `~/.bitgrin/main/bitgrin-server.toml`.

También es posible hacer que BitGrin cree sus propios archivos de datos en el directorio actual. Para ello ejecute:

```sh
bitgrin server config
```

Lo que generará un archivo `bitgrin-server.toml` en el directorio actual, preconfigurado para usar el directorio actual para todos sus datos. Ejecutando BitGrin desde un directorio que contiene el archivo `bitgrin-server.toml` usará los valores de ese archivo en lugar de los valores por defecto de `~/.bitgrin/main/bitgrin-server.toml`.

Durante las pruebas, ponga el binario de BitGrin en su ruta de esta manera:

```sh
export PATH=/path/to/bitgrin/dir/target/release:$PATH
```

Donde `path/to/bitgrin/dir` es su ruta absoluta al directorio raíz de la instalación de BitGrin.

Puede ejecutar `bitgrin` directamente (pruebe `bitgrin help` para más opciones).

## Configuración

BitGrin se ejecuta con valores predeterminados, y puede configurarse aún más a través del archivo `bitgrin-server.toml`. Este fichero es generado por bitgrin en su primera ejecución, y contiene documentación sobre cada opción disponible.

Aunque se recomienda que realice toda la configuración de bitgrin server a través de `bitgrin-server.toml`, también es posible suministrar cambios de comandos para bitgrin que anulan cualquier configuración en el archivo.

Para obtener ayuda sobre los comandos de bitgrin y sus cambios intente:

```sh
bitgrin help
bitgrin wallet help
bitgrin client help
```

## Docker

```sh
docker build -t bitgrin -f etc/Dockerfile .
```

Puede ubicar la caché de BitGrin para que se ejecute dentro del contenedor

```sh
docker run -it -d -v $HOME/.bitgrin:/root/.bitgrin bitgrin
```
## Compilación multiplataforma

Rust (cargo) puede compilar BitGrin para muchas plataformas, así que en teoría ejecutar `bitgrin` como un nodo de validación en un dispositivo de baja potencia podría ser posible. Para hacer una compilación cruzada `bitgrin` en una plataforma x86 Linux y generar binarios de ARM, por ejemplo para Raspberry-pi.

## Usando BitGrin

La página de la wiki [Cómo usar bitgrin](https://github.com/mimblewimble/docs/wiki/How-to-use-bitgrin) y las páginas de enlaces tienen más información sobre las características que disponemos, resolución de problemas, etc.

## Minando en BitGrin

Tenga en cuenta que todas las funciones de minería de BitGrin se han trasladado a un paquete independiente llamado [bitgrin_minner](https://github.com/bitgrin/bitgrin-miner). Una vez que el nodo de bitgrin esté listo y funcionando, puede empezar a minar compilando y ejecutando bitgrin-miner con su nodo BitGrin en funcionamiento.
