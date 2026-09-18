//! 嘴唇干裂护理
//!
//! 秋冬嘴唇干燥脱皮干裂的保湿与护理方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChappedLipCareRules,
    name: "嘴唇干裂护理",
    desc: "秋冬嘴唇干燥脱皮干裂的保湿与护理方法",
    origin: "中国",
    tags: ["健康", "嘴唇", "干裂", "护理"]
}

impl ChappedLipCareRules {
    /// 干裂成因
    pub fn cause(&self) -> Vec<&'static str> {
        vec!["干燥天气蒸发快", "缺水", "风吹日晒", "常舔唇加重"]
    }

    /// 保湿习惯
    pub fn moisture(&self) -> Vec<&'static str> {
        vec!["勤涂润唇膏", "补足水分", "睡前保湿", "选滋润成分"]
    }

    /// 避免行为
    pub fn avoid(&self) -> Vec<&'static str> {
        vec!["不撕扯死皮", "少舔嘴唇", "不用刺激性唇部品", "遮口防风"]
    }

    /// 就医提示
    pub fn seek_help(&self) -> Vec<&'static str> {
        vec!["长期不愈开裂", "红肿疼痛化浓", "角化异常", "及时皮肤科查"]
    }
}

impl Rule for ChappedLipCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("chapped_lip")
    }

    fn explain(&self) -> String {
        format!(
            "【嘴唇干裂护理】\n{}",
            [
                format!(
                    "干裂成因：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保湿习惯：\\n{}",
                    self.moisture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "避免行为：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seek_help()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_chappedlipcarerules_basic() {
        let rules = ChappedLipCareRules::new();
        assert_eq!(rules.metadata().name, "嘴唇干裂护理");
        assert!(!rules.cause().is_empty());
        assert!(!rules.moisture().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.seek_help().is_empty());
    }

    #[test]
    fn test_chappedlipcarerules_validation() {
        let rules = ChappedLipCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("chapped_lip"));
    }

    #[test]
    fn test_chappedlipcarerules_explain() {
        let rules = ChappedLipCareRules::new();
        let e = rules.explain();
        assert!(e.contains("干裂成因"));
        assert!(e.contains("保湿习惯"));
        assert!(e.contains("避免行为"));
    }
}
