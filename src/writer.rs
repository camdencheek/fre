
pub fn read_store(path: &PathBuf) -> Result<UsageStore, io::Error> {
    if path.is_file() {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let store = serde_json::from_reader(reader)?;
        Ok(store)
    } else {
        Ok(UsageStore::default())
    }
}

pub fn write_store(d: &UsageStore, path: &PathBuf) -> io::Result<()> {
    let store_dir = path.parent().expect("file must have parent");
    fs::create_dir_all(&store_dir)?;
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &d)?;

    return Ok(());
}
