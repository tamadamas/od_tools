use crate::error::AppError;
use calamine::{Xlsx, open_workbook};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

const LAST_HOUR: u32 = 73;

// const OVERVIEW_SHEET: &str = "Overview";
// const IMPS_SHEET: &str = "Imps";

#[allow(dead_code)]
pub struct GameLogGenerator {
    path: PathBuf,
    workbook: Xlsx<BufReader<File>>,
    current_hour: u32,
    sim_hour: u32,
}

impl GameLogGenerator {
    /// Creates a new generator and loads the Excel file.
    pub fn new(path: &Path) -> Result<Self, AppError> {
        let workbook = open_workbook(path).map_err(|e| AppError::ExcelError {
            path: path.to_path_buf(),
            source: e,
        })?;

        Ok(Self {
            path: path.to_path_buf(),
            workbook,
            current_hour: 0,
            sim_hour: 0,
        })
    }

    /// Sets the current hour, adjusting for the 0-based vs 1-based indexing
    /// and the 3-row header offset in the sim.
    fn set_current_hour(&mut self, hr: u32) {
        self.current_hour = hr;
        self.sim_hour = hr + 3; // Sim rows are offset by 3
    }

    /// Main execution loop.
    pub fn execute(&mut self, specific_hour: Option<u32>) -> Result<String, AppError> {
        let mut full_log = String::new();

        if let Some(hr) = specific_hour {
            self.set_current_hour(hr);
            full_log.push_str(&self.execute_actions_for_current_hour()?);
        } else {
            for hr in 1..=LAST_HOUR {
                self.set_current_hour(hr);
                let hour_log = self.execute_actions_for_current_hour()?;
                if !hour_log.is_empty() {
                    full_log.push_str(&hour_log);
                    full_log.push('\n');
                }
            }
        }
        Ok(full_log)
    }

    /// Executes all action methods for the currently set hour.
    fn execute_actions_for_current_hour(&mut self) -> Result<String, AppError> {
        Ok("done!".to_string())
    }
    //     let mut output = String::new();

    //     // In Rust, we can call methods directly. This is often cleaner
    //     // than a vector of function pointers unless true dynamic dispatch is needed.
    //     let actions = [
    //         self.tick_action()?,
    //         self.draft_rate_action()?,
    //         self.construction_action()?, // Example action
    //                                      // ... add other action calls here
    //     ];

    //     for action_result in actions.iter().filter(|s| !s.is_empty()) {
    //         output.push_str(action_result);
    //         if !action_result.ends_with('\n') {
    //             output.push('\n');
    //         }
    //     }

    //     // Skip empty ticks
    //     if output.ends_with("======\n") || output.is_empty() {
    //         return Ok(String::new());
    //     }

    //     Ok(output)
    // }

    // --- Action Implementations ---

    // fn tick_action(&self) -> Result<String, AppError> {
    //     let date_str = self.read_value(OVERVIEW_SHEET, "B15")?;
    //     let local_time_str = self.read_value_by_hour(IMPS_SHEET, "BY")?;
    //     let dom_time_str = self.read_value_by_hour(IMPS_SHEET, "BZ")?;

    //     // Use chrono for robust date/time parsing
    //     let date = chrono::NaiveDate::parse_from_str(&date_str, "%m/%d/%Y")
    //         .or_else(|_| chrono::NaiveDate::parse_from_str(&date_str, "%Y/%m/%d"))?;

    //     let local_time = chrono::NaiveTime::parse_from_str(&local_time_str, "%H:%M")?;
    //     let dom_time = chrono::NaiveTime::parse_from_str(&dom_time_str, "%H:%M")?;

    //     let local_datetime = chrono::NaiveDateTime::new(date, local_time);
    //     let dom_datetime = chrono::NaiveDateTime::new(date, dom_time);

    //     Ok(format!(
    //         "====== Protection Hour: {} ( Local Time: {} ) ( Domtime: {} ) ======\n",
    //         self.current_hour,
    //         local_datetime.format("%-I:%M:%S %p %-m/%-d/%Y"),
    //         dom_datetime.format("%-I:%M:%S %p %-m/%-d/%Y")
    //     ))
    // }

    // fn draft_rate_action(&self) -> Result<String, AppError> {
    //     // ... implementation would be similar to Go, using read_value helpers ...
    //     Ok(String::new()) // Placeholder
    // }

    // fn construction_action(&self) -> Result<String, AppError> {
    //     // Example showing iteration and string building
    //     let mut built_items = Vec::new();
    //     let building_names = ["Homes", "Alchemies", "Farms"]; // etc.
    //     let cols = ["O", "P", "Q"]; // etc.

    //     for (i, col) in cols.iter().enumerate() {
    //         let value = self.read_int_by_hour("Construction", col)?;
    //         if value > 0 {
    //             built_items.push(format!("{} {}", value, building_names[i]));
    //         }
    //     }

    //     if built_items.is_empty() {
    //         return Ok(String::new());
    //     }

    //     let plat_cost = self.read_int_by_hour("Construction", "AQ")?;
    //     let lumber_cost = self.read_int_by_hour("Construction", "AR")?;

    //     Ok(format!(
    //         "Construction of {} started at a cost of {} platinum and {} lumber.\n",
    //         built_items.join(", "),
    //         plat_cost,
    //         lumber_cost
    //     ))
    // }

    // --- Helper Functions ---

    // fn get_range(&self, sheet_name: &str) -> Result<Range<DataType>, AppError> {
    //     self.workbook
    //         .worksheet_range(sheet_name)
    //         .ok_or_else(|| AppError::Custom(format!("Sheet '{}' not found", sheet_name)))?
    //         .map_err(AppError::from)
    // }

    // fn read_value(&self, sheet: &str, cell_ref: &str) -> Result<String, AppError> {
    //     let (row, col) = calamine::CellReference::from_str(cell_ref)
    //         .map_err(|e| AppError::Custom(e.to_string()))?
    //         .to_tuple();

    //     let range = self.get_range(sheet)?;
    //     let cell = range.get((row, col)).ok_or(AppError::CellNotFound {
    //         sheet: sheet.to_string(),
    //         cell: cell_ref.to_string(),
    //     })?;

    //     Ok(cell.to_string().trim().to_string())
    // }

    // fn read_value_by_hour(&self, sheet: &str, col: &str) -> Result<String, AppError> {
    //     self.read_value(sheet, &format!("{}{}", col, self.sim_hour))
    // }

    // fn read_int_by_hour(&self, sheet: &str, col: &str) -> Result<i64, AppError> {
    //     let val_str = self.read_value_by_hour(sheet, col)?;
    //     if val_str.is_empty() {
    //         return Ok(0);
    //     }
    //     val_str
    //         .replace(',', "")
    //         .parse::<i64>()
    //         .map_err(|e| AppError::ParseInt {
    //             value: val_str,
    //             source: e,
    //         })
    // }
}
