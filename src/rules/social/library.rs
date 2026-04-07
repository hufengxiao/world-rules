//! 图书馆礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 图书馆礼仪规则
pub struct LibraryEtiquette {
    metadata: RuleMetadata,
}

impl LibraryEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "图书馆礼仪",
                "图书馆使用礼仪规范"
            )
            .with_origin("通用")
            .with_tags(vec!["社交".into(), "图书馆".into()]),
        }
    }

    /// 入馆礼仪
    pub fn entrance_rules(&self) -> Vec<&'static str> {
        vec![
            "保持安静",
            "手机静音",
            "不要带食物和饮料",
            "按规范存放物品",
        ]
    }

    /// 借阅礼仪
    pub fn borrowing_rules(&self) -> Vec<&'static str> {
        vec![
            "按期归还图书",
            "不要在书上做标记",
            "不要撕毁书页",
            "借书前检查书籍状况",
            "按时归还或续借",
        ]
    }

    /// 阅读礼仪
    pub fn reading_etiquette(&self) -> Vec<&'static str> {
        vec![
            "保持安静",
            "不要占座",
            "用完的书放回原处或还书车",
            "不要在馆内打电话",
            "不要在馆内吃东西",
        ]
    }

    /// 座位礼仪
    pub fn seating_rules(&self) -> Vec<&'static str> {
        vec![
            "不要占座",
            "暂时离开要收拾物品",
            "不要趴桌睡觉",
            "不要把脚放在座位上",
        ]
    }

    /// 电子设备使用
    pub fn device_rules(&self) -> Vec<&'static str> {
        vec![
            "手机静音",
            "接电话到馆外",
            "使用电脑要轻声",
            "不要在馆内看视频(无声除外)",
        ]
    }
}

impl Default for LibraryEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LibraryEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("library")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【图书馆礼仪】\n\n\
            入馆礼仪:\n{}\n\n\
            借阅礼仪:\n{}\n\n\
            阅读礼仪:\n{}\n\n\
            座位礼仪:\n{}\n",
            self.entrance_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.borrowing_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.reading_etiquette().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.seating_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_etiquette() {
        let etiquette = LibraryEtiquette::new();
        assert!(etiquette.borrowing_rules().contains(&"按期归还图书"));
    }
}