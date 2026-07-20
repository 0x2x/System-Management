 // TODO: Make sure application functionality works.
pub fn write_to_json(file_path: String, file_name: String, data: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = format!("{}/{}", file_path, file_name);
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())

}


// Quick Caching System to save and return results when application ends
pub struct Cache {
    pub data: serde_json::Value,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: serde_json::json!({}),
        }
    }

    pub fn get_key(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    pub fn edit_value_by_key(&mut self, key: &str, new_value: serde_json::Value) {
        if let Some(value) = self.data.get_mut(key) {
            *value = new_value;
        }
    }

    pub fn add_key_value(&mut self, key: &str, value: serde_json::Value) {
        self.data[key] = value;
    }

    pub fn erase_key(&mut self, key: &str) {
        self.data.as_object_mut().map(|obj| obj.remove(key));
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, &self.data)?;
        Ok(())
    }
    
    pub fn load_from_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        self.data = serde_json::from_reader(file)?;
        Ok(())
    }
}