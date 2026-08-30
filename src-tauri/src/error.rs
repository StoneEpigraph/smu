use serde::Serializer;

/// 数据层统一错误类型；序列化为字符串，前端拿到的错误格式保持不变
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("文件系统错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("应用目录错误: {0}")]
    Path(#[from] tauri::Error),
    #[error("数据库连接不可用")]
    Lock,
}

impl serde::Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
