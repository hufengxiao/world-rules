//! 助行器正确使用
//!
//! 拐杖助行架的正确调高、迈步与安全

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MobilityAidWalkerRules,
    name: "助行器正确使用",
    desc: "拐杖助行架的正确调高、迈步与安全",
    origin: "医学",
    tags: ["健康", "助行器", "拐杖", "老人"]
}

impl MobilityAidWalkerRules {
    /// 选械合适
    pub fn choose(&self) -> Vec<&'static str> {
        vec!["按需选拐杖或助行架", "高度合适", "握把舒适", "胶垫防滑"]
    }

    /// 正确迈步
    pub fn walking(&self) -> Vec<&'static str> {
        vec![
            "先迈助行器再迈脚",
            "重心移动平稳",
            "慢行不赶",
            "上下台阶留意",
        ]
    }

    /// 安全维护
    pub fn maintain(&self) -> Vec<&'static str> {
        vec!["检查胶垫磨损", "松动及时修", "不超负荷", "存放稳固"]
    }

    /// 用前评估
    pub fn evaluate(&self) -> Vec<&'static str> {
        vec!["医生评估需要", "平衡差者用围架", "不擅自改用", "安全第一"]
    }
}

impl Rule for MobilityAidWalkerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("mobility_aid")
    }

    fn explain(&self) -> String {
        format!(
            "【助行器正确使用】\n{}",
            [
                format!(
                    "选械合适：\\n{}",
                    self.choose()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确迈步：\\n{}",
                    self.walking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全维护：\\n{}",
                    self.maintain()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用前评估：\\n{}",
                    self.evaluate()
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
    fn test_mobilityaidwalkerrules_basic() {
        let rules = MobilityAidWalkerRules::new();
        assert_eq!(rules.metadata().name, "助行器正确使用");
        assert!(!rules.choose().is_empty());
        assert!(!rules.walking().is_empty());
        assert!(!rules.maintain().is_empty());
        assert!(!rules.evaluate().is_empty());
    }

    #[test]
    fn test_mobilityaidwalkerrules_validation() {
        let rules = MobilityAidWalkerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("mobility_aid"));
    }

    #[test]
    fn test_mobilityaidwalkerrules_explain() {
        let rules = MobilityAidWalkerRules::new();
        let e = rules.explain();
        assert!(e.contains("选械合适"));
        assert!(e.contains("正确迈步"));
        assert!(e.contains("安全维护"));
    }
}
