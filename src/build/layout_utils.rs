use std::{ffi::OsStr, fs, io, path::PathBuf, vec};

pub enum SourceFileType {
    Html,
    _Md,
}

pub struct SourceFile {
    pub filetype: SourceFileType,
    pub path: String,
}

fn available_layouts() -> io::Result<Vec<PathBuf>> {
    let mut layouts = vec![];

    for path in fs::read_dir("./_layouts/")? {
        let path = path?.path();
        if let Some("html") = path.extension().and_then(OsStr::to_str) {
            layouts.push(path.to_owned());
        }
    }
    Ok(layouts)
}

fn names_from_path(paths: Vec<PathBuf>, ext_len: usize) -> Vec<String> {
    let mut layout_list: Vec<String> = vec![];
    let mut path: String;

    for p in paths {
        path = p.display().to_string();
        layout_list.push(String::from(&path[11..(path.len() - ext_len)]));
    }

    layout_list
}

pub fn detect_layout(source_file: SourceFile) -> Option<String> {
    let mut layout: String = "".to_string();
    let contents =
        fs::read_to_string(source_file.path).expect("Something went wrong reading the file");

    for line in contents.lines() {
        match source_file.filetype {
            SourceFileType::Html => {
                if line.contains("<!-- layout:") {
                    layout = String::from(&line[13..(line.len() - 4)]);
                    break;
                }
            }
            SourceFileType::_Md => {}
        }
    }

    let available_layouts = names_from_path(available_layouts().unwrap(), 5);
    println!("{:?}", available_layouts);
    println!("{:?}", layout);
    for l in available_layouts {
        if l == layout {
            return Some(layout);
        }
    }
    return None;
}
