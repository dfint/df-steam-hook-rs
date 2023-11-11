#[cfg(target_os = "windows")]
pub const PATH_SDL2: &'static str = "SDL2.dll";
#[cfg(target_os = "linux")]
pub const PATH_SDL2: &'static str = "libSDL2-2.0.so.0";
#[cfg(target_os = "windows")]
pub const PATH_EXE: &'static str = "./Dwarf Fortress.exe";
#[cfg(target_os = "linux")]
pub const PATH_EXE: &'static str = "./dwarfort";

pub const PATH_ENCODING: &'static str = "./dfint-data/encoding.toml";
pub const PATH_CONFIG: &'static str = "./dfint-data/config.toml";
pub const PATH_OFFSETS: &'static str = "./dfint-data/offsets.toml";
pub const PATH_DICTIONARY: &'static str = "./dfint-data/dictionary.csv";
