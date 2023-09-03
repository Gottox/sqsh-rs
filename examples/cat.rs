use sqsh_rs::archive::ArchiveBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let archive_path = std::env::args().nth(1).unwrap();
    let path = std::env::args().nth(2).unwrap();

    let mut archive = ArchiveBuilder::new(archive_path).build()?;

    let file = archive.open(path)?;

    let mut stdout = std::io::stdout();

    let mut reader = file.reader()?;
    std::io::copy(&mut reader, &mut stdout)?;

    Ok(())
}
