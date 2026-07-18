//! # 规则上传下载管理
//!
//! 提供规则包的上传、下载、版本管理功能。

use super::types::{MarketError, RulePackage};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// 上传管理器
#[derive(Debug)]
pub struct UploadManager {
    /// 上传目录
    upload_dir: PathBuf,
    /// 最大文件大小（字节）
    max_file_size: usize,
    /// 允许的文件扩展名
    allowed_extensions: Vec<String>,
}

impl UploadManager {
    /// 创建新的上传管理器
    ///
    /// # Arguments
    ///
    /// * `upload_dir` - 上传目录路径
    pub fn new(upload_dir: impl Into<PathBuf>) -> Self {
        Self {
            upload_dir: upload_dir.into(),
            max_file_size: 10 * 1024 * 1024, // 10 MB
            allowed_extensions: vec!["json".to_string(), "zip".to_string()],
        }
    }

    /// 处理上传请求
    ///
    /// # Arguments
    ///
    /// * `filename` - 文件名
    /// * `content` - 文件内容
    pub fn upload(&self, filename: &str, content: &[u8]) -> Result<RulePackage, MarketError> {
        // 验证文件大小
        if content.len() > self.max_file_size {
            return Err(MarketError::UploadFailed(format!(
                "文件大小超过限制（{} MB）",
                self.max_file_size / 1024 / 1024
            )));
        }

        // 验证文件扩展名
        let extension = Path::new(filename)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if !self.allowed_extensions.contains(&extension.to_string()) {
            return Err(MarketError::UploadFailed(format!(
                "不允许的文件类型: {}",
                extension
            )));
        }

        // 创建上传目录（如果不存在）
        fs::create_dir_all(&self.upload_dir)
            .map_err(|e| MarketError::UploadFailed(format!("创建目录失败: {}", e)))?;

        // 保存文件
        let file_path = self.upload_dir.join(filename);
        let mut file = fs::File::create(&file_path)
            .map_err(|e| MarketError::UploadFailed(format!("创建文件失败: {}", e)))?;

        file.write_all(content)
            .map_err(|e| MarketError::UploadFailed(format!("写入文件失败: {}", e)))?;

        // 解析规则包
        let package: RulePackage = serde_json::from_slice(content)
            .map_err(|e| MarketError::ParseError(format!("解析规则包失败: {}", e)))?;

        Ok(package)
    }

    /// 获取上传目录
    pub fn upload_directory(&self) -> &Path {
        &self.upload_dir
    }
}

/// 下载管理器
#[derive(Debug)]
pub struct DownloadManager {
    /// 下载目录
    download_dir: PathBuf,
    /// 下载计数器
    download_counts: HashMap<String, u64>,
}

impl DownloadManager {
    /// 创建新的下载管理器
    ///
    /// # Arguments
    ///
    /// * `download_dir` - 下载目录路径
    pub fn new(download_dir: impl Into<PathBuf>) -> Self {
        Self {
            download_dir: download_dir.into(),
            download_counts: HashMap::new(),
        }
    }

    /// 处理下载请求
    ///
    /// # Arguments
    ///
    /// * `package` - 规则包
    /// * `format` - 下载格式（json 或 zip）
    pub fn download(
        &mut self,
        package: &RulePackage,
        format: &str,
    ) -> Result<Vec<u8>, MarketError> {
        // 验证格式
        if format != "json" && format != "zip" {
            return Err(MarketError::DownloadFailed(format!(
                "不支持的下载格式: {}",
                format
            )));
        }

        // 序列化规则包
        let content = serde_json::to_vec(package)
            .map_err(|e| MarketError::DownloadFailed(format!("序列化失败: {}", e)))?;

        // 增加下载计数
        let count = self.download_counts.entry(package.id.clone()).or_insert(0);
        *count += 1;

        Ok(content)
    }

    /// 获取下载次数
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn get_download_count(&self, rule_id: &str) -> u64 {
        *self.download_counts.get(rule_id).unwrap_or(&0)
    }

    /// 获取下载目录
    pub fn download_directory(&self) -> &Path {
        &self.download_dir
    }
}

/// 版本管理器
#[derive(Debug)]
pub struct VersionManager {
    /// 版本历史
    versions: HashMap<String, Vec<RulePackage>>,
}

impl VersionManager {
    /// 创建新的版本管理器
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    /// 添加新版本
    ///
    /// # Arguments
    ///
    /// * `package` - 规则包
    pub fn add_version(&mut self, package: RulePackage) {
        let rule_id = package.id.clone();
        self.versions.entry(rule_id).or_default().push(package);
    }

    /// 获取最新版本
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn get_latest_version(&self, rule_id: &str) -> Option<&RulePackage> {
        self.versions.get(rule_id).and_then(|v| v.last())
    }

    /// 获取指定版本
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    /// * `version` - 版本号
    pub fn get_version(&self, rule_id: &str, version: &str) -> Option<&RulePackage> {
        self.versions
            .get(rule_id)
            .and_then(|v| v.iter().find(|p| p.version == version))
    }

    /// 获取所有版本
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn get_all_versions(&self, rule_id: &str) -> Option<&[RulePackage]> {
        self.versions.get(rule_id).map(|v| v.as_slice())
    }

    /// 获取版本数量
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn version_count(&self, rule_id: &str) -> usize {
        self.versions.get(rule_id).map(|v| v.len()).unwrap_or(0)
    }
}

impl Default for VersionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_manager_creation() {
        let manager = UploadManager::new("/tmp/uploads");
        assert_eq!(manager.max_file_size, 10 * 1024 * 1024);
    }

    #[test]
    fn test_upload_manager_validation() {
        let manager = UploadManager::new("/tmp/uploads");

        // 测试文件大小超限
        let large_content = vec![0u8; 20 * 1024 * 1024]; // 20 MB
        let result = manager.upload("test.json", &large_content);
        assert!(result.is_err());

        // 测试不允许的文件类型
        let result = manager.upload("test.exe", b"content");
        assert!(result.is_err());
    }

    #[test]
    fn test_download_manager_creation() {
        let manager = DownloadManager::new("/tmp/downloads");
        assert!(manager
            .download_directory()
            .to_str()
            .unwrap()
            .contains("downloads"));
    }

    #[test]
    fn test_download_manager_download() {
        let mut manager = DownloadManager::new("/tmp/downloads");

        let package = RulePackage::new(
            "test-1".to_string(),
            "测试规则".to_string(),
            "games".to_string(),
        );

        let result = manager.download(&package, "json");
        assert!(result.is_ok());

        let count = manager.get_download_count("test-1");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_download_manager_invalid_format() {
        let mut manager = DownloadManager::new("/tmp/downloads");

        let package = RulePackage::new(
            "test-1".to_string(),
            "测试规则".to_string(),
            "games".to_string(),
        );

        let result = manager.download(&package, "exe");
        assert!(result.is_err());
    }

    #[test]
    fn test_version_manager_creation() {
        let manager = VersionManager::new();
        assert_eq!(manager.version_count("test"), 0);
    }

    #[test]
    fn test_version_manager_add_version() {
        let mut manager = VersionManager::new();

        let package = RulePackage::new(
            "test-1".to_string(),
            "测试规则".to_string(),
            "games".to_string(),
        );

        manager.add_version(package);
        assert_eq!(manager.version_count("test-1"), 1);
    }

    #[test]
    fn test_version_manager_get_latest() {
        let mut manager = VersionManager::new();

        let mut package1 = RulePackage::new(
            "test-1".to_string(),
            "测试规则".to_string(),
            "games".to_string(),
        );
        package1.version = "1.0.0".to_string();

        let mut package2 = RulePackage::new(
            "test-1".to_string(),
            "测试规则".to_string(),
            "games".to_string(),
        );
        package2.version = "2.0.0".to_string();

        manager.add_version(package1);
        manager.add_version(package2);

        let latest = manager.get_latest_version("test-1");
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().version, "2.0.0");
    }

    #[test]
    fn test_version_manager_get_specific_version() {
        let mut manager = VersionManager::new();

        let mut package1 = RulePackage::new(
            "test-1".to_string(),
            "测试规则".to_string(),
            "games".to_string(),
        );
        package1.version = "1.0.0".to_string();

        let mut package2 = RulePackage::new(
            "test-1".to_string(),
            "测试规则".to_string(),
            "games".to_string(),
        );
        package2.version = "2.0.0".to_string();

        manager.add_version(package1);
        manager.add_version(package2);

        let version1 = manager.get_version("test-1", "1.0.0");
        assert!(version1.is_some());
        assert_eq!(version1.unwrap().version, "1.0.0");

        let version2 = manager.get_version("test-1", "2.0.0");
        assert!(version2.is_some());
        assert_eq!(version2.unwrap().version, "2.0.0");
    }

    #[test]
    fn test_version_manager_get_all_versions() {
        let mut manager = VersionManager::new();

        let package1 = RulePackage::new(
            "test-1".to_string(),
            "测试规则".to_string(),
            "games".to_string(),
        );

        let package2 = RulePackage::new(
            "test-1".to_string(),
            "测试规则".to_string(),
            "games".to_string(),
        );

        manager.add_version(package1);
        manager.add_version(package2);

        let versions = manager.get_all_versions("test-1");
        assert!(versions.is_some());
        assert_eq!(versions.unwrap().len(), 2);
    }

    #[test]
    fn test_version_manager_default() {
        let manager = VersionManager::default();
        assert_eq!(manager.version_count("test"), 0);
    }
}
