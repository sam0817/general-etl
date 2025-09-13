use crate::utils::error::Result;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::{ExtendedFileOptions, FileOptions};
use zip::ZipWriter;

pub struct Archiver;

impl Archiver {
    pub fn create_zip<P: AsRef<Path>>(
        output_path: P,
        files: Vec<(String, Vec<u8>)>,
    ) -> Result<()> {
        let file = File::create(output_path)?;
        let mut zip = ZipWriter::new(file);

        let options: FileOptions<'_, ExtendedFileOptions> = FileOptions::default()
            // .compression_method(zip::CompressionMethod::Deflated)
            .compression_method(zip::CompressionMethod::Ppmd)
            // .compression_level()
            .unix_permissions(0o755);

        for (name, content) in files {
            zip.start_file(name, options.clone())?;
            zip.write_all(&content)?;
        }

        zip.finish()?;
        Ok(())
    }

    pub fn extract_zip<P: AsRef<Path>>(zip_path: P) -> Result<Vec<(String, Vec<u8>)>> {
        let file = File::open(zip_path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        let mut files = Vec::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name().to_string();

            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            files.push((name, contents));
        }

        Ok(files)
    }
}
