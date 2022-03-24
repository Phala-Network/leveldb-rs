use rusty_leveldb;
use std::path::Path;
use std::rc::Rc;

const DB_PATH: &'static str = "/data";

fn aux_get_byte_slice<T: AsRef<[u8]>>(source: &'_ T) -> &'_ [u8] {
    source.as_ref()
}

fn main() {
    let path = format!("{}/simple_db_workds", DB_PATH);
    {
        let mut options = rusty_leveldb::Options::default();
        options.env = Rc::new(Box::new(rusty_leveldb::gramine_env::GramineEnv::new()));
        let mut db = rusty_leveldb::DB::open(Path::new(&path), options).unwrap();
        assert!(db.put(b"k1", b"v1").is_ok());
        let result = db.get(b"k1").unwrap();
        assert_eq!(aux_get_byte_slice(&result), b"v1");
    }
}
