//! 肠易激综合征管理
//!
//! 肠易激综合征的饮食调节与生活管理要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: IbsManagementRules,
    name: "肠易激综合征管理",
    desc: "肠易激综合征的饮食调节与生活管理要点",
    origin: "中国",
    tags: ["消化", "肠道", "慢性"]
}

impl IbsManagementRules {
    /// 饮食调节
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "规律进食避免暴饮暴食",
            "记录饮食与症状关系",
            "可试低发漫饮食法",
            "少食产气与刺激食物",
        ]
    }

    /// 纤维选择
    pub fn fiber(&self) -> Vec<&'static str> {
        vec![
            "可溶性纤维有助通便",
            "增加纤维宜循序渐进",
            "足量饮水配合纤维",
            "腹泻型慎用粗纤维",
        ]
    }

    /// 生活管理
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "规律作息充足睡眠",
            "适度运动缓解压力",
            "学会放松减轻焦虑",
            "情绪管理助控症状",
        ]
    }

    /// 就医要点
    pub fn medical(&self) -> Vec<&'static str> {
        vec![
            "便血体重下降须排查",
            "症状突变及时就诊",
            "四十岁后初发应查",
            "遵医嘱用药不自行停",
        ]
    }
}

impl Rule for IbsManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("ibs_management")
    }

    fn explain(&self) -> String {
        format!(
            "【肠易激综合征管理】\n{}",
            [
                format!(
                    "饮食调节：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "纤维选择：\\n{}",
                    self.fiber()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活管理：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医要点：\\n{}",
                    self.medical()
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
    fn test_ibsmanagementrules_basic() {
        let rules = IbsManagementRules::new();
        assert_eq!(rules.metadata().name, "肠易激综合征管理");
        assert!(!rules.diet().is_empty());
        assert!(!rules.fiber().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.medical().is_empty());
    }

    #[test]
    fn test_ibsmanagementrules_validation() {
        let rules = IbsManagementRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("ibs_management"));
    }

    #[test]
    fn test_ibsmanagementrules_explain() {
        let rules = IbsManagementRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食调节"));
        assert!(e.contains("纤维选择"));
        assert!(e.contains("生活管理"));
    }
}
