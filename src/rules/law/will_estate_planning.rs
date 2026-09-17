//! 遗嘱与遗产规划
//!
//! 立遗嘱、遗产分配与法定继承的常识要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WillEstatePlanningRules,
    name: "遗嘱与遗产规划",
    desc: "立遗嘱、遗产分配与法定继承的常识要点",
    origin: "中国",
    tags: ["法律", "遗嘱", "遗产", "继承"]
}

impl WillEstatePlanningRules {
    /// 法定继承
    pub fn statutory(&self) -> Vec<&'static str> {
        vec![
            "了解法定继承人顺序",
            "配偶子女父母等属第一顺序",
            "无遗嘱时按法定分配",
            "遗赠扶养另有规定",
        ]
    }

    /// 遗嘱形式
    pub fn form(&self) -> Vec<&'static str> {
        vec![
            "遗嘱形式须符合法律规定",
            "自书遗嘱亲笔签名等",
            "代书见证等按规办理",
            "遗嘱内容应真实自愿",
        ]
    }

    /// 合理规划
    pub fn plan(&self) -> Vec<&'static str> {
        vec![
            "适时订立明确遗嘱",
            "妥善保管遗嘱文件",
            "可附个人安排说明",
            "避免无法定形式争议",
        ]
    }

    /// 善后处理
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "依法处理遗产分割",
            "尊重被继承人意愿",
            "必要时请法律评估",
            "遵从遗嘱公平解决",
        ]
    }
}

impl Rule for WillEstatePlanningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("will_estate")
    }

    fn explain(&self) -> String {
        format!(
            "【遗嘱与遗产规划】\n{}",
            [
                format!(
                    "法定继承：\\n{}",
                    self.statutory()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "遗嘱形式：\\n{}",
                    self.form()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合理规划：\\n{}",
                    self.plan()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "善后处理：\\n{}",
                    self.after()
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
    fn test_willestateplanningrules_basic() {
        let rules = WillEstatePlanningRules::new();
        assert_eq!(rules.metadata().name, "遗嘱与遗产规划");
        assert!(!rules.statutory().is_empty());
        assert!(!rules.form().is_empty());
        assert!(!rules.plan().is_empty());
        assert!(!rules.after().is_empty());
    }

    #[test]
    fn test_willestateplanningrules_validation() {
        let rules = WillEstatePlanningRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("will_estate"));
    }

    #[test]
    fn test_willestateplanningrules_explain() {
        let rules = WillEstatePlanningRules::new();
        let e = rules.explain();
        assert!(e.contains("法定继承"));
        assert!(e.contains("遗嘱形式"));
        assert!(e.contains("合理规划"));
    }
}
