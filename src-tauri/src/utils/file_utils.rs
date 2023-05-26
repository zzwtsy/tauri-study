use std::fs::{File, remove_file};

// 文件工具类
pub struct FileUtils {}

impl FileUtils {
    // 创建文件
    pub fn create_file(filename: &str) -> std::io::Result<File> {
        File::create(filename)
    }

    // 删除文件
    pub fn remove_file(filename: &str) -> std::io::Result<()> {
        remove_file(filename)
    }

    // 检测文件是否存在
    pub fn file_exists(filename: &str) -> bool {
        // File::open(filename) 返回 Ok(_) 表示文件存在
        matches!(File::open(filename), Ok(_))
    }
}