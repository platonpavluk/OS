use crate::vga_buffer;


pub struct File {
    pub name: &'static str,
    pub size: u64,
}

pub struct Directory {
    pub name: &'static str,
    pub files: [Option<File>; MAX_FILES],
}

pub struct FileSystem {
    pub root: Directory,
}

const MAX_FILES: usize = 10; // Максимальна кількість файлів у каталозі

impl FileSystem {
    /// Створення нової файлової системи з порожнім кореневим каталогом
    pub fn new() -> Self {
        FileSystem {
            root: Directory {
                name: "home",
                files: [const { None }; MAX_FILES],
            },
        }
    }

    /// Додати файл у кореневий каталог
    pub fn create_file(&mut self, name: &'static str) -> Result<(), &'static str> {
        let size = name.len() as u64 * 10; // Наприклад, розмір залежить від довжини імені
        for file in self.root.files.iter_mut() {
            if file.is_none() {
                *file = Some(File { name, size });
                return Ok(());
            }
        }
        Err("Error: Directory is full. Cannot create more files.")
    }

    /// Вивести всі файли в кореневому каталозі
    pub fn list_files(&self) {
        for file in self.root.files.iter() {
            if let Some(f) = file {
                vga_buffer::println("File: ");
                vga_buffer::println(f.name);
            }
        }
    }
}
