use glob::glob;
use std::env;
use std::fs;
use std::time::SystemTime;

type MyResult = Result<(), Box<dyn std::error::Error>>;

/// グロブパターンを引数にとって、マッチしたファイルパスと更新時刻(UNIX_EPOCH)を集計
/// CSVとして構造化
fn main() -> MyResult {
    let args: Vec<String> = env::args().collect();
    let ptrn = &args[1]; // e.g. "**/*txt"
    println!("is_dir,modified,filepath"); // 簡易的にCSV形式で出力
    for entry in glob(ptrn)? {
        let entry = entry?;
        let metadata = fs::metadata(&entry)?;
        let time = if let Ok(time) = metadata.modified() {
            time.duration_since(SystemTime::UNIX_EPOCH).ok().unwrap().as_secs().to_string()
        } else {
            "".to_string()
        };
        let is_dir = if metadata.is_dir() { 1 } else { 0 };
        println!(r#"{},{},"{}""#, is_dir, time, entry.display());
    }

    Ok(())
}
