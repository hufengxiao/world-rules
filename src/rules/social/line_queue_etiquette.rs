//! 排队秩序礼仪
//!
//! 公共场所排队、守序与让行的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LineQueueEtiquetteRules,
    name: "排队秩序礼仪",
    desc: "公共场所排队、守序与让行的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "排队", "秩序"]
}

impl LineQueueEtiquetteRules {
    /// 自觉排队
    pub fn wait(&self) -> Vec<&'static str> {
        vec![
            "按先后顺序排队",
            "认清队尾不插队",
            "不挤占他人位置",
            "耐心等待不催促",
        ]
    }

    /// 礼让老幼
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "让老人孕妇先行",
            "体谅行动不便者",
            "有急事礼貌说明",
            "得照顾要道谢",
        ]
    }

    /// 守规有序
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "遵守一人一位",
            "不代排请托乱序",
            "保持间隔不贴靠",
            "离开队伍重排或说明",
        ]
    }

    /// 文明氛围
    pub fn manner(&self) -> Vec<&'static str> {
        vec!["不乱穿队扰乱", "不大声喧哗", "维护公共秩序", "互相体谅礼让"]
    }
}

impl Rule for LineQueueEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("line_queue")
    }

    fn explain(&self) -> String {
        format!(
            "【排队秩序礼仪】\n{}",
            [
                format!(
                    "自觉排队：\\n{}",
                    self.wait()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼让老幼：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "守规有序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "文明氛围：\\n{}",
                    self.manner()
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
    fn test_linequeueetiquetterules_basic() {
        let rules = LineQueueEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "排队秩序礼仪");
        assert!(!rules.wait().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.order().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_linequeueetiquetterules_validation() {
        let rules = LineQueueEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("line_queue"));
    }

    #[test]
    fn test_linequeueetiquetterules_explain() {
        let rules = LineQueueEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("自觉排队"));
        assert!(e.contains("礼让老幼"));
        assert!(e.contains("守规有序"));
    }
}
