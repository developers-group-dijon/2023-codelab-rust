use std::collections::BTreeMap;

use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS,
    presets::{NOTHING, UTF8_FULL},
    Attribute, Cell, CellAlignment, Row, Table,
};
use console::{style, Emoji, Term};
use indicatif::ProgressBar;
use inquire::{Confirm, DateSelect, MultiSelect, Password, PasswordDisplayMode, Select, Text};

/// separator for title outputs
const HEAVY_SEPARATOR: &str = "==================================";

/// separator for section output
const LIGHT_SEPARATOR: &str = "----------------------------------";

/// meta separator for lists
pub const LIST_SEPARATOR: &str = "$__SEPARATOR__$";

/// real separator for lists
const LIST_REAL_SEPARATOR: &str = "----------";

/// handler struct for standard outputs.
pub struct ConsoleIO {
    stdout: Term,
    stderr: Term,
}

#[allow(clippy::new_without_default)]
impl ConsoleIO {
    /// creates an instance of ConsoleIO
    pub fn new() -> Self {
        Self {
            stdout: Term::stdout(),
            stderr: Term::stderr(),
        }
    }

    /// writes to STDOUT without line return
    pub fn write(&self, text: &str) {
        self.stdout.write_str(text).unwrap();
    }

    /// writes to STDOUT with line return
    pub fn writeln(&self, text: &str) {
        self.stdout.write_line(text).unwrap();
    }

    /// writes to STDOUT with line return and bold font
    pub fn writeln_bold(&self, text: &str) {
        self.stdout
            .write_line(&format!("{}", style(text).white().bold()))
            .unwrap();
    }

    /// creates a new empty line in STDOUT
    pub fn new_line(&self) {
        self.stdout.write_line("").unwrap();
    }

    /// creates a title formatted output
    pub fn title(&self, title: &str) {
        self.new_line();
        self.stdout
            .write_line(&format!("{}", style(title).yellow().bold()))
            .unwrap();
        self.stdout
            .write_line(&format!("{}", style(HEAVY_SEPARATOR).yellow().bold()))
            .unwrap();
    }

    /// creates a section formatted output
    pub fn section(&self, title: &str) {
        self.new_line();
        self.stdout
            .write_line(&format!("{}", style(title).cyan().bold()))
            .unwrap();
        self.stdout
            .write_line(&format!("{}", style(LIGHT_SEPARATOR).cyan().bold()))
            .unwrap();
    }

    /// creates a comment formatted output
    pub fn comment(&self, comment: &str) {
        self.stdout
            .write_line(&format!("// {}", style(comment).white().dim().bold()))
            .unwrap();
    }

    /// creates a "step".
    ///
    /// Example:
    /// ```text
    /// [1/4] doing things...
    /// ```
    pub fn step(&self, nb: usize, max: usize, message: &str) {
        let step_str = format!("[{nb}/{max}]");

        self.stdout
            .write_line(&format!(
                "{} {}",
                style(step_str).white().dim().bold(),
                style(message).white().bold()
            ))
            .unwrap();
    }

    /// creates a success output
    pub fn success(&self, text: &str) {
        let success_symb_str = format!("[{} SUCCESS]", Emoji("✅", "✓"));

        self.stdout
            .write_line(&format!(
                "{} {}",
                style(success_symb_str).green().bold(),
                style(text).white().bold()
            ))
            .unwrap();
    }

    /// creates a error output
    pub fn error(&self, text: &str) {
        let error_symb_str = format!("[{} ERROR]", Emoji("❌", "X"));

        self.stderr
            .write_line(&format!(
                "{} {}",
                style(error_symb_str).red().bold(),
                style(text).white().bold()
            ))
            .unwrap();
    }

    /// creates a warning output
    pub fn warning(&self, text: &str) {
        let warning_symb_str = format!("[{}  WARNING]", Emoji("⚠️", "!"));

        self.stdout
            .write_line(&format!(
                "{} {}",
                style(warning_symb_str).yellow().bold(),
                style(text).white().bold()
            ))
            .unwrap();
    }

    /// creates a note output
    pub fn note(&self, text: &str) {
        let note_symb_str = format!("[{} NOTE]", Emoji("📘", "🕮"));

        self.stdout
            .write_line(&format!(
                "{} {}",
                style(note_symb_str).cyan().bold(),
                style(text).white().bold()
            ))
            .unwrap();
    }

    /// creates a info output
    pub fn info(&self, text: &str) {
        let note_symb_str = format!("[{} INFO]", Emoji("📝", "▤"));

        self.stdout
            .write_line(&format!(
                "{} {}",
                style(note_symb_str).magenta().bold(),
                style(text).white().bold()
            ))
            .unwrap();
    }

    /// creates a formatted (e.g. unordered) listing
    pub fn listing(&self, list: Vec<&str>) {
        list.iter()
            .map(|item| self.writeln(&format!("• {item}")))
            .for_each(drop);
    }

    /// creates a data table with headers
    pub fn table(&self, headers: Vec<&str>, data: Vec<Vec<&str>>) {
        let mut table = Table::new();

        let header_bold = headers
            .iter()
            .map(|e| {
                Cell::new(e)
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center)
            })
            .collect::<Vec<Cell>>();

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(header_bold)
            .add_rows(data);

        self.writeln(&format!("{table}"));
    }

    /// creates a data table with headers with Strings
    pub fn string_table(&self, headers: Vec<String>, data: Vec<Vec<String>>) {
        let mut table = Table::new();

        let header_bold = headers
            .iter()
            .map(|e| {
                Cell::new(e)
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center)
            })
            .collect::<Vec<Cell>>();

        let rows = data
            .iter()
            .map(|data| Row::from(data))
            .collect::<Vec<Row>>();

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(header_bold)
            .add_rows(rows);

        self.writeln(&format!("{table}"));
    }

    /// creates a key-value pair display.
    ///
    /// You can use LIST_SEPARATOR to split your listings
    pub fn key_value_pair(&self, values: Vec<(&str, String)>) {
        let mut table = Table::new();
        table.load_preset(NOTHING);

        for (key, value) in values.iter() {
            if value == LIST_SEPARATOR {
                table.add_row(vec![
                    Cell::new(LIST_REAL_SEPARATOR).add_attribute(Attribute::Bold),
                    Cell::new(""),
                ]);
            } else {
                table.add_row(vec![
                    Cell::new(key).add_attribute(Attribute::Bold),
                    Cell::new(value),
                ]);
            }
        }

        self.writeln(&format!("{table}"));
    }

    /// creates a definition list
    pub fn definition_list(&self, values: BTreeMap<String, String>) {
        let mut table = Table::new();
        table.load_preset(NOTHING);

        let mut i = 0;
        let len = values.len();

        for (key, value) in values.iter() {
            table.add_row(vec![
                Cell::new(key).add_attribute(Attribute::Bold),
                Cell::new(value),
            ]);

            i += 1;

            if i != len {
                table.add_row(vec!["", ""]);
            }
        }

        self.writeln(&format!("{table}"));
    }

    /// shorthand method to create a text question for the user.
    ///
    /// **Note:** You must use the `prompt()` method to actually display it to the user.
    ///
    /// See https://github.com/mikaelmello/inquire
    pub fn input_text<'a>(&'a self, question: &'a str) -> Text {
        Text::new(question)
    }

    /// shorthand method to create a date question for the user.
    ///
    /// **Note:** You must use the `prompt()` method to actually display it to the user.
    ///
    /// See https://github.com/mikaelmello/inquire
    pub fn input_date<'a>(&'a self, question: &'a str) -> DateSelect {
        DateSelect::new(question)
    }

    /// shorthand method to create a select (e.g. choice) question for the user.
    ///
    /// **Note:** You must use the `prompt()` method to actually display it to the user.
    ///
    /// See https://github.com/mikaelmello/inquire
    pub fn input_select<'a>(&'a self, question: &'a str, choices: Vec<&'a str>) -> Select<&'a str> {
        Select::new(question, choices)
    }

    /// shorthand method to create a multi-select (e.g. multiple choice) question for the user.
    ///
    /// **Note:** You must use the `prompt()` method to actually display it to the user.
    ///
    /// See https://github.com/mikaelmello/inquire
    pub fn input_multi_select<'a>(
        &'a self,
        question: &'a str,
        choices: Vec<&'a str>,
    ) -> MultiSelect<&'a str> {
        MultiSelect::new(question, choices)
    }

    /// shorthand method to create a confirm (e.g. yes/no) question for the user.
    ///
    /// **Note:** You must use the `prompt()` method to actually display it to the user.
    ///
    /// See https://github.com/mikaelmello/inquire
    pub fn input_confirm<'a>(&'a self, question: &'a str) -> Confirm {
        Confirm::new(question)
    }

    /// shorthand method to create a password (e.g. hidden) question for the user.
    ///
    /// **Note:** You must use the `prompt()` method to actually display it to the user.
    ///
    /// See https://github.com/mikaelmello/inquire
    pub fn input_password<'a>(&'a self, question: &'a str) -> Password {
        Password::new(question)
    }

    /// shorthand method to directly ask a question to the user.
    pub fn ask_question(&self, question: &str) -> String {
        let mut response = self.input_text(question).prompt();

        while response.is_err() {
            self.error("An error occured while data input, please try again");

            response = Text::new(question).prompt();
        }

        response.unwrap()
    }

    /// shorthand method to directly ask a question to the user with a default response.
    pub fn ask_question_default(&self, question: &str, default: &str) -> String {
        let mut response = self.input_text(question).with_default(default).prompt();

        while response.is_err() {
            self.error("An error occured while data input, please try again");

            response = Text::new(question).prompt();
        }

        response.unwrap()
    }

    /// shorthand method to directly ask a confirmation to the user.
    pub fn ask_confirm(&self, question: &str) -> bool {
        let mut response = self.input_confirm(question).with_default(true).prompt();

        while response.is_err() {
            self.error("An error occured while data input, please try again");

            response = self.input_confirm(question).prompt();
        }

        response.unwrap()
    }

    /// shorthand method to directly ask a password to the user.
    pub fn ask_password(&self, question: &str) -> String {
        let mut response = self
            .input_password(question)
            .with_display_mode(PasswordDisplayMode::Masked)
            .prompt();

        while response.is_err() {
            self.error("An error occured while data input, please try again");

            response = self
                .input_password(question)
                .with_display_mode(PasswordDisplayMode::Masked)
                .prompt();
        }

        response.unwrap()
    }

    /// creates a progress bar for the user.
    ///
    /// - use `inc(u64)` to increment the bar.
    /// - use `finish()` to finish the progress.
    ///
    /// See https://docs.rs/indicatif/latest/indicatif/struct.ProgressBar.html
    pub fn create_progress_bar(&self, max: u64) -> ProgressBar {
        ProgressBar::new(max)
    }

    /// creates a spinner for the user.
    ///
    /// - use `tick()` to make the spinner progress.
    ///
    /// See https://docs.rs/indicatif/latest/indicatif/struct.ProgressBar.html
    pub fn create_spinner(&self) -> ProgressBar {
        ProgressBar::new_spinner()
    }
}
