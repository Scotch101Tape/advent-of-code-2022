use std::fs;
use util::regex::{lazy_static, Regex};

#[derive(Debug)]
enum FileSpecs {
    Dir(Vec<FileRef>),
    File(i32),
}

#[derive(Debug)]
struct File {
    specs: FileSpecs,
    parent: Option<FileRef>,
    name: String,
}

#[derive(Clone, Copy, Debug)]
struct FileRef(usize);

#[derive(Debug)]
struct Files(Vec<File>);

impl Files {
    fn add_file(&mut self, file: File) -> FileRef {
        let file_ref = FileRef(self.0.len());

        if let Some(parent_ref) = file.parent {
            let parent = self.get_file_mut(parent_ref).unwrap();
            let children = match &mut parent.specs {
                FileSpecs::Dir(children) => children,
                _ => panic!("Parent must be dir"),
            };
            children.push(file_ref);
        }

        self.0.push(file);
        file_ref
    }

    fn get_file(&self, file_ref: FileRef) -> Option<&File> {
        self.0.get(file_ref.0)
    }

    fn get_file_mut(&mut self, file_ref: FileRef) -> Option<&mut File> {
        self.0.get_mut(file_ref.0)
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day-07.txt").unwrap();

    let mut files = Files(Vec::new());
    let top_dir = File {
        specs: FileSpecs::Dir(Vec::new()),
        parent: None,
        name: "XXX_TOP_LEVEL_XXX".to_string(),
    };
    let top_dir_ref = files.add_file(top_dir);
    let mut current_dir_ref = top_dir_ref;

    // skip the problematic first
    // $ cd /
    for line in input.lines().skip(1) {
        lazy_static! {
            static ref CD_REGEX: Regex = Regex::new(r"\$ cd (.*)").unwrap();
            static ref LS_REGEX: Regex = Regex::new(r"\$ ls").unwrap();
            static ref DIR_REGEX: Regex = Regex::new(r"dir (.*)").unwrap();
            static ref FILE_REGEX: Regex = Regex::new(r"(\d*) (.*)").unwrap();
        }

        let current_dir = files.get_file(current_dir_ref).unwrap();

        if let Some(caps) = CD_REGEX.captures(line) {
            let dir_name = caps.get(1).unwrap().as_str();

            current_dir_ref = if dir_name == ".." {
                current_dir.parent.unwrap()
            } else if let FileSpecs::Dir(children) = &current_dir.specs {
                *children
                    .iter()
                    .find(|child_ref| files.get_file(**child_ref).unwrap().name == dir_name)
                    .unwrap()
            } else {
                panic!()
            };
        } else if let Some(_caps) = LS_REGEX.captures(line) {
        } else if let Some(caps) = DIR_REGEX.captures(line) {
            let new_dir = File {
                specs: FileSpecs::Dir(Vec::new()),
                parent: Some(current_dir_ref),
                name: caps.get(1).unwrap().as_str().to_string(),
            };

            files.add_file(new_dir);
        } else if let Some(caps) = FILE_REGEX.captures(line) {
            let new_file = File {
                specs: FileSpecs::File(caps.get(1).unwrap().as_str().parse::<i32>().unwrap()),
                parent: Some(current_dir_ref),
                name: caps.get(2).unwrap().as_str().to_string(),
            };

            files.add_file(new_file);
        }
    }

    let get_size = |file| {
        fn get_size(files: &Files, file: &File) -> i32 {
            match &file.specs {
                FileSpecs::File(size) => *size,
                FileSpecs::Dir(children) => children
                    .iter()
                    .map(|child| get_size(files, files.get_file(*child).unwrap()))
                    .sum(),
            }
        }

        get_size(&files, file)
    };

    let answer = files
        .0
        .iter()
        .filter(|file| matches!(file.specs, FileSpecs::Dir(_)))
        .map(get_size)
        .filter(|size| *size <= 100_000)
        .sum::<i32>();

    println!("{answer}");
}
