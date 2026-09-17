//! 公交乘车礼仪
//!
//! 公交上下、让座与车厢内的公共礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BusRiderEtiquetteRules,
    name: "公交乘车礼仪",
    desc: "公交上下、让座与车厢内的公共礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "公交", "乘车", "公共交通"]
}

impl BusRiderEtiquetteRules {
    /// 候车与上落
    pub fn boarding(&self) -> Vec<&'static str> {
        vec![
            "在站点排队候车",
            "先下后上不拥挤",
            "有序刷卡投币",
            "停稳再上下车",
        ]
    }

    /// 车厢礼仪
    pub fn carriage(&self) -> Vec<&'static str> {
        vec![
            "主动让座老弱病残孕",
            "控制随身物品不占道",
            "不高声喧哗或看外音",
            "站好扶稳注意安全",
        ]
    }

    /// 文明举止
    pub fn manners(&self) -> Vec<&'static str> {
        vec![
            "不随地吐痰丢物",
            "饮食注意气味",
            "接听电话压低音量",
            "雨天收伞不滴水",
        ]
    }

    /// 下车引导
    pub fn alighting(&self) -> Vec<&'static str> {
        vec![
            "到站前按铃提示",
            "下车前行提前移向门口",
            "不堵在门口堵塞",
            "下车礼让行动不便者",
        ]
    }
}

impl Rule for BusRiderEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("bus_rider")
    }

    fn explain(&self) -> String {
        format!(
            "【公交乘车礼仪】\n{}",
            [
                format!(
                    "候车与上落：\\n{}",
                    self.boarding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "车厢礼仪：\\n{}",
                    self.carriage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "文明举止：\\n{}",
                    self.manners()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "下车引导：\\n{}",
                    self.alighting()
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
    fn test_busrideretiquetterules_basic() {
        let rules = BusRiderEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "公交乘车礼仪");
        assert!(!rules.boarding().is_empty());
        assert!(!rules.carriage().is_empty());
        assert!(!rules.manners().is_empty());
        assert!(!rules.alighting().is_empty());
    }

    #[test]
    fn test_busrideretiquetterules_validation() {
        let rules = BusRiderEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("bus_rider"));
    }

    #[test]
    fn test_busrideretiquetterules_explain() {
        let rules = BusRiderEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("候车与上落"));
        assert!(e.contains("车厢礼仪"));
        assert!(e.contains("文明举止"));
    }
}
