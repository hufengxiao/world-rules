//! 垂钓礼仪
//!
//! 野钓、休闲垂钓时的互相尊重、环保与安全礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FishingEtiquetteRules,
    name: "垂钓礼仪",
    desc: "野钓、休闲垂钓时的互相尊重、环保与安全礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "垂钓", "钓鱼", "水边"]
}

impl FishingEtiquetteRules {
    /// 择位与间距
    pub fn spacing(&self) -> Vec<&'static str> {
        vec![
            "与人钓位保持安全距离",
            "不抢占他人占好的窝点",
            "observer垂钓指引时尊重",
            "钓具互让不缠线吵闹",
        ]
    }

    /// 环保放流
    pub fn release(&self) -> Vec<&'static str> {
        vec![
            "小鱼或不食取者放流",
            "遵守禁渔期与渔获规格",
            "不用违禁渔具下药",
            "垃圾随钓随收",
        ]
    }

    /// 安静与安全
    pub fn quiet(&self) -> Vec<&'static str> {
        vec![
            "提竿时轻声提示邻近",
            "不大声喧哗惊鱼",
            "甩竿前观察身后有人",
            "水边留意水深与水流",
        ]
    }

    /// 人与水鸟
    pub fn wildlife(&self) -> Vec<&'static str> {
        vec![
            "不惊扰水鸟与鱼群过度",
            "渔获不随意丢弃死鱼",
            "爱护水域生态",
            "遵守当地钓鱼规定",
        ]
    }
}

impl Rule for FishingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("fishing")
    }

    fn explain(&self) -> String {
        format!(
            "【垂钓礼仪】\n{}",
            [
                format!(
                    "择位与间距：\\n{}",
                    self.spacing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环保放流：\\n{}",
                    self.release()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安静与安全：\\n{}",
                    self.quiet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "人与水鸟：\\n{}",
                    self.wildlife()
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
    fn test_fishingetiquetterules_basic() {
        let rules = FishingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "垂钓礼仪");
        assert!(!rules.spacing().is_empty());
        assert!(!rules.release().is_empty());
        assert!(!rules.quiet().is_empty());
        assert!(!rules.wildlife().is_empty());
    }

    #[test]
    fn test_fishingetiquetterules_validation() {
        let rules = FishingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("fishing"));
    }

    #[test]
    fn test_fishingetiquetterules_explain() {
        let rules = FishingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("择位与间距"));
        assert!(e.contains("环保放流"));
        assert!(e.contains("安静与安全"));
    }
}
