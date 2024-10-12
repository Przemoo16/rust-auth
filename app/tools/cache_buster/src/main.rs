use sha2::{Digest, Sha256};
use std::{
    env::args,
    fs::{read_dir, read_to_string, rename, File, OpenOptions},
    io::{Result, Write},
    path::PathBuf,
};

const SOURCE_MAP_FILE_NAME: &str = "source_map.json";
const SOURCE_MAP_DELIMETER: &str = ";";
const HASH_LENGTH: usize = 10;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Usage: cache-buster <directory>");
        return;
    }
    let dir = PathBuf::from(&args[1]);
    if !dir.is_dir() {
        panic!("The provided path is not a directory");
    }
    let source_map_path = dir.join(SOURCE_MAP_FILE_NAME);
    File::create(&source_map_path).unwrap_or_else(|_| {
        panic!(
            "Failed to create a source map file at {}",
            source_map_path.display()
        )
    });
    let mut source_map = OpenOptions::new()
        .append(true)
        .open(&source_map_path)
        .unwrap_or_else(|_| {
            panic!(
                "Failed to open a source map file at {}",
                source_map_path.display(),
            )
        });
    for entry in DirWalker::new(dir.clone()) {
        let path = entry.expect("Failed to read a path");
        if path.eq(&source_map_path) {
            continue;
        }
        let content = read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read the {} file", path.display()));
        let hash = calculate_hash(&content, HASH_LENGTH);
        let path_with_hash = append_hash_to_path(&path, &hash);
        rename(&path, &path_with_hash).unwrap_or_else(|_| {
            panic!(
                "Failed to rename the {} file to {}",
                path.display(),
                path_with_hash.display()
            )
        });
        let stripped_path = path.strip_prefix(&dir).unwrap_or_else(|_| {
            panic!(
                "Failed to strip prefix {} from {}",
                dir.display(),
                path.display()
            )
        });
        let stripped_path_with_hash = path_with_hash.strip_prefix(&dir).unwrap_or_else(|_| {
            panic!(
                "Failed to strip prefix {} from {}",
                dir.display(),
                path_with_hash.display()
            )
        });
        writeln!(
            source_map,
            "{}{}{}",
            stripped_path.display(),
            SOURCE_MAP_DELIMETER,
            stripped_path_with_hash.display()
        )
        .unwrap_or_else(|_| {
            panic!(
                "Failed to add entry to a source map at {}",
                source_map_path.display()
            )
        });
        println!("{} -> {}", path.display(), path_with_hash.display());
    }
}

struct DirWalker {
    stack: Vec<PathBuf>,
}

impl DirWalker {
    fn new(root: PathBuf) -> Self {
        Self { stack: vec![root] }
    }
}

impl Iterator for DirWalker {
    type Item = Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(path) = self.stack.pop() {
            if path.is_file() {
                return Some(Ok(path));
            }
            match read_dir(&path) {
                Err(e) => return Some(Err(e)),
                Ok(iter) => {
                    for entry in iter {
                        match entry {
                            Err(e) => return Some(Err(e)),
                            Ok(sub_path) => self.stack.push(sub_path.path()),
                        }
                    }
                }
            }
        }
        None
    }
}

fn calculate_hash(content: &str, length: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    let hash_part = &format!("{:x}", result)[..length];
    hash_part.to_owned()
}

fn append_hash_to_path(path: &PathBuf, hash: &str) -> PathBuf {
    let mut file_name = hash.to_owned();

    if let Some(file_stem) = path.file_stem().and_then(|value| value.to_str()) {
        file_name = format!("{}.{}", file_stem, hash);
    }
    if let Some(file_extension) = path.extension().and_then(|value| value.to_str()) {
        file_name = format!("{}.{}", file_name, file_extension);
    }

    return path.with_file_name(file_name);
}

#[cfg(test)]
mod tests {
    use super::append_hash_to_path;
    use std::path::PathBuf;

    #[test]
    fn properly_append_hash_to_path() {
        let hash = "123";
        let tests = vec![
            ("dist/file.js", "dist/file.123.js"),
            ("file.js", "file.123.js"),
            ("file", "file.123"),
            ("", "123"),
        ];

        for (input, expected) in tests {
            assert_eq!(
                append_hash_to_path(&PathBuf::from(input), &hash),
                PathBuf::from(expected)
            );
        }
    }
}
