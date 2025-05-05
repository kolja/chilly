
use bevy::prelude::Res;
use crate::grid::{CellType, Grid};

pub fn print_ways_html(grid: Res<Grid>) {
    let mut html = String::new();

    html.push_str("<table style='border-collapse: collapse;'>");

    for row in 0..grid.size.1 {
        html.push_str("<tr>");
        for col in 0..grid.size.0 {
            let index = row * grid.size.0 + col;
            let cell = &grid.cells[index];
            let cell_color = match cell.cell_type {
                CellType::Empty => "white",
                CellType::Obstacle => "gray",
                CellType::Portal(_) => "lightblue",
                CellType::Exit => "yellow",
            };
            let (way1, way2, way3, way4) = cell.ways;

            let way1_str = way1.map_or("&nbsp;".to_string(), |x| format!("{:>3}", x));
            let way2_str = way2.map_or("&nbsp;".to_string(), |x| format!("{:>3}", x));
            let way3_str = way3.map_or("&nbsp;".to_string(), |x| format!("{:>3}", x));
            let way4_str = way4.map_or("&nbsp;".to_string(), |x| format!("{:>3}", x));

            html.push_str(&format!(
                "<td style='background: {}; width: 80px; height: 80px; border: 1px solid black; text-align: center; vertical-align: middle;'>
                    <div style='position: relative; width: 100%; height: 100%;'>
                        <div style='position: absolute; top: 2px; left: 2px; font-size: smaller;'>{}</div>
                        <div style='position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);'>
                            <div>{} {}</div>
                            <div>{} {}</div>
                        </div>
                    </div>
                </td>",
                cell_color, cell.id, way1_str, way2_str, way3_str, way4_str
            ));
        }
        html.push_str("</tr>");
    }

    html.push_str("</table>");
    std::fs::write("grid.html", html.as_str()).unwrap();
    println!("Wrote grid.html");
}
