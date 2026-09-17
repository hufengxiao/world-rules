//! 共用办公与茶水间礼仪
//!
//! 共用办公环境、茶水间与邻里协作礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SharedOfficeCourtesyRules,
    name: "共用办公与茶水间礼仪",
    desc: "共用办公环境、茶水间与邻里协作礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "办公室", "茶水间"]
}

impl SharedOfficeCourtesyRules {
    /// 公共使用
    pub fn common(&self) -> Vec<&'static str> {
        vec!["公共设备轮流用", "用完归位整洁", "节约耗材", "保持工位整洁"]
    }

    /// 茶水间礼仪
    pub fn kitchen(&self) -> Vec<&'static str> {
        vec![
            "用了餐具及时清洁",
            "食物集中分享或标注",
            "冰箱私物标注",
            "保持台面干净",
        ]
    }

    /// 安静专注
    pub fn quiet(&self) -> Vec<&'static str> {
        vec![
            "低声交谈少打扰",
            "电话到走廊",
            "尊重他人专注",
            "避免制造噪音",
        ]
    }

    /// 邻里协作
    pub fn team(&self) -> Vec<&'static str> {
        vec![
            "互相支持问候",
            "主动协助分担",
            "经验乐于分享",
            "营造和睦氛围",
        ]
    }
}

impl Rule for SharedOfficeCourtesyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("shared_office")
    }

    fn explain(&self) -> String {
        format!(
            "【共用办公与茶水间礼仪】\n{}",
            [
                format!(
                    "公共使用：\\n{}",
                    self.common()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "茶水间礼仪：\\n{}",
                    self.kitchen()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安静专注：\\n{}",
                    self.quiet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "邻里协作：\\n{}",
                    self.team()
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
    fn test_sharedofficecourtesyrules_basic() {
        let rules = SharedOfficeCourtesyRules::new();
        assert_eq!(rules.metadata().name, "共用办公与茶水间礼仪");
        assert!(!rules.common().is_empty());
        assert!(!rules.kitchen().is_empty());
        assert!(!rules.quiet().is_empty());
        assert!(!rules.team().is_empty());
    }

    #[test]
    fn test_sharedofficecourtesyrules_validation() {
        let rules = SharedOfficeCourtesyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("shared_office"));
    }

    #[test]
    fn test_sharedofficecourtesyrules_explain() {
        let rules = SharedOfficeCourtesyRules::new();
        let e = rules.explain();
        assert!(e.contains("公共使用"));
        assert!(e.contains("茶水间礼仪"));
        assert!(e.contains("安静专注"));
    }
}
