// filesystem.rs

// Структура для каталогу
pub struct Directory {
    pub name: &'static str,
}

// Структура для файлової системи
pub struct FileSystem {
    pub root: Directory,
}

impl FileSystem {
    // Створення нової файлової системи з каталогом "home"
    pub fn new() -> Self {
        FileSystem {
            root: Directory { name: "home" },
        }
    }
}
