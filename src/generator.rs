use calamine::{Data, Reader, Xlsx, XlsxError, open_workbook};
use std::{
    fmt,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};
// Using the phf crate for compile-time generated hash maps.
// This is more efficient for static maps than std::collections::HashMap.
use phf::phf_map;

const LAST_HOUR: usize = 73;
const EMPTY_RATE: &str = "Draftrate";

// Game section names (string constants)
pub const OVERVIEW: &str = "Overview";
pub const POPULATION: &str = "Population";
pub const PRODUCTION: &str = "Production";
pub const CONSTRUCTION: &str = "Construction";
pub const EXPLORE: &str = "Explore";
pub const REZONE: &str = "Rezone";
pub const MILITARY: &str = "Military";
pub const MAGIC: &str = "Magic";
pub const TECHS: &str = "Techs";
pub const IMPS: &str = "Imps";
pub const CONSTANTS: &str = "Constants";
pub const RACES: &str = "Races";

// Magic spell names (string constants)
pub const GAIAS_WATCH: &str = "Gaia's Watch";
pub const MINING_STRENGTH: &str = "Mining Strength";
pub const ARES_CALL: &str = "Ares' Call";
pub const MIDAS_TOUCH: &str = "Midas Touch";
pub const HARMONY: &str = "Harmony";

pub const RACIAL_SPELL: &str = "Racial Spell";

// Multiplier constants
pub const PLAT_AWARDED_MULT: u8 = 4;
pub const LAND_BONUS: u8 = 20;

// Array of building names
pub const BUILDING_NAMES: &'static [&'static str] = &[
    "Homes",
    "Alchemies",
    "Farms",
    "Smithies",
    "Masonries",
    "Lumber Yards",
    "Ore Mines",
    "Gryphon Nests",
    "Factories",
    "Guard Towers",
    "Barracks",
    "Shrines",
    "Towers",
    "Temples",
    "Wizard Guilds",
    "Diamond Mines",
    "Schools",
    "Docks",
];

// Maps for explore and rezone lands, using phf_map! for compile-time maps.
// This provides O(1) average time complexity for lookups,
// without the runtime overhead of constructing a HashMap.
pub const EXPLORE_LANDS: phf::Map<&'static str, &'static str> = phf_map! {
    "Plains" => "T",
    "Forest" => "U",
    "Mountains" => "V",
    "Hills" => "W",
    "Swamps" => "X",
    "Caverns" => "Y",
    "Water" => "Z",
};

pub const REZONE_LANDS: phf::Map<&'static str, &'static str> = phf_map! {
    "Plains" => "L",
    "Forest" => "M",
    "Mountains" => "N",
    "Hills" => "O",
    "Swamps" => "P",
    "Caverns" => "Q",
    "Water" => "R",
};

#[derive(Debug, PartialEq, Clone)]
enum CellValue {
    Float(f64),
    Int(i64),
    String(String),
    DateTime(chrono::NaiveDateTime),
    Data(Data),
    None,
}

// Implement Display for CellValue
impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellValue::Float(val) => write!(f, "{:.2}", val),
            CellValue::Int(val) => write!(f, "{}", val),
            CellValue::String(val) => write!(f, "{}", val),
            CellValue::DateTime(val) => write!(f, "{}", val),
            CellValue::Data(val) => write!(f, "{}", val),
            CellValue::None => write!(f, "None"),
        }
    }
}

impl From<f64> for CellValue {
    fn from(val: f64) -> Self {
        CellValue::Float(val)
    }
}

impl From<i64> for CellValue {
    fn from(val: i64) -> Self {
        CellValue::Int(val)
    }
}

impl From<&str> for CellValue {
    fn from(val: &str) -> Self {
        CellValue::String(val.to_string())
    }
}

impl From<String> for CellValue {
    fn from(val: String) -> Self {
        CellValue::String(val)
    }
}

impl From<chrono::NaiveDateTime> for CellValue {
    fn from(val: chrono::NaiveDateTime) -> Self {
        CellValue::DateTime(val)
    }
}

impl From<Data> for CellValue {
    fn from(val: Data) -> Self {
        CellValue::Data(val)
    }
}

impl CellValue {
    fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}

pub struct GameLogGenerator {
    workbook: Xlsx<BufReader<File>>,
    current_hour: usize,
    sim_hour: usize,
}

impl GameLogGenerator {
    /// Creates a new generator and loads the Excel file.
    pub fn new(path: &Path) -> Result<Self, XlsxError> {
        let workbook = open_workbook(path)?;

        Ok(Self {
            workbook,
            current_hour: 0,
            sim_hour: 0,
        })
    }

    /// Sets the current hour, adjusting for the 0-based vs 1-based indexing
    /// and the 3-row header offset in the sim.
    fn set_current_hour(&mut self, hr: usize) {
        self.current_hour = hr;
        self.sim_hour = hr + 3; // Sim rows are offset by 3
    }

    /// Main execution loop.
    pub fn execute(&mut self, specific_hour: Option<usize>) -> Result<String, XlsxError> {
        let mut full_log = String::new();

        if let Some(hr) = specific_hour {
            self.set_current_hour(hr);
            return Ok(self.execute_actions_for_current_hour()?);
        }

        for hr in 1..=LAST_HOUR {
            self.set_current_hour(hr);

            let hour_log = self.execute_actions_for_current_hour()?;
            if !hour_log.is_empty() {
                full_log.push_str(&hour_log);
                full_log.push('\n');
            }
        }

        Ok(full_log)
    }

    /// Executes all action methods for the currently set hour.
    fn execute_actions_for_current_hour(&mut self) -> Result<String, XlsxError> {
        let mut output = String::new();

        let actions = [
            self.tick_action()?,
            self.draft_rate_action()?,
            // c.releaseUnitsAction,
            // c.castMagicSpells,
            // c.unlockTechAction,
            // c.dailtyPlatinumAction,
            // c.tradeResources,
            // c.exploreAction,
            // c.dailyLandAction,
            // c.destroyBuildingsAction,
            // c.rezoneAction,
            // c.constructionAction,
            // c.trainUnitsAction,
            // c.improvementsAction,
        ];

        for action_result in actions.iter().filter(|s| !s.is_empty()) {
            output.push_str(dbg!(action_result));

            if !action_result.ends_with('\n') {
                output.push('\n');
            }
        }

        // Skip empty ticks
        if output.ends_with("======\n") || output.is_empty() {
            return Ok(String::new());
        }

        Ok(output)
    }

    // --- Action Implementations ---

    fn tick_action(&mut self) -> Result<String, XlsxError> {
        // let date_str = self.read_value(OVERVIEW, "B", 15)?;

        let local_time_str = self.read_value_by_hour(IMPS, "BY")?;
        let dom_time_str = self.read_value_by_hour(IMPS, "BZ")?;

        Ok(format!(
            "====== Protection Hour: {} ( Local Time: {} ) ( Domtime: {} ) ======\n",
            self.current_hour, local_time_str, dom_time_str
        ))
    }

    fn draft_rate_action(&mut self) -> Result<String, XlsxError> {
        let mut current_rate = self.read_value_by_hour(MILITARY, "Y")?;
        let previous_rate = self.read_value(MILITARY, "Z", self.sim_hour - 1)?;

        if let CellValue::String(s) = &previous_rate {
            if s == EMPTY_RATE && current_rate.is_none() {
                current_rate = CellValue::Float(0.9);
            }
        }

        if current_rate == previous_rate || current_rate.is_none() {
            return Ok(String::new());
        }

        if let CellValue::Float(f) = current_rate {
            current_rate = CellValue::Int((f * 100.0) as i64);
        }

        Ok(format!("Draftrate changed to {}%\n", current_rate))
    }

    // Read value in row with a current hour as BY{symHour}
    fn read_value_by_hour(&mut self, sheet: &str, column: &str) -> Result<CellValue, XlsxError> {
        Ok(self.read_value(sheet, column, self.sim_hour)?)
    }

    // Read value from cell in format B15
    fn read_value(
        &mut self,
        sheet: &str,
        column: &str,
        row: usize,
    ) -> Result<CellValue, XlsxError> {
        let range = self.workbook.worksheet_range(sheet)?;

        let column_index = column.chars().fold(0, |acc, c| {
            acc * 26 + (c.to_ascii_uppercase() as u8 - b'A' + 1) as usize
        }) - 1;

        let cell_value = range.get((row - 1, column_index));

        match cell_value {
            Some(Data::Float(f)) => Ok(CellValue::from(*f)),
            Some(Data::Int(i)) => Ok(CellValue::from(*i)),
            Some(Data::String(s)) => {
                if s.trim().len() == 0 {
                    Ok(CellValue::None)
                } else {
                    Ok(CellValue::from(s.clone()))
                }
            }
            Some(Data::DateTime(s)) => Ok(CellValue::from(s.as_datetime().unwrap())),
            Some(Data::Empty) => Ok(CellValue::None),
            Some(s) => Ok(CellValue::from(s.clone())),
            None => Ok(CellValue::None),
        }
    }
}
