//! 克服拖延
//!
//! 认识拖延成因、任务拆解与行动的方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ProcrastinationManageRules,
    name: "克服拖延",
    desc: "认识拖延成因、任务拆解与行动的方法",
    origin: "心理学",
    tags: ["健康", "拖延", "行动", "效率"]
}

impl ProcrastinationManageRules {
    /// 认识拖延
    pub fn understand(&self) -> Vec<&'static str> {
        vec![
            "识别拖延背后的畏惧",
            "分清任务轻重缓急",
            "不因追求完美卡壳",
            "承认拖延并不丢人",
        ]
    }

    /// 拆解任务
    pub fn breakdown(&self) -> Vec<&'static str> {
        vec![
            "把大任务分成小步",
            "先做最容易的一步",
            "设定明确小目标",
            "完成的就及时肯定",
        ]
    }

    /// 启动行动
    pub fn start(&self) -> Vec<&'static str> {
        vec![
            "用定时专注法开启",
            "不被杂念分心",
            "先做五分钟再说",
            "减少手机干扰",
        ]
    }

    /// 长期坚持
    pub fn sustain(&self) -> Vec<&'static str> {
        vec![
            "计划现实有弹性",
            "复盘改进不苛责",
            "寻求督促支持",
            "习惯养成需时间",
        ]
    }
}

impl Rule for ProcrastinationManageRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("procrastination")
    }

    fn explain(&self) -> String {
        format!(
            "【克服拖延】\n{}",
            [
                format!(
                    "认识拖延：\\n{}",
                    self.understand()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "拆解任务：\\n{}",
                    self.breakdown()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "启动行动：\\n{}",
                    self.start()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "长期坚持：\\n{}",
                    self.sustain()
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
    fn test_procrastinationmanagerules_basic() {
        let rules = ProcrastinationManageRules::new();
        assert_eq!(rules.metadata().name, "克服拖延");
        assert!(!rules.understand().is_empty());
        assert!(!rules.breakdown().is_empty());
        assert!(!rules.start().is_empty());
        assert!(!rules.sustain().is_empty());
    }

    #[test]
    fn test_procrastinationmanagerules_validation() {
        let rules = ProcrastinationManageRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("procrastination"));
    }

    #[test]
    fn test_procrastinationmanagerules_explain() {
        let rules = ProcrastinationManageRules::new();
        let e = rules.explain();
        assert!(e.contains("认识拖延"));
        assert!(e.contains("拆解任务"));
        assert!(e.contains("启动行动"));
    }
}
