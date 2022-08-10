# Filemonitor

## Descripción

**Filemonitor** es un programa en linea de comandos desarrollado en lenguaje **Rust** que permite realizar seguimiento a un directorio y todos los archivos y sub-directorios que este incluye, registrando **puntos en el tiempo** en una base de datos **SQLite**. Así, es fácil conocer que archivos han sido borrados, modificados o creados entre distintos puntos en el tiempo.

**¿Para qué?**
> R:/ En ciertas tareas que requieran almacenar gran cantidad de datos críticos para posteriormente ser analizados, ej: recolección de datos científicos, bitácoras de laboratorio; es útil tener mayor confianza de no eliminación o modificación imprevista de los archivos.

**Nota importante:**
- El programa no realiza ningún tipo de copia de seguridad.
- Solo ha sido probado en una distribución Linux.
- Actualmente la comparación entre puntos de tiempo se realiza únicamente usando los tiempos de modificación de los archivos.

## Uso

- Realizar compilación: `cargo build --release`
- usar las opciones dadas al ejecutar `./filemonitor -h`

En caso de Ejecutar desde cargo `cargo run --release -- [opciones]`

## Opciones

``` bash
-b, --basename Especificar nombre base datos [default: base.db] 
-c, --compare  Comparar dos puntos, ej: punto 3 con el 1: -c 3_1  
-f, --foldername Especificar nombre carpeta a analizar  
--filter  Usa un regex especificado para filtrar la salida  
-h, --help  Imprime información de ayuda  
-l, --listpoints  Listar puntos  
-n, --new  Crea un nuevo punto  
-r, --remove  Remover un punto  
-v, --verbose  Verbose Output  
-V, --version  Imprime el número de la versión del programa
```
Ejemplo:
Se desea monitorear un directorio llamado "MisDocumentos" ubicado en donde está en el momento la terminal.

`./filemonitor -b mibase.db -n -f MisDocumentos` o si se usa cargo
`cargo run --release -- -b mibase.db -n -f MisDocumentos`

Se puede omitir especificar la base SQLite a usar, en este caso se usaría una con nombre "base.db"

Las siguiente veces que se use el programa no es necesario especificar el directorio ni el nombre de la base SQLite siempre y cuando se esté ejecutando el programa desde la misma ubicación y la base SQLite tenga el nombre por defecto. En caso de cambiar la ubicación de la carpeta, el nombre de la data base o haber usado un nombre diferente a "base.db", será entonces necesario especificarlos.

La opción `--filter` permite que cuando se utilice la opción `-c ` o `--compare` ver únicamente los archivos de interés que cumplan una expresión regular **regex**.

## Roadmap

- Mejorar la documentación general del proyecto, precisamente incluir un esquema.
- Incluir la capacidad de realizar comparaciones entre puntos en el tiempo y archivos usando algoritmos **hash**, actualmente **filemonitor** analiza únicamente las fechas de modificación para dar cuenta si un archivo fue modificado.




