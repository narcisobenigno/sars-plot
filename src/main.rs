extern crate csv;
extern crate serde;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;

use plotters::prelude::*;
use std::iter::FromIterator;

mod row;

struct ParsedRow {
    pub year_week: i32,
    pub cases: f64,
}

struct IndexedYearWeek {
    pub index: usize,
    pub year_week: i32,
}
struct IndexedCases {
    pub index: usize,
    pub cases: f64,
}

const OUT_FILE_NAME: &'static str = "./sample.png";

fn main() -> Result<(), Box<dyn Error>> {
    let mut csv_file = csv::ReaderBuilder::new().delimiter(b';').from_path("./serie_temporal_com_estimativas_recentes_sem_filtro_sintomas.csv")?;
    let mut rows: Vec<ParsedRow> = vec![];
    let mut min_cases = 0.0;
    let mut max_cases = 0.0;
    let mut min_year_week = 0;
    let mut max_year_week = 0;
    let mut initialized = false;

    let mut xs: HashSet<i32> = HashSet::new();

    for csv_row in csv_file.deserialize() {
        let row: row::Row = csv_row?;
        if row.unidade_da_federacao == "Brasil" && row.dado == "srag" && row.casos_semanais_reportados_ate_a_ultima_atualizacao != "" && row.casos_semanais_reportados_ate_a_ultima_atualizacao != "0" {
            let cases = row.casos_semanais_reportados_ate_a_ultima_atualizacao.replace(",", ".").parse::<f64>().unwrap();
            let year_week = format!(
                "{:?}{:02}",
                row.ano_epidemiologico.parse::<i32>().unwrap(),
                row.semana_epidemiologica.parse::<i32>().unwrap(),
            ).parse::<i32>().unwrap();
            rows.push(ParsedRow { year_week, cases });
            xs.insert(year_week);

            if !initialized {
                min_cases = cases;
                min_year_week = year_week;
                initialized = true;
            }

            if cases > max_cases {
                max_cases = cases;
            }
            if cases < min_cases {
                min_cases = cases;
            }

            if max_year_week < year_week {
                max_year_week = year_week;
            }
            if min_year_week > year_week {
                min_year_week = year_week;
            }
        }
    }
    rows.sort_by(|r1, r2|
        if r1.year_week < r2.year_week {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    );
    let mut sorted_xs = Vec::from_iter(xs.iter().clone());
    sorted_xs.sort();

    let mut indexed_year_week : Vec<IndexedYearWeek> = vec![];
    let mut indexed_cases : Vec<IndexedCases> = vec![];
    for x in rows {
        let index = sorted_xs.iter().position(|&sx| *sx == x.year_week).unwrap();
        indexed_year_week.push(IndexedYearWeek {
            index,
            year_week: x.year_week,
        });
        indexed_cases.push(IndexedCases {
            index,
            cases: x.cases,
        });
    }

    let root_area = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let root_area = root_area.titled("Image Title", ("sans-serif", 60))?;

    let (upper, _) = root_area.split_vertically(512);

    let mut cc = ChartBuilder::on(&upper)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("SARS", ("sans-serif", 40))
        .build_cartesian_2d((0 as usize)..(sorted_xs.len() as usize), min_cases..max_cases)?;

    cc.configure_mesh()
        .x_labels(sorted_xs.len()/52).y_labels(15)
        .disable_mesh()
        .y_label_formatter(&|y| format!("{:?}", y))
        .x_label_formatter(&|x| { format!("{:?}", sorted_xs[*x])})
        .draw()?;

    let tuples: Vec<(usize, f64)> = indexed_cases.iter().map(|r| (r.index, r.cases)).collect();
    cc.draw_series(LineSeries::new(tuples, &RED))?
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &RED));


    cc.configure_series_labels().border_style(&BLACK).draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
