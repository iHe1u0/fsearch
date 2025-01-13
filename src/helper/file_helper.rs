use std::fs;
use std::path::Path;

pub fn get_file_list(path: &str) -> Option<Vec<String>> {
    let folder_path = Path::new(path);

    // 如果不是目录，返回 None
    if !folder_path.is_dir() {
        return if folder_path.exists() {
            Some(vec![path.to_string()])
        } else {
            None
        };
    }

    let mut file_list = Vec::new();

    // 遍历目录中的文件
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            file_list.push(path.display().to_string());

            // 如果是子目录，递归调用
            if path.is_dir() {
                if let Some(sub_files) = get_file_list(path.to_str().unwrap()) {
                    file_list.extend(sub_files);
                }
            }
        }
    }

    // 如果文件列表为空，返回 None
    if file_list.is_empty() {
        None
    } else {
        Some(file_list)
    }
}

pub fn find_file(files: Vec<String>, filename: &str, exact: Option<bool>) -> Option<Vec<String>> {
    let is_exact = exact.unwrap_or(false);
    let mut paths = Vec::new();
    if is_exact {
        for file in files {
            if &file == filename {
                paths.push(file);
            }
        }
    } else {
        for file in files {
            if file.contains(filename) {
                paths.push(file);
            }
        }
    }
    Some(paths)
}
