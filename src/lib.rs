#[cfg(test)]
extern crate temp_testdir;

use std::path::Path;
use std::fs::File;
use std::ops::Deref;
use std::io::Read;
use temp_testdir::TempDir;
use std::path::PathBuf;
use std::io::Write;

struct TempPath {
    #[allow(dead_code)]
    root: TempDir,
    path: PathBuf,
}

impl TempPath {
    pub fn new<P: AsRef<Path>>(name: P) -> Self {
        let root = TempDir::default();
        let path = root.join(name.as_ref());

        TempPath { root, path }
    }
}

impl Deref for TempPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.path.as_ref()
    }
}

impl AsRef<Path> for TempPath {
    fn as_ref(&self) -> &Path {
        self
    }
}

pub struct Person {
    pub name: String,
    pub surname: String,
    pub age: u8
}

impl Person {
    pub fn new(name: &str, surname: &str, age: u8) -> Self {
         Self { name: name.to_string(), surname: surname.to_string(), age }
    }
}

#[derive(Default)]
pub struct Repository {
    entries: Vec<Person>
}

impl Repository {
    pub fn insert(&mut self, p: Person) {
        self.entries.push(p)
    }

    pub fn all(&self) -> &Vec<Person> {
        &self.entries
    }
}

pub fn dump_repository(repository: &Repository, out: &Path) {
    let mut f = File::create(out).unwrap();
    let persons = repository.all();
    if persons.is_empty() {
        write!(f, "No Entries").unwrap();
    } else {
        persons.iter().for_each(|p|
            writeln!(f, "{}, {} : {}", p.name, p.surname, p.age).unwrap()
        )
    }
}

fn out() -> TempPath {
    TempPath::new("dump")
}

fn repository() -> Repository {
    Default::default()
}

fn two_entries_repository() -> Repository {
    let mut repository = repository();

    repository.insert(Person::new("Michele", "d'Amico", 44));
    repository.insert(Person::new("John", "Doe", 37));

    repository
}

/// Desiderata code:
/// ```
/// #[rstest]
/// fn dump_empty_repository(out: TempPath, repository: Repository) {
///     dump_repository(&repository, &out);
///
///     let mut content= String::new();
///     File::open(&out).unwrap().read_to_string(&mut content).unwrap();
///
///     assert_eq!(content.trim(), "No Entries");
/// }
///
/// #[rstest]
/// fn dump_no_empty_repository(out: TempPath, two_entries_repository: Repository) {
///     dump_repository(&two_entries_repository, &out);
///
///     let mut content = String::new();
///     File::open(&out).unwrap().read_to_string(&mut content).unwrap();
///
///     assert!(content.contains("Michele, d'Amico : 44"));
///     assert!(content.contains("John, Doe : 37"));
///     assert_eq!(2, content.lines().count());
/// }


/// Desugared!!!!

#[test]
fn dump_empty_repository() {
    let out = out();
    let repository = repository();

    dump_repository(&repository, &out);

    let mut content = String::new();
    File::open(&out).unwrap().read_to_string(&mut content).unwrap();

    assert_eq!(content.trim(), "No Entries");
}

#[test]
fn dump_no_empty_repository() {
    let out = out();
    let two_entries_repository = two_entries_repository();

    dump_repository(&two_entries_repository, &out);

    let mut content= String::new();
    File::open(&out).unwrap().read_to_string(&mut content).unwrap();

    assert!(content.contains("Michele, d'Amico : 44"));
    assert!(content.contains("John, Doe : 37"));
    assert_eq!(2, content.lines().count());
}