use calamine::{Data, DataType, Reader, Xlsx, XlsxError, open_workbook};
use std::{fs::File, io::BufReader, path::Path};
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

pub const SPELLS: &[(&str, &str)] = &[
    (GAIAS_WATCH, "G"),
    (MINING_STRENGTH, "H"),
    (ARES_CALL, "I"),
    (MIDAS_TOUCH, "J"),
    (HARMONY, "K"),
    (RACIAL_SPELL, "L"),
    (RACIAL_SPELL, "M"),
    (RACIAL_SPELL, "N"),
    (RACIAL_SPELL, "O"),
    (RACIAL_SPELL, "P"),
    (RACIAL_SPELL, "Q"),
    (RACIAL_SPELL, "R"),
    (RACIAL_SPELL, "S"),
    (RACIAL_SPELL, "T"),
    (RACIAL_SPELL, "U"),
];

// Array of building names
pub const BUILDING_NAMES: &[&str] = &[
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

pub const DESTROY_BUILDING_COLUMNS: &[&str] = &[
    "BW", "BX", "BY", "BZ", "CA", "CB", "CD", "CE", "CF", "CG", "CH", "CI", "CJ", "CK", "CL", "CM",
    "CN", "CO",
];

pub const CREATE_BUILDING_COLUMNS: &[&str] = &[
    "O", "P", "Q", "R", "S", "T", "V", "W", "X", "Y", "Z", "AA", "AB", "AC", "AD", "AE", "AF", "AG",
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
            self.release_units_action()?,
            self.cast_magic_spells_action()?,
            self.unlock_tech_action()?,
            self.daily_platinum_action()?,
            self.trade_resources_action()?,
            self.explore_action()?,
            self.daily_land_action()?,
            self.destroy_buildings_action()?,
            self.rezone_action()?,
            self.construction_action()?,
            self.train_units_action()?,
            self.improvements_action()?,
        ];

        for action_result in actions.iter().filter(|s| !s.is_empty()) {
            // DEBUG code: remove after implementing all methods
            if action_result != "not implemented yet" {
                output.push_str(dbg!(action_result));

                if !action_result.ends_with('\n') {
                    output.push('\n');
                }
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
        let local_time_col = self.column_str_int("BY");

        let local_time_str = self
            .read_value_by_hour(IMPS, local_time_col)?
            .as_datetime()
            .unwrap()
            .to_string();
        let dom_time_str = self
            .read_value_by_hour(IMPS, local_time_col + 1)?
            .as_datetime()
            .unwrap()
            .to_string();

        Ok(format!(
            "====== Protection Hour: {} ( Local Time: {} ) ( Domtime: {} ) ======\n",
            self.current_hour, local_time_str, dom_time_str
        ))
    }

    fn draft_rate_action(&mut self) -> Result<String, XlsxError> {
        let current_rate_col = self.column_str_int("Y");

        let mut current_rate = self.read_value_by_hour(MILITARY, current_rate_col)?;
        let previous_rate = self.read_value(MILITARY, current_rate_col + 1, self.sim_hour - 1)?;

        if previous_rate == EMPTY_RATE && current_rate.is_empty() {
            current_rate = Data::Float(0.9);
        }

        if current_rate == previous_rate || current_rate.is_empty() {
            return Ok(String::new());
        }

        let rate: i64 = match current_rate {
            Data::Float(f) => (f * 100.0) as i64,
            _ => return Ok(String::new()),
        };

        Ok(format!("Draftrate changed to {}%\n", rate))
    }

    fn release_units_action(&mut self) -> Result<String, XlsxError> {
        // Read unit names and unit counts
        let ay_col = self.column_str_int("AY");

        let mut sb = String::from("You successfully released ");

        let mut added_items = 0;

        for col in ay_col..ay_col + 8 {
            let name = self.read_value(MILITARY, col, 2)?;
            let value = self.read_value_by_hour(MILITARY, col)?;

            if value.is_empty() || value == 0 {
                continue;
            }

            if added_items > 0 {
                sb.push_str(", ")
            }

            sb.push_str(&format!("{} {}", value, name));
            added_items += 1;
        }

        if added_items == 0 {
            sb = String::new();
        } else {
            sb.push_str(".\n");
        }

        // // Read draftees count from AX column
        let draftees = self.read_value_by_hour(MILITARY, ay_col - 1)?;

        if draftees.is_empty() || draftees == 0 {
            return Ok(sb);
        }

        sb.push_str(&format!(
            "You successfully released {} draftees into the peasantry.\n",
            draftees
        ));

        Ok(sb)
    }

    fn cast_magic_spells_action(&mut self) -> Result<String, XlsxError> {
        let mut sb = String::new();
        let col_y = self.column_str_int("Y");

        let mana = self.read_value_by_hour(MAGIC, col_y)?;

        for (spell_name, magic_col) in SPELLS {
            let magic_data = self.read_value_by_hour(MAGIC, self.column_str_int(magic_col))?;

            if !magic_data.is_empty() {
                sb.push_str(&format!(
                    "Your wizards successfully cast {} at a cost of {} mana.\n",
                    spell_name, mana
                ));
            }
        }

        Ok(sb)
    }

    fn unlock_tech_action(&mut self) -> Result<String, XlsxError> {
        let tech_unlocked = self.read_value_by_hour(TECHS, self.column_str_int("K"))?;

        if tech_unlocked.is_empty() || tech_unlocked == 0.0 {
            return Ok(String::new());
        }

        let tech_name = self.read_value_by_hour(TECHS, self.column_str_int("CA"))?;

        Ok(format!("You have unlocked {}.\n", tech_name))
    }

    fn daily_platinum_action(&mut self) -> Result<String, XlsxError> {
        let plat_checked = self.read_value_by_hour(PRODUCTION, self.column_str_int("C"))?;

        if plat_checked.is_empty() || plat_checked == 0 {
            return Ok(String::new());
        }

        let population_value = self.read_value_by_hour(POPULATION, self.column_str_int("C"))?;

        let platinum_awarded = population_value.as_i64().unwrap() * PLAT_AWARDED_MULT as i64;

        Ok(format!(
            "You have been awarded with {} platinum.\n",
            platinum_awarded
        ))
    }

    fn trade_resources_action(&mut self) -> Result<String, XlsxError> {
        let plat_column = self.column_str_int("BC");

        let plat_value = self.read_value_by_hour(PRODUCTION, plat_column)?; //BC
        let lumber_value = self.read_value_by_hour(PRODUCTION, plat_column + 1)?; //BD
        let ore_value = self.read_value_by_hour(PRODUCTION, plat_column + 2)?; //BE
        let gems_value = self.read_value_by_hour(PRODUCTION, plat_column + 3)?; //BF

        // No exchange happened
        if plat_value.is_empty()
            && lumber_value.is_empty()
            && ore_value.is_empty()
            && gems_value.is_empty()
        {
            return Ok(String::new());
        }

        let mut sb = String::new();
        let mut traded_items: Vec<String> = Vec::new();
        let mut received_items: Vec<String> = Vec::new();

        let mut add_item = |item: &str, amount: Data| {
            if let Some(amount_value) = amount.as_i64() {
                if amount_value < 0 {
                    traded_items.push(format!("{} {}", -amount_value, item));
                } else if amount_value > 0 {
                    received_items.push(format!("{} {}", amount_value, item));
                }
            }
        };

        add_item("platinum", plat_value);
        add_item("lumber", lumber_value);
        add_item("ore", ore_value);
        add_item("gems", gems_value);

        if traded_items.len() > 0 {
            sb.push_str(&format!(
                "{} have been traded for ",
                traded_items.join(" and ")
            ));
        }

        if received_items.len() > 0 {
            sb.push_str(&format!("{}.\n", received_items.join(" and ")));
        }

        Ok(sb)
    }

    fn explore_action(&mut self) -> Result<String, XlsxError> {
        let mut sb = String::from("Exploration for ");
        let mut added_items = 0u8;

        for (land_type, col) in EXPLORE_LANDS.into_iter() {
            let value = self.read_value_by_hour(EXPLORE, self.column_str_int(col))?;

            if value.is_empty() || value == 0 {
                continue;
            }

            if added_items > 0 {
                sb.push_str(", ");
            }

            sb.push_str(&format!("{} {}", value, land_type));
            added_items += 1;
        }

        if added_items == 0 {
            return Ok(String::new());
        }

        let plat_cost = self.read_value_by_hour(EXPLORE, self.column_str_int("AH"))?;
        let draftee_cost = self.read_value_by_hour(EXPLORE, self.column_str_int("AI"))?;

        sb.push_str(&format!(
            " begun at a cost of {} platinum and {} draftees.\n",
            plat_cost, draftee_cost
        ));

        Ok(sb)
    }

    fn daily_land_action(&mut self) -> Result<String, XlsxError> {
        let land_bonus = self.read_value_by_hour(EXPLORE, self.column_str_int("S"))?;

        if land_bonus.is_empty() || land_bonus == 0 {
            return Ok(String::new());
        }

        let land_type = self.read_value(OVERVIEW, self.column_str_int("B"), 70)?;

        Ok(format!(
            "You have been awarded with {} {}.\n",
            LAND_BONUS, land_type
        ))
    }

    fn destroy_buildings_action(&mut self) -> Result<String, XlsxError> {
        let mut sb = String::from("Destruction of ");
        let mut added_items = 0u8;

        for (index, col) in DESTROY_BUILDING_COLUMNS.iter().enumerate() {
            let name = BUILDING_NAMES[index];
            let value = self.read_value_by_hour(CONSTRUCTION, self.column_str_int(col))?;

            if value.is_empty() || value == 0 {
                continue;
            }

            if added_items > 0 {
                sb.push_str(", ");
            }

            sb.push_str(&format!("{} {}", value, name));
            added_items += 1;
        }

        if added_items == 0 {
            return Ok(String::new());
        }

        sb.push_str(&format!(" is complete.\n"));

        Ok(sb)
    }

    fn rezone_action(&mut self) -> Result<String, XlsxError> {
        let plat_cost = self.read_value_by_hour(REZONE, self.column_str_int("Y"))?;

        if plat_cost.is_empty() || plat_cost == 0 {
            return Ok(String::new());
        }

        let mut sb = format!(
            "Rezoning begun at a cost of {} platinum. The changes in land are as following: ",
            plat_cost
        );

        let mut added_items = 0u8;

        for (land_type, col) in REZONE_LANDS.into_iter() {
            let value = self.read_value_by_hour(REZONE, self.column_str_int(col))?;

            if value.is_empty() || value == 0 {
                continue;
            }

            if added_items > 0 {
                sb.push_str(", ");
            }

            sb.push_str(&format!("{} {}", value, land_type));
            added_items += 1;
        }

        sb.push_str(&format!(".\n"));

        Ok(sb)
    }

    fn construction_action(&mut self) -> Result<String, XlsxError> {
        let mut sb = String::from("Construction of ");
        let mut added_items = 0u8;

        for (index, col) in CREATE_BUILDING_COLUMNS.iter().enumerate() {
            let name = BUILDING_NAMES[index];
            let value = self.read_value_by_hour(CONSTRUCTION, self.column_str_int(col))?;

            if value.is_empty() || value == 0 {
                continue;
            }

            if added_items > 0 {
                sb.push_str(", ");
            }

            sb.push_str(&format!("{} {}", value, name));
            added_items += 1;
        }

        if added_items == 0 {
            return Ok(String::new());
        }

        let plat_cost = self.read_value_by_hour(CONSTRUCTION, self.column_str_int("AQ"))?;
        let lumber_cost = self.read_value_by_hour(CONSTRUCTION, self.column_str_int("AR"))?;

        sb.push_str(&format!(
            " started at a cost of {} platinum and {} lumber.\n",
            plat_cost, lumber_cost
        ));

        Ok(sb)
    }

    fn train_units_action(&mut self) -> Result<String, XlsxError> {
        let mut sb = String::from("Destruction of ");
        let mut added_items = 0u8;

        for (index, col) in DESTROY_BUILDING_COLUMNS.iter().enumerate() {
            let name = BUILDING_NAMES[index];
            let value = self.read_value_by_hour(CONSTRUCTION, self.column_str_int(col))?;

            if value.is_empty() || value == 0 {
                continue;
            }

            if added_items > 0 {
                sb.push_str(", ");
            }

            sb.push_str(&format!("{} {}", value, name));
            added_items += 1;
        }

        if added_items == 0 {
            return Ok(String::new());
        }

        sb.push_str(&format!(" is complete.\n"));

        Ok(String::new())
    }

    fn improvements_action(&mut self) -> Result<String, XlsxError> {
        let mut sb = String::from("Destruction of ");
        let mut added_items = 0u8;

        for (index, col) in DESTROY_BUILDING_COLUMNS.iter().enumerate() {
            let name = BUILDING_NAMES[index];
            let value = self.read_value_by_hour(CONSTRUCTION, self.column_str_int(col))?;

            if value.is_empty() || value == 0 {
                continue;
            }

            if added_items > 0 {
                sb.push_str(", ");
            }

            sb.push_str(&format!("{} {}", value, name));
            added_items += 1;
        }

        if added_items == 0 {
            return Ok(String::new());
        }

        sb.push_str(&format!(" is complete.\n"));

        Ok(String::new())
    }

    // Read value in row with a current hour as BY{symHour}
    fn read_value_by_hour(&mut self, sheet: &str, column: usize) -> Result<Data, XlsxError> {
        Ok(self.read_value(sheet, column, self.sim_hour)?)
    }

    // Read value from cell in format B15
    fn read_value(&mut self, sheet: &str, column: usize, row: usize) -> Result<Data, XlsxError> {
        let range = self.workbook.worksheet_range(sheet)?;
        let cell_value = range.get((row - 1, column - 1));

        match cell_value {
            Some(value) => Ok(value.clone()),
            None => Ok(Data::Empty),
        }
    }

    fn column_str_int(&self, column: &str) -> usize {
        column.chars().fold(0, |acc, c| {
            acc * 26 + (c.to_ascii_uppercase() as u8 - b'A' + 1) as usize
        })
    }
}
