
pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {

        use std::{fs::File, io::{BufWriter, Write}};

        let logs = self.search(keyword);
        let mut file = BufWriter::new(File::create(path)?);
        for log in logs {
            writeln!(file, "{log}")?;
        }
        file.flush()?;
        Ok(())
    }

}
