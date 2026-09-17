//! 职业倦怠预防
//!
//! 识别职场倦怠、恢复精力与平衡生活的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BurnoutPreventionRules,
    name: "职业倦怠预防",
    desc: "识别职场倦怠、恢复精力与平衡生活的规则",
    origin: "心理学",
    tags: ["健康", "倦怠", "职业", "工作", "平衡"]
}

impl BurnoutPreventionRules {
    /// 识别倦怠
    pub fn signs(&self) -> Vec<&'static str> {
        vec![
            "留意过度疲惫难恢复",
            "关注对工作冷漠疏离",
            "注意效率下降抱怨增多",
            "承认累并重视休息",
        ]
    }

    /// 设定边界
    pub fn boundary(&self) -> Vec<&'static str> {
        vec![
            "合理管理工作时间",
            "学会婉拒不合理的过度",
            "保证休息与假期",
            "工作与生活区分开",
        ]
    }

    /// 恢复精力
    pub fn recharge(&self) -> Vec<&'static str> {
        vec![
            "规律充足睡眠",
            "适度运动与放松",
            "培养工作外爱好",
            "亲近人际支持",
        ]
    }

    /// 寻求调整
    pub fn adjust(&self) -> Vec<&'static str> {
        vec![
            "与上级沟通工作量",
            "适当调整任务分配",
            "严重时寻求专业帮助",
            "必要时考虑工作调整",
        ]
    }
}

impl Rule for BurnoutPreventionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("burnout_prevention")
    }

    fn explain(&self) -> String {
        format!(
            "【职业倦怠预防】\n{}",
            [
                format!(
                    "识别倦怠：\\n{}",
                    self.signs()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "设定边界：\\n{}",
                    self.boundary()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "恢复精力：\\n{}",
                    self.recharge()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "寻求调整：\\n{}",
                    self.adjust()
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
    fn test_burnoutpreventionrules_basic() {
        let rules = BurnoutPreventionRules::new();
        assert_eq!(rules.metadata().name, "职业倦怠预防");
        assert!(!rules.signs().is_empty());
        assert!(!rules.boundary().is_empty());
        assert!(!rules.recharge().is_empty());
        assert!(!rules.adjust().is_empty());
    }

    #[test]
    fn test_burnoutpreventionrules_validation() {
        let rules = BurnoutPreventionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("burnout_prevention"));
    }

    #[test]
    fn test_burnoutpreventionrules_explain() {
        let rules = BurnoutPreventionRules::new();
        let e = rules.explain();
        assert!(e.contains("识别倦怠"));
        assert!(e.contains("设定边界"));
        assert!(e.contains("恢复精力"));
    }
}
