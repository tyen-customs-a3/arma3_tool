use prettytable::{Table, Row, Cell, format};

pub fn create_standard_table() -> Table {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separator(
            format::LinePosition::Title,
            format::LineSeparator::new('-', '+', '+', '+')
        )
        .padding(1, 1)
        .build();
    table.set_format(format);
    table
}

pub fn create_summary_table() -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("Metric"),
        Cell::new("Value"),
    ]));
    table
}

pub fn table_to_string(table: &Table) -> String {
    let mut buffer = Vec::new();
    table.print(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
} 