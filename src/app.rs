use std::num::ParseIntError;
use std::str::FromStr;

use clap::Parser;

/// Monitorea cambios en directorios usando Sqlite
#[derive(Parser, Debug)]
#[clap(version = "0.5.0", name = "filemonitor")]
pub struct FileMcli {
    /// Especificar nombre base datos
    #[clap(short, long, default_value = "base.db")]
    pub basename: String,

    ///Especificar nombre carpeta a analizar
    #[clap(short, long)]
    pub foldername: Option<String>,

    ///Verbose Output
    #[clap(short, long)]
    pub verbose: bool,

    ///Crea un nuevo punto
    #[clap(short, long)]
    pub new: bool,

    ///Remover un punto
    #[clap(short, long)]
    pub remove: Option<u64>,

    ///Listar puntos
    #[clap(short, long)]
    pub listpoints: bool,

    ///Comparar dos puntos, ej. punto 3 con el 1: -c 3_1
    #[clap(short, long)]
    pub compare: Option<CompareStruct>,

    ///Usa un regex especificado para filtrar la salida.
    #[clap(long)]
    pub filter: Option<String>,
}

#[derive(Debug)]
pub struct CompareStruct {
    pub b: u64,
    pub a: u64,
}

impl FromStr for CompareStruct {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let struc: Vec<&str> = s.split('_').collect();

        Ok(CompareStruct {
            b: struc[0].parse::<u64>()?,
            a: struc[1].parse::<u64>()?,
        })
    }
}
