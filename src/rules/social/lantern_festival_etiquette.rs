//! 元宵节礼仪
//!
//! 元宵赏灯猜谜吃汤圆的节庆礼节与团圆讲究

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LanternFestivalEtiquetteRules,
    name: "元宵节礼仪",
    desc: "元宵赏灯猜谜吃汤圆的节庆礼节与团圆讲究",
    origin: "中国",
    tags: ["节庆", "传统", "团圆"]
}

impl LanternFestivalEtiquetteRules {
    /// 节庆习俗
    pub fn customs(&self) -> Vec<&'static str> {
        vec![
            "正月十五赏花灯",
            "猜灯谜添雅趣",
            "吃汤圆寓团圆",
            "舞龙舞狮助兴",
        ]
    }

    /// 阖家团圆
    pub fn reunion(&self) -> Vec<&'static str> {
        vec![
            "家人团聚共吃元宵",
            "长辈为小辈备灯",
            "互道节日祝福",
            "共享天伦之乐",
        ]
    }

    /// 观灯礼仪
    pub fn viewing(&self) -> Vec<&'static str> {
        vec![
            "人多处礼让不推挤",
            "照看老人小孩",
            "不攀爬灯架",
            "爱护花灯设施",
        ]
    }

    /// 祝福往来
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "向亲友送上节日问候",
            "祝福团圆美满",
            "送汤圆寓意甜蜜",
            "礼轻情意重",
        ]
    }
}

impl Rule for LanternFestivalEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("lantern_festival_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【元宵节礼仪】\n{}",
            [
                format!(
                    "节庆习俗：\\n{}",
                    self.customs()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "阖家团圆：\\n{}",
                    self.reunion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观灯礼仪：\\n{}",
                    self.viewing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "祝福往来：\\n{}",
                    self.greetings()
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
    fn test_lanternfestivaletiquetterules_basic() {
        let rules = LanternFestivalEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "元宵节礼仪");
        assert!(!rules.customs().is_empty());
        assert!(!rules.reunion().is_empty());
        assert!(!rules.viewing().is_empty());
        assert!(!rules.greetings().is_empty());
    }

    #[test]
    fn test_lanternfestivaletiquetterules_validation() {
        let rules = LanternFestivalEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("lantern_festival_etiquette")
        );
    }

    #[test]
    fn test_lanternfestivaletiquetterules_explain() {
        let rules = LanternFestivalEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("节庆习俗"));
        assert!(e.contains("阖家团圆"));
        assert!(e.contains("观灯礼仪"));
    }
}
