use crate::app;
use regex::Regex;
use rusqlite::{params, Connection};

use std::time::SystemTime;
use std::{fs, str};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Filetime {
    pub file_id: u64,
    pub filename: String,
    pub modifiedtime: u64,
}

pub fn inicializacion(
    conn: &Connection,
    folder_name: &Option<std::string::String>,
) -> Result<usize, rusqlite::Error> {
    //Si no existen crea las tablas necesarias
    conn.execute(
        "CREATE TABLE IF NOT EXISTS 'time_points' (
        'point_id'	INTEGER,
        'Unix_time'	INTEGER,
        'datatime'	TEXT,
        PRIMARY KEY('point_id')
    );",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS 'files' (
            'file_id' INTEGER PRIMARY KEY AUTOINCREMENT,
            'filename'	TEXT,
            'modifiedtime'	INTEGER
    );",
        [],
    )?;
    conn.execute(
    "CREATE TABLE IF NOT EXISTS 'file_points' (
        'file_points_id'    INTEGER,
        'point_id'	INTEGER,
        'file_id'	INTEGER,
        PRIMARY KEY('file_points_id'),
        FOREIGN KEY('point_id') REFERENCES 'time_points'('point_id') ON UPDATE CASCADE ON DELETE CASCADE,
        FOREIGN KEY('file_id') REFERENCES 'files'('file_id') ON UPDATE CASCADE ON DELETE CASCADE
    );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS 'Configuration' (
            'config_id' INTEGER,
            'folder_name'   TEXT
        );",
        [],
    )?;

    let folder_name_base =
        select_onecolumn::<String>(conn, "folder_name", "Configuration", Some("config_id=1"));

    // Valor ingresado desde la terminal
    match folder_name {
        Some(folname) => {
            if folder_name_base.len() != 0 {
                // Dado que se ingresa un directorio y ya está registrado el directorio en la base, se actualiza
                conn.execute(
                    "UPDATE Configuration SET folder_name = ?1 WHERE config_id=1",
                    [folname],
                )?;
            } else {
                // Dado que se ingresa un directorio y no hay registo en la base, se ingresa
                conn.execute(
                    "INSERT INTO Configuration (config_id, folder_name) VALUES (?1, ?2);",
                    params![1, folname],
                )?;
            }
        }
        None => {
            if folder_name_base.len() == 0 {
                panic!("No se eligió ni existe registro de carpeta");
            }
        }
    }

    Ok(0)
}

pub fn modificados(conn: &mut Connection, fm_cli: &app::FileMcli) -> Vec<Filetime> {
    let root =
        &select_onecolumn::<String>(conn, "folder_name", "Configuration", Some("config_id=1"))[0];

    let mut files: Vec<Filetime> = vec![];
    let baseignore = root.to_owned() + "/" + &fm_cli.basename;

    for entry in WalkDir::new(root).min_depth(1) {
        let nentry = entry.unwrap();
        let metadata = fs::metadata(nentry.clone().path()).unwrap();
        let filename = nentry.path().to_str().unwrap().to_string();

        if metadata.is_file() && filename != baseignore {
            let last_modified = metadata
                .modified()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            files.push(Filetime {
                file_id: 0,
                filename: filename
                    .split_once(&(root.to_owned() + "/"))
                    .unwrap()
                    .1
                    .to_string(),
                modifiedtime: last_modified,
            });
        }
    }
    files
}

/// Indica cuál es el máximo id de una determinada tabla
pub fn max_id(conn: &Connection, columna: &str, tabla: &str) -> u64 {
    struct MaxId(u64);
    let mut stmt = conn
        .prepare(&format!("SELECT max({}) FROM {}", columna, tabla))
        .unwrap_or_else(|_| panic!("Falló en encontrar el índice máximo en {}", tabla));
    let mut max_id_iter = stmt.query_map([], |row| Ok(MaxId(row.get(0)?))).unwrap();
    let id_point: u64 = match max_id_iter.next().unwrap() {
        Ok(x) => x.0,
        Err(_e) => 0,
    };
    id_point
}

/// Permite obtener los datos de una columna
pub fn select_onecolumn<T: rusqlite::types::FromSql>(
    conn: &Connection,
    columna: &str,
    tabla: &str,
    whereop: Option<&str>,
) -> Vec<T> {
    let mut files: Vec<T> = Vec::new();

    let stmt_result = match whereop {
        Some(condicion) => conn.prepare(&format!(
            "SELECT {} FROM {} WHERE {}",
            columna, tabla, condicion
        )),
        None => conn.prepare(&format!("SELECT {} FROM {}", columna, tabla)),
    };

    let mut stmt = stmt_result.expect("Falló en listar los datos");

    let files_id_iter1 = stmt.query_map([], |row| row.get(0)).unwrap();

    for file_id in files_id_iter1 {
        files.push(file_id.unwrap());
    }
    files
}

///diff_points(B, A) Indica que archivos cambiaron en el punto B respecto al A
pub fn diff_points(conn: &Connection, b: u64, a: u64, regex: Option<String>) {
    let re = match regex {
        Some(a) => Regex::new(&a).unwrap(),
        None => Regex::new(r".*").unwrap(),
    };
    let files_a = select_onecolumn::<u64>(
        conn,
        "file_id",
        "file_points",
        Some(&format!("point_id={}", a)),
    );
    let files_b = select_onecolumn::<u64>(
        conn,
        "file_id",
        "file_points",
        Some(&format!("point_id={}", b)),
    );

    let mut file_name;

    println!("\n\n {:?}\n {:?}", &files_a, &files_b);

    println!("\n\nLos archivos borrados o modificados son:\n");
    for file_a in files_a {
        if !files_b.contains(&file_a) {
            file_name = select_onecolumn::<String>(
                conn,
                "filename",
                "files",
                Some(&format!("file_id={}", file_a)),
            )[0]
            .clone();
            if re.is_match(&file_name) {
                println!("{:?}", file_name);
            }
        }
    }
}
