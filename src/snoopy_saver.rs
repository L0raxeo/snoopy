pub mod snoopy_save {

    use std::{fs::File, io::Write};

    pub fn save(code: &str) -> std::io::Result<()> {
        let mut file = File::create("snoopster.txt")?;
        file.write_all(code.as_bytes())?;
        Ok(())
    }
}

/*
pub mod snoopy_json_serializer {

    use serde_json::*;

    pub fn serialize() {
        println!("serializing")
    }
}
*/
