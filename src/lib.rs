use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

/// `MyFile` to store path along with file
#[derive(Debug)]
pub struct MyFile {
    path: PathBuf,
    file: File,
}

pub fn write_tmp_files<R: BufRead>(reader: &mut R, tmp_path: &Path) -> Vec<MyFile> {
    // Push unique file names
    let mut set = HashSet::new();

    let mut files = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let first_char = l.chars().next();

        // Ignore if blank
        let Some(first_char) = first_char else { continue };

        // Some chars are invalid names as a file. We stick them into '-' as a fallback
        let first_char = if first_char.is_alphanumeric() {
            first_char
        } else {
            '-'
        };

        #[cfg(debug_assertions)]
        println!("{first_char}\t{l}");

        let path_str = format!("{}{}", first_char, ".txt");
        let file_path = tmp_path.join(&path_str);
        let tmp_file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(&file_path)
            .unwrap();

        let mut writer = BufWriter::new(tmp_file);
        writer.write_all(l.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
        writer.flush().unwrap();

        set.insert(file_path);
    }

    for path in set {
        files.push(MyFile {
            file: File::open(&path).unwrap(),
            path,
        });
    }

    files
}

pub fn sort_files<W: Write>(mut files: Vec<MyFile>, output_writer: &mut W) {
    // sort files by file name
    files.sort_by(|a, b| a.path.cmp(&b.path));

    for my_file in files {
        let mut reader = BufReader::new(my_file.file);

        let mut buf = String::new();
        reader
            .read_to_string(&mut buf)
            .expect("Expected tmpfile to have content");

        let mut res = buf.lines().collect::<Vec<_>>();
        res.sort_unstable();
        let res = res.join("\n");

        output_writer.write_all(res.as_bytes()).unwrap();
        output_writer.write_all(b"\n").unwrap();
    }

    output_writer.flush().unwrap();
}
