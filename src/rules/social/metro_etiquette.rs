//! 地铁乘行礼仪
//!
//! 地铁候车、让座与上下车的公共礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MetroEtiquetteRules,
    name: "地铁乘行礼仪",
    desc: "地铁候车、让座与上下车的公共礼仪",
    origin: "中国",
    tags: ["社交", "地铁", "礼仪", "候车"]
}

impl MetroEtiquetteRules {
    /// 候车有序
    pub fn waiting(&self) -> Vec<&'static str> {
        vec!["站台黄线外候车", "先下后上", "不抢门不推挤", "依秩序排队"]
    }

    /// 上下车厢
    pub fn boarding(&self) -> Vec<&'static str> {
        vec!["门开侧身进出", "往里走让门", "不堵门口", "随身物品不占位"]
    }

    /// 车厢礼仪
    pub fn manner(&self) -> Vec<&'static str> {
        vec!["安静不喧哗", "耳机不外放", "不跷腿乱放", "女士优先轻声"]
    }

    /// 主动让座
    pub fn yielding(&self) -> Vec<&'static str> {
        vec!["给老弱让座", "孕妇残障优先", "凭需要真诚", "受让致谢"]
    }
}

impl Rule for MetroEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("metro")
    }

    fn explain(&self) -> String {
        format!(
            "【地铁乘行礼仪】\n{}",
            [
                format!(
                    "候车有序：\\n{}",
                    self.waiting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "上下车厢：\\n{}",
                    self.boarding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "车厢礼仪：\\n{}",
                    self.manner()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "主动让座：\\n{}",
                    self.yielding()
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
    fn test_metroetiquetterules_basic() {
        let rules = MetroEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "地铁乘行礼仪");
        assert!(!rules.waiting().is_empty());
        assert!(!rules.boarding().is_empty());
        assert!(!rules.manner().is_empty());
        assert!(!rules.yielding().is_empty());
    }

    #[test]
    fn test_metroetiquetterules_validation() {
        let rules = MetroEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("metro"));
    }

    #[test]
    fn test_metroetiquetterules_explain() {
        let rules = MetroEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("候车有序"));
        assert!(e.contains("上下车厢"));
        assert!(e.contains("车厢礼仪"));
    }
}
