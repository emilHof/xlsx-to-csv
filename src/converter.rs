pub struct Converter;

impl Converter {
    pub fn workbook_to_csv(workbook_path: String) {
        let xlsx = ooxml::document::SpreadsheetDocument::open(workbook_path).expect("open xlsx file");
        let workbook = xlsx.get_workbook();

        let sheet_names = workbook.worksheet_names();

        for sheet in sheet_names {

            let mut wtr = csv::WriterBuilder::new()
                .delimiter(b',')
                .from_path("output.csv")
                .expect("open file for output");
            Converter::worksheet_to_csv(&workbook, &sheet, &mut wtr, false);
        }
    }

    fn worksheet_to_csv<W>(
        workbook: &ooxml::document::Workbook,
        sheet: &str,
        wtr: &mut csv::Writer<W>,
        header: bool,
    ) where
        W: std::io::Write,
    {
        let worksheet = workbook
            .get_worksheet_by_name(&sheet)
            .expect("worksheet name error");
        let mut iter = worksheet.rows();
        if header {
            let header = iter.next();
            if header.is_none() {
                return;
            }
            let header = header.unwrap();
            let size = header
                .into_iter()
                .position(|cell| cell.is_empty())
                .expect("find header row size");

            for row in worksheet.rows() {
                let cols: Vec<String> = row
                    .into_iter()
                    .take(size)
                    .map(|cell| cell.to_string().unwrap_or_default())
                    .collect();
                wtr.write_record(&cols).unwrap();
            }
        } else {
            for row in worksheet.rows() {
                let cols: Vec<String> = row
                    .into_iter()
                    .map(|cell| cell.to_string().unwrap_or_default())
                    .collect();
                wtr.write_record(&cols).unwrap();
            }
        }
        wtr.flush().unwrap();
    }
}

#[cfg(test)]
mod test_converter {
    use super::*;

    #[test]
    fn test_workbook_to_csv() {
        Converter::workbook_to_csv("test-converter.xlsx".to_string())
    }
}
