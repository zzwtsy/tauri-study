use std::{
    env,
    fs::{remove_file, File},
    path::PathBuf,
};

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
    pub fn file_exists(filepath: &str) -> bool {
        println!("{}", env::current_dir().unwrap().display());
        let mut path = PathBuf::new();
        path.push(env::current_dir().unwrap());
        path.push(filepath);
        path.exists()
    }
}
