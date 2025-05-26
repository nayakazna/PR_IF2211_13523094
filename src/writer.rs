/////////////////////////////////////////////////////////////////
// File ini berisi writer untuk menulis output ke file. 
/////////////////////////////////////////////////////////////////
use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::path::Path;


// content: konten yang akan ditulis ke file
// file_path_str: path ke file tempat konten akan ditulis
// -> io::Result<()>: Hasil operasi penulisan ke file
pub fn write_output_to_file(file_path_str: &str, content: &str) -> io::Result<()> {
    let path = Path::new(file_path_str);

    if let Some(parent_dir) = path.parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)?;
        }
    }
    
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(content.as_bytes())?;    
    writer.flush()?;

    Ok(())
}