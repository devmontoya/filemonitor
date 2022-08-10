use crate::app;
use crate::ops;

use chrono::Local;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct TimePoints {
    pub id: u64,
    pub unix_time: u64,
    pub datatime: String,
}

/// Obtiene una lista completa de archivos ya registrados
fn read_list_files(conn: &Connection) -> Vec<ops::Filetime> {
    let mut files: Vec<ops::Filetime> = vec![];

    let mut stmt = conn
        .prepare("SELECT file_id, filename, modifiedtime FROM files")
        .expect("Falló en listar los archivos ya registrados");
    let files_id_iter = stmt
        .query_map([], |row| {
            Ok(ops::Filetime {
                file_id: row.get(0)?,
                filename: row.get(1)?,
                modifiedtime: row.get(2)?,
            })
        })
        .unwrap();

    let mut file: ops::Filetime;
    for rowfiles in files_id_iter {
        file = rowfiles.unwrap();
        files.push(file);
    }
    files
}
///Enlista los puntos en points_time
pub fn read_list_points(conn: &Connection) -> Vec<TimePoints> {
    let mut points_time: Vec<TimePoints> = Vec::new();

    let mut stmt = conn
        .prepare("SELECT point_id, unix_time, datatime FROM time_points")
        .expect("Falló en listar un time_points");
    let files_id_iter = stmt
        .query_map([], |row| {
            Ok(TimePoints {
                id: row.get(0)?,
                unix_time: row.get(1)?,
                datatime: row.get(2)?,
            })
        })
        .unwrap();

    let mut point: TimePoints;
    for rowpoint in files_id_iter {
        point = rowpoint.unwrap();
        points_time.push(point);
    }
    points_time
}

pub fn drop_point(conn: &mut Connection, id: u64) -> Result<()> {
    conn.execute("DELETE FROM time_points WHERE point_id=(?)", [id])?;

    let files_infiles = ops::select_onecolumn::<u64>(conn, "file_id", "files", None);
    let files_infiles_points = ops::select_onecolumn::<u64>(conn, "file_id", "file_points", None);
    println!("file_id {:?}", files_infiles);
    println!("file_points {:?}", files_infiles_points);

    let tx = conn.transaction()?;
    for file in files_infiles {
        if !files_infiles_points.contains(&file) {
            tx.execute("DELETE FROM files WHERE file_id=(?)", [&file])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn new_point(conn: &mut Connection, point_id: u64, fm_cli: &app::FileMcli) -> Result<()> {
    //Array con los archivos de la carpeta elegida
    let files = ops::modificados(conn, fm_cli);

    if fm_cli.verbose {
        println!("{:?}", &files);
    }

    let now = Local::now();
    println!("Se crea el punto: {:?}", &point_id);

    // Se evalúa que archivos distan de lo que se tenia previamente
    println!("Se realiza una comparación");
    let filecompare: Vec<ops::Filetime> = read_list_files(conn);

    let mut existing_files: Vec<u64> = vec![];
    let mut new_files: Vec<ops::Filetime> = vec![];

    for file in files {
        match filecompare
            .iter()
            .position(|d| d.filename == file.filename && d.modifiedtime == file.modifiedtime)
        {
            Some(x) => {
                existing_files.push(filecompare[x].file_id);
            }
            None => {
                new_files.push(file);
            }
        }
    }
    //Encuentra cuál actualmente es el máximo file_id
    let mut file_id = ops::max_id(conn, "file_id", "file_points");

    let tx = conn.transaction()?;

    tx.execute(
        "INSERT INTO time_points (point_id, unix_time, datatime) VALUES (?1, ?2, ?3)",
        params![point_id, now.timestamp(), now.to_string()],
    )?;

    for file_id in existing_files {
        tx.execute(
            "INSERT INTO file_points (point_id, file_id) VALUES (?1, ?2)",
            params![point_id, file_id],
        )?;
    }

    for file in new_files {
        file_id += 1;
        println!("Se ingresa nuevo archivo id_{}", &file_id);

        if fm_cli.verbose {
            println!("se agrega: {}", file.filename);
        }
        println!("file {:?}", &file);
        tx.execute(
            "INSERT INTO files (filename, modifiedtime) VALUES (?1, ?2)",
            params![file.filename, file.modifiedtime.to_string()],
        )?;

        tx.execute(
            "INSERT INTO file_points (point_id, file_id) VALUES (?1, ?2)",
            params![point_id, file_id],
        )?;
    }

    tx.commit()?;
    Ok(())
}
