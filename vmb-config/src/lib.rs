use std::path::Path;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
/// Errors that can be returned by `parse`.
pub enum Error<E> {
    /// Returned if the file path that has been passed to `parse` does not point to a file.
    FileNotFound,
    /// Returned if any IO error happens while operating on the file that has been passed to `parse`.
    Io(std::io::Error),
    /// Server as a container for the error that the TryFrom implementation of the CONFIG type that
    /// `parse` is trying to create might return.
    ConversionError(E),
    /// Returned if the file that has been passed to `parse` contains an invalid line, the line is
    /// contained in the error.
    FormatError(String)
}

/// Parses a file at `file_path` according to http://vmb.sourceforge.net/configuration.html
pub fn parse<CONFIG, E>(file_path: String, device_name: String) -> Result<CONFIG, Error<E>> where CONFIG: TryFrom<HashMap<String, String>, Error = E>{
    let path = Path::new(&file_path);
    if !path.is_file() {
        return Err(Error::FileNotFound);
    }

    // The value for #FILE#
    let filename_variable: &str = path.file_name().unwrap().to_str().unwrap();
    // The value for #PATH#
    let path_variable: &str = &path.canonicalize().map_err(|e| Error::Io(e))?.into_os_string().into_string().unwrap();

    let mut file = File::open(path).map_err(|e| Error::Io(e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| Error::Io(e))?;

    let mut skip = false;
    let mut variables = HashMap::new();

    for line in contents.lines() {
        if !skip {
            // If we see a condition, check whether it contains our device name or not.
            if line.starts_with("#if ") {
                let condition = &line[4..];
                skip = condition != device_name;
            }
            // Ignore comments.
            else if line.starts_with("#") {
                continue;
            }
            // Ignore empty lines or ones with leading whitespaces
            // The unwrap is fine since || should short circuit on the left if
            // there is no first character.
            else if line.len() == 0 || line.chars().nth(0).unwrap() == ' ' {
                continue;
            }
            // Must be a variable.
            else {
                let line = line.replace("#FILE#", filename_variable).replace("#PATH#", &path_variable);
                let split = line.find(" ").ok_or_else(|| Error::FormatError(line.clone()))?;
                let (key, value) = line.split_at(split);
                let value = &value[1..];
                variables.insert(key.to_string(), value.to_string());
            }
        }
        else {
            if line == "#endif" {
                skip = false;
            }
        }
    }

    CONFIG::try_from(variables).map_err(|e|  Error::ConversionError(e))
}
