use hashbrown::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct FileStruct {
    name: String,
    size: usize,
    children: Vec<FileStruct>,
    is_dir: bool,
}

impl FileStruct {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: 0,
            children: Vec::new(),
            is_dir: false,
        }
    }

    fn get_path(&mut self, path: &[String]) -> &mut Self {
        let mut current = self;
        for part in path {
            current = current
                .children
                .iter_mut()
                .find(|f| f.name == *part)
                .unwrap();
        }

        current
    }

    fn propagate_size(&mut self) -> usize {
        for i in &mut self.children {
            self.size += i.propagate_size();
        }

        self.size
    }

    fn get_all_children(&self) -> HashSet<Self> {
        let mut out = HashSet::new();

        for i in &self.children {
            out.insert(i.clone());
            out.extend(i.get_all_children());
        }

        out
    }
}

fn main() {
    if let Ok(lines) = read_lines("./7.txt") {
        let mut tree = FileStruct::new("root");
        let mut path: Vec<String> = Vec::new();

        for line in lines {
            if let Ok(inputs) = line {
                println!("{:?}", inputs);

                let parts = inputs.split_whitespace().collect::<Vec<_>>();
                println!("{:?}", parts);

                if parts[..2] == ["$", "cd"] {
                    match parts[2] {
                        "/" => continue,
                        ".." => {
                            path.pop().unwrap();
                            continue;
                        }
                        _ => {}
                    }

                    let parent = tree.get_path(&path);
                    path.push(parts[2].to_owned());

                    if parent.children.iter().any(|x| x.name == parts[2]) {
                        continue;
                    }

                    parent.children.push(FileStruct::new(parts[2]));
                }

                if parts[0] == "dir" {
                    let parent = tree.get_path(&path);

                    if let Some(i) = parent.children.iter_mut().find(|x| x.name == parts[1]) {
                        i.is_dir = true;
                    }

                    let mut child = FileStruct::new(parts[1]);
                    child.is_dir = true;
                    parent.children.push(child);
                    continue;
                }

                if let Ok(i) = parts[0].parse::<usize>() {
                    let mut child = FileStruct::new(parts[1]);
                    child.size = i;
                    tree.get_path(&path).children.push(child);
                }

                println!("========")
            }
        }

        tree.propagate_size();

        let result = tree
            .get_all_children()
            .iter()
            .filter(|x| x.is_dir && x.size <= 100000)
            .fold(0, |a, x| a + x.size)
            .to_string();
        println!("{:?}", result);

        let needed_space = 30000000 - (70000000 - tree.size);

        let folder_vec = tree.get_all_children();
        let mut folder_vec = folder_vec.iter().collect::<Vec<_>>();
        folder_vec.sort_by(|a, b| a.size.cmp(&b.size));

        let part_b = folder_vec
            .iter()
            .find(|x| x.is_dir && x.size > needed_space)
            .unwrap()
            .size
            .to_string();
        println!("{:?}", part_b);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
