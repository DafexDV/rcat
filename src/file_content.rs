use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) enum Content {
    Lines(Vec<String>),
    Error(String),
}

pub(crate) struct FileContent {
    path: PathBuf,
    content: Content,
}

impl FileContent {
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn content(&self) -> &Content {
        &self.content
    }
}

pub(crate) fn read_file_contents(files: &Vec<String>) -> Vec<FileContent> {
    let mut file_contents: Vec<FileContent> = Vec::new();

    for file in files {
        let path = Path::new(file);

        let content: Content;

        if path.exists() {
            match fs::read_to_string(path) {
                Ok(contents) => {
                    let mut lines: Vec<String> = Vec::new();

                    for l in contents.lines() {
                        lines.push(l.to_owned());
                    }

                    content = Content::Lines(lines);
                }
                Err(error) => {
                    content = Content::Error(error.to_string());
                }
            }
        } else {
            content = Content::Error("No such file or directory".to_owned());
        }

        file_contents.push(FileContent {
            path: path.into(),
            content,
        });
    }

    file_contents
}
