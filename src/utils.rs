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
