# Modo de uso
Se realiza la ejecución del programa desde terminal con el comando *cargo run*.
![Ejecución](Proyecto1/images/cargorun.png)

Se observa en pantalla la ejecución del contenedor de logs (docker compose).
![Ejecución](Proyecto1/images/cargorun1.png)

Se abre en una nueva terminal el contenedor de docker compose, así podrá ver las peticiones realizadas, y podrá finalizar la ejecución del contenedor con Ctrl+C.
![Ejecución](Proyecto1/images/VentanaContenedor.png)

Se ejecuta el script.sh.
![Script](Proyecto1/images/Script.png)

Se observa la creación de los contenedores.
![Script](Proyecto1/images/Script1.png)

Se muestra en pantalla la información del contenido json leído del archivo /proc/sysinfo_201901103
![Script](Proyecto1/images/Contenidojson.png)

Se muestra en pantalla la información de la ram y los contenedores de alto y bajo consumo, los contenedores eliminados, y los logs que serán enviados al contenedor de logs.
![Script](Proyecto1/images/informacionContenedores.png)

Se muestra un mensaje después de haber realizado todo, el programa finaliza la ejecución automaticamente del contenedor de los.
![Script](Proyecto1/images/contenedorDetenido.png)

# Instalación

Para poder realizar la ejecución del programa, se deben instalar los siguientes software (debido a que está realizado en ubuntu):
 
### Docker
1. Abre la terminal y ejecuta el siguiente comando y elige la opción de la documentación de acuerdo a los prerrequisitos.

`sudo apt install gnome-terminal`

Para mayor información visita el sitio oficial
[Documentación de docker]: https://docs.docker.com/engine/install/ubuntu/#install-using-the-repository

### Rust
1. Abre la terminal y ejecuta el siguiente comando.

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. Si ya tenías instalado rust solo actualiza, de lo contrario omitir este paso.

`rustup update`

3. Para verificar que la instalación se realizó correctamente ejecuta.

`rustc --version`

4. Si por algún motivo deseas desinstalar rust, ejecuta el comando o de lo contrario omite el paso.

`rustup self uninstall`

Para mayor información visita el sitio oficial
[Documentación de rust]: https://www.rust-lang.org/es/tools/install

# Ejemplos


