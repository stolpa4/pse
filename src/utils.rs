use std::str::FromStr;

const KIB: f64 = 1024.0;
const MIB: f64 = KIB * 1024.0;
const GIB: f64 = MIB * 1024.0;
const TIB: f64 = GIB * 1024.0;

pub fn size_to_label(size: u64) -> String {
    if size < 1024 {
        return format!("{} bytes", size);
    }

    let size_f64 = size as f64;
    if size_f64 < MIB {
        return format!("{:.4} KB", size_f64 / KIB);
    }
    if size_f64 < GIB {
        return format!("{:.4} MB", size_f64 / MIB);
    }
    if size_f64 < TIB {
        return format!("{:.4} GB", size_f64 / GIB);
    }
    format!("{:.4} TB", size_f64 / TIB)
}

pub fn label_to_size(label: &str) -> Result<u64, String> {
    let parts: Vec<&str> = label.trim().split_whitespace().collect();

    if parts.len() != 2 {
        return Err("Invalid format".to_string());
    }

    let number = f64::from_str(parts[0]).map_err(|_| "Invalid number".to_string())?;
    let unit = parts[1].to_lowercase();

    match unit.as_str() {
        "bytes" => Ok(number as u64),
        "kb" => Ok((number * KIB) as u64),
        "mb" => Ok((number * MIB) as u64),
        "gb" => Ok((number * GIB) as u64),
        "tb" => Ok((number * TIB) as u64),
        _ => Err("Invalid unit".to_string()),
    }
}
