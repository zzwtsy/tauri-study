use std::{env, path::PathBuf};

// 文件工具类
pub struct FileUtils {}

impl FileUtils {
    // 检测文件是否存在
    pub fn file_exists(filepath: &str) -> bool {
        let mut path = PathBuf::new();
        path.push(env::current_dir().unwrap());
        path.push(filepath);
        path.exists()
    }
}
