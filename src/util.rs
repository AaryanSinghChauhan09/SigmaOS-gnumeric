//! gnumeric model — GNOME spreadsheet.

#[derive(Debug, Clone)]
pub struct Workbook {
    pub id: String,
    pub title: String,
    pub sheets: Vec<Sheet>,
}

#[derive(Debug, Clone)]
pub struct Sheet {
    pub name: String,
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub row: u32,
    pub col: u32,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub workbooks: Vec<Workbook>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { workbooks: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, title: &str) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.workbooks.push(Workbook { id: id.clone(), title: title.into(), sheets: vec![Sheet { name: "Sheet1".into(), cells: vec![] }] });
        id
    }

    pub fn set_cell(&mut self, wb_id: &str, sheet: &str, row: u32, col: u32, value: &str) -> bool {
        if let Some(w) = self.workbooks.iter_mut().find(|w| w.id == wb_id) {
            if let Some(s) = w.sheets.iter_mut().find(|s| s.name == sheet) {
                s.cells.push(Cell { row, col, value: value.into() });
                return true;
            }
        }
        false
    }

    pub fn list(&self) -> &[Workbook] { &self.workbooks }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn workbook_cell() {
        let mut g = Model::new();
        let id = g.create("Budget");
        assert!(g.set_cell(&id, "Sheet1", 1, 1, "100"));
    }
}
