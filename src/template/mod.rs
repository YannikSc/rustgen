pub mod config_extractor;
pub mod writer;
pub mod helpers;

/// Prepares the template file
/// - Splits the yaml head and template content
#[derive(Debug, Clone)]
pub struct PreProcessor {
    template: String,
}

/// Writes the rendered template to the target file (creates a new file/appends to an existing one)
pub struct Writer {
    header: TemplateHeader,
    rendered_template: String,
}

/// Structural use. Shows the available options for the header
///
/// # Example for new file:
/// ```yaml
/// ---
/// path: src/project/controllers/{{name}}.lua
/// action: CreateFile
/// ---
/// ```
///
/// # Example for adding content to a file
/// ```yaml
/// ---
/// path: src/project/controllers.lua
/// action:
///     Append:
///         After:  "-- Marker for controller registration"
/// ---
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateHeader {
    /// The path where the final file should be put/which file should be replaced
    pub path: String,

    /// The action that has te be performed, default=CreateFile
    #[serde(default)]
    pub action: WriteAction,
}

/// Structural use. Available actions for modifying the source code
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WriteAction {
    /// Creates a new file
    CreateFile,

    /// Appends to an existing file
    /// Can be at different locations/markers in a file
    Append(ExtendLocation),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExtendLocation {
    /// Appends to the beginning of the file, before all the other code
    BeginOfFile,

    /// Appends to the end of the file, after all the other code
    EndOfFile,

    /// Appends after a given marker (eg. a code comment or a function start)
    After(String),

    /// Appends before a given marker (.eg a class header)
    Before(String),
}

/// A helper which replaces with regex
///
/// # Example:
/// ```hbs
/// {{regex_replace from_string "W(or)d pattern" "Captured group: ${1}"}});
/// ```
pub struct RegexReplaceHelper;

/// A helper which outputs a default value if a variable is not containing data
/// Empty data is:
/// - an empty string ("")
/// - a 0 (number)
/// - false (boolean)
///
/// # Example
///
/// ```hbs
/// {{default my_optional_string "A default value"}}
/// ```
///
pub struct DefaultHelper;

/// A helper for setting variables
///
/// # Example
///
/// ```hbs
/// {{set "variable_name" "Value for the variable" }}
///
/// {{! Other options including a mix of variables and text or the }}
/// {{! name for the variable, by the content of a variable }}
///
/// {{set name_from_variable (concat from_path "/appended/to/path")}}
/// ```
pub struct SetHelper;

/// A helper for concatenating strings
///
/// # Example
///
/// ```hbs
/// {{concat base_path "/sub/directories/" file_name file_extension }}
/// ```
pub struct ConcatHelper;

/// A helper for outputting a formatted date string
///
/// # Example
///
/// ```hbs
/// Today's date: {{time "%Y-%m-%d"}}
/// Also: {{time "%F"}}
/// ```
///
/// # Formatting
///
/// The formatting is done with the [chrono](https://crates.io/crates/chrono) crate.
/// The available syntax is described [here](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html)
///
pub struct TimeHelper;
