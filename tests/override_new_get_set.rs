//! Test that the `constructor` macro overrides
//! the `new` method's name and `get` and `set`
//! change the name of the getter and setter of the fields.
#![allow(warnings)]

use std::fmt::Display;

#[salsa::jar(db = Db)]
struct Jar(MyInput, MyInterned<'_>, MyTracked<'_>);

trait Db: salsa::DbWithJar<Jar> {}

#[salsa::input(jar = Jar, constructor = from_string)]
struct MyInput {
    #[get(text)]
    #[set(set_text)]
    field: String,
}

impl MyInput {
    pub fn new(db: &mut dyn Db, s: impl Display) -> MyInput {
        MyInput::from_string(db, s.to_string())
    }

    pub fn field(self, db: &dyn Db) -> String {
        self.text(db)
    }

    pub fn set_field(self, db: &mut dyn Db, id: String) {
        self.set_text(db).to(id);
    }
}

#[salsa::interned(constructor = from_string)]
struct MyInterned<'db> {
    #[get(text)]
    #[return_ref]
    field: String,
}

impl<'db> MyInterned<'db> {
    pub fn new(db: &'db dyn Db, s: impl Display) -> MyInterned<'db> {
        MyInterned::from_string(db, s.to_string())
    }

    pub fn field(self, db: &'db dyn Db) -> &str {
        &self.text(db)
    }
}

#[salsa::tracked(constructor = from_string)]
struct MyTracked<'db> {
    #[get(text)]
    field: String,
}

impl<'db> MyTracked<'db> {
    pub fn new(db: &'db dyn Db, s: impl Display) -> MyTracked<'db> {
        MyTracked::from_string(db, s.to_string())
    }

    pub fn field(self, db: &'db dyn Db) -> String {
        self.text(db)
    }
}

#[test]
fn execute() {
    #[salsa::db(Jar)]
    #[derive(Default)]
    struct Database {
        storage: salsa::Storage<Self>,
    }

    impl salsa::Database for Database {}

    impl Db for Database {}

    let mut db = Database::default();
}
