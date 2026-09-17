//! 扶梯与楼梯礼仪
//!
//! 乘坐扶梯、走楼梯时的让行与安全礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EscalatorEtiquetteRules,
    name: "扶梯与楼梯礼仪",
    desc: "乘坐扶梯、走楼梯时的让行与安全礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "扶梯", "楼梯", "安全"]
}

impl EscalatorEtiquetteRules {
    /// 扶梯站立
    pub fn standing(&self) -> Vec<&'static str> {
        vec![
            "站稳扶手靠一侧",
            "如需行走走另一侧",
            "老人儿童抓紧扶手",
            "不占两侧挡道",
        ]
    }

    /// 通行有序
    pub fn passing(&self) -> Vec<&'static str> {
        vec![
            "不迎面上行逆行",
            "通行时轻让不急跑",
            "照顾行李与推车",
            "不行人并排阻挡",
        ]
    }

    /// 上下安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "进出时观察来向",
            "不嬉戏追跑",
            "严扶扶手防跌倒",
            "携带重物轻慢行",
        ]
    }

    /// 楼梯让行
    pub fn stair(&self) -> Vec<&'static str> {
        vec![
            "靠右行走留左侧通行",
            "遇老人放慢让行",
            "不拥挤争道",
            "注意脚下灯昏暗",
        ]
    }
}

impl Rule for EscalatorEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("escalator")
    }

    fn explain(&self) -> String {
        format!(
            "【扶梯与楼梯礼仪】\n{}",
            [
                format!(
                    "扶梯站立：\\n{}",
                    self.standing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "通行有序：\\n{}",
                    self.passing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "上下安全：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "楼梯让行：\\n{}",
                    self.stair()
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
    fn test_escalatoretiquetterules_basic() {
        let rules = EscalatorEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "扶梯与楼梯礼仪");
        assert!(!rules.standing().is_empty());
        assert!(!rules.passing().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.stair().is_empty());
    }

    #[test]
    fn test_escalatoretiquetterules_validation() {
        let rules = EscalatorEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("escalator"));
    }

    #[test]
    fn test_escalatoretiquetterules_explain() {
        let rules = EscalatorEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("扶梯站立"));
        assert!(e.contains("通行有序"));
        assert!(e.contains("上下安全"));
    }
}
