use anyhow::Result;

fn main() -> Result<()> {
    use std::fs::File;
    use std::io::Write as _;
    use std::time::SystemTime;

    let n = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let seed = (n.as_secs() & 0xffffffff) as u32;

    let mut file = File::options()
        .write(true)
        .append(false)
        .create(true)
        .open("./src/rand_seed.txt")?;
    write!(file, "{}", seed)?;

    Ok(())
}
