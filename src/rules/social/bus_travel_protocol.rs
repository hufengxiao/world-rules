//! 公交出行礼仪
//!
//! 公交车上下门、让座与乘车的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BusTravelProtocolRules,
    name: "公交出行礼仪",
    desc: "公交车上下门、让座与乘车的礼仪",
    origin: "中国",
    tags: ["社交", "公交", "出行", "礼仪"]
}

impl BusTravelProtocolRules {
    /// 候车守规
    pub fn waiting(&self) -> Vec<&'static str> {
        vec!["站台有序等候", "先下后上", "排队依次", "不追逼车"]
    }

    /// 刷卡上车
    pub fn boarding(&self) -> Vec<&'static str> {
        vec!["从前门刷卡上车", "备好零钱或卡", "往里挪不挤门", "拉好扶手"]
    }

    /// 让座互助
    pub fn yielding(&self) -> Vec<&'static str> {
        vec![
            "老弱孕残让座",
            "他人让座致谢",
            "背包放下不占地",
            "不一人占多位",
        ]
    }

    /// 礼貌乘车
    pub fn quiet(&self) -> Vec<&'static str> {
        vec!["车厢内安静", "接电话低声", "不倚靠贴人", "到站前按铃"]
    }
}

impl Rule for BusTravelProtocolRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("bus_protocol")
    }

    fn explain(&self) -> String {
        format!(
            "【公交出行礼仪】\n{}",
            [
                format!(
                    "候车守规：\\n{}",
                    self.waiting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "刷卡上车：\\n{}",
                    self.boarding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "让座互助：\\n{}",
                    self.yielding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼貌乘车：\\n{}",
                    self.quiet()
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
    fn test_bustravelprotocolrules_basic() {
        let rules = BusTravelProtocolRules::new();
        assert_eq!(rules.metadata().name, "公交出行礼仪");
        assert!(!rules.waiting().is_empty());
        assert!(!rules.boarding().is_empty());
        assert!(!rules.yielding().is_empty());
        assert!(!rules.quiet().is_empty());
    }

    #[test]
    fn test_bustravelprotocolrules_validation() {
        let rules = BusTravelProtocolRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("bus_protocol"));
    }

    #[test]
    fn test_bustravelprotocolrules_explain() {
        let rules = BusTravelProtocolRules::new();
        let e = rules.explain();
        assert!(e.contains("候车守规"));
        assert!(e.contains("刷卡上车"));
        assert!(e.contains("让座互助"));
    }
}
