//! 网络骚扰维权
//!
//! 遭遇网络辱骂骚扰诽谤时的取证与维权路径

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OnlineHarassmentLawRules,
    name: "网络骚扰维权",
    desc: "遭遇网络辱骂骚扰诽谤时的取证与维权路径",
    origin: "中国",
    tags: ["法律", "网络", "人格权"]
}

impl OnlineHarassmentLawRules {
    /// 行为界定
    pub fn definition(&self) -> Vec<&'static str> {
        vec![
            "辱骂诽谤属侵权",
            "恶意骚扰可违法",
            "人肉搜索侵犯隐私",
            "威胁恐吓或涉犯罪",
        ]
    }

    /// 证据固定
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "及时截图保存内容",
            "记录发布者账号信息",
            "可申请平台留存证据",
            "必要时公证保全",
        ]
    }

    /// 处理途径
    pub fn channels(&self) -> Vec<&'static str> {
        vec![
            "要求平台删除屏蔽",
            "向平台投诉举报",
            "报警处理违法行为",
            "提起民事诉讼索赔",
        ]
    }

    /// 法律责任
    pub fn liability(&self) -> Vec<&'static str> {
        vec![
            "侵权人承担民事赔偿",
            "情节严重可治安处罚",
            "构成犯罪追究刑责",
            "平台失职亦担责",
        ]
    }
}

impl Rule for OnlineHarassmentLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("online_harassment_law")
    }

    fn explain(&self) -> String {
        format!(
            "【网络骚扰维权】\n{}",
            [
                format!(
                    "行为界定：\\n{}",
                    self.definition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "证据固定：\\n{}",
                    self.evidence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "处理途径：\\n{}",
                    self.channels()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "法律责任：\\n{}",
                    self.liability()
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
    fn test_onlineharassmentlawrules_basic() {
        let rules = OnlineHarassmentLawRules::new();
        assert_eq!(rules.metadata().name, "网络骚扰维权");
        assert!(!rules.definition().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.channels().is_empty());
        assert!(!rules.liability().is_empty());
    }

    #[test]
    fn test_onlineharassmentlawrules_validation() {
        let rules = OnlineHarassmentLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("online_harassment_law"));
    }

    #[test]
    fn test_onlineharassmentlawrules_explain() {
        let rules = OnlineHarassmentLawRules::new();
        let e = rules.explain();
        assert!(e.contains("行为界定"));
        assert!(e.contains("证据固定"));
        assert!(e.contains("处理途径"));
    }
}
