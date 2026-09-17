//! 逻辑谬误识别
//!
//! 识别常见推理错误、避免以偏概全与诉诸情感

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LogicMistakesAvoidRules,
    name: "逻辑谬误识别",
    desc: "识别常见推理错误、避免以偏概全与诉诸情感",
    origin: "国际",
    tags: ["科学", "逻辑", "推理", "谬误"]
}

impl LogicMistakesAvoidRules {
    /// 常见谬误
    pub fn fallacies(&self) -> Vec<&'static str> {
        vec![
            "警惕人身攻击论证",
            "识别诉诸情感压制",
            "防止稻草人曲解",
            "不以个别特例概全",
        ]
    }

    /// 因果辨析
    pub fn causation(&self) -> Vec<&'static str> {
        vec![
            "不把相关当因果",
            "警惕引申因果倒置",
            "寻找干扰变量",
            "谨慎做条件推理",
        ]
    }

    /// 论证严谨
    pub fn rigor(&self) -> Vec<&'static str> {
        vec![
            "关注前提是否成立",
            "推理是否有效",
            "结论是否被证据支持",
            "区分事实与意见",
        ]
    }

    /// 开放求证
    pub fn open(&self) -> Vec<&'static str> {
        vec![
            "欢迎反驳与检验",
            "承认自身认知局限",
            "依据证据更新观点",
            "不固守偏见",
        ]
    }
}

impl Rule for LogicMistakesAvoidRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("logic_mistakes")
    }

    fn explain(&self) -> String {
        format!(
            "【逻辑谬误识别】\n{}",
            [
                format!(
                    "常见谬误：\\n{}",
                    self.fallacies()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "因果辨析：\\n{}",
                    self.causation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "论证严谨：\\n{}",
                    self.rigor()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "开放求证：\\n{}",
                    self.open()
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
    fn test_logicmistakesavoidrules_basic() {
        let rules = LogicMistakesAvoidRules::new();
        assert_eq!(rules.metadata().name, "逻辑谬误识别");
        assert!(!rules.fallacies().is_empty());
        assert!(!rules.causation().is_empty());
        assert!(!rules.rigor().is_empty());
        assert!(!rules.open().is_empty());
    }

    #[test]
    fn test_logicmistakesavoidrules_validation() {
        let rules = LogicMistakesAvoidRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("logic_mistakes"));
    }

    #[test]
    fn test_logicmistakesavoidrules_explain() {
        let rules = LogicMistakesAvoidRules::new();
        let e = rules.explain();
        assert!(e.contains("常见谬误"));
        assert!(e.contains("因果辨析"));
        assert!(e.contains("论证严谨"));
    }
}
