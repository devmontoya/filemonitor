mod app;
mod ops;
mod point_handling;

use clap::Parser;

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let fm_cli = app::FileMcli::parse();

    //Preparación base de datos
    let mut conn = Connection::open(&fm_cli.basename)?;

    ops::inicializacion(&conn, &fm_cli.foldername)?;

    //Obtiene el máximo id en points_time
    let max_id_point = ops::max_id(&conn, "point_id", "time_points");

    if fm_cli.new {
        point_handling::new_point(&mut conn, max_id_point + 1, &fm_cli)?;
    } else {
        match fm_cli.remove {
            Some(a) => point_handling::drop_point(&mut conn, a)?,
            None => {}
        }
    }

    //Enlistar puntos existentes
    if fm_cli.listpoints {
        println!("Lista actual de puntos : \n");
        println!("\tID\tCreationTime");
        for point in point_handling::read_list_points(&conn) {
            println!("\t{}\t{}", point.id, point.datatime);
        }
    }

    // Realiza una comparación entre el punto ingresado y último punto existente en la database
    match fm_cli.compare {
        Some(app::CompareStruct { a, b }) => ops::diff_points(&conn, b, a, fm_cli.filter.clone()),
        None => {}
    }

    Ok(())
}
