//! 足底筋膜炎护理
//!
//! 足跟疼痛的足底筋膜劳损缓解与康复

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PlantarFasciitisCareRules,
    name: "足底筋膜炎护理",
    desc: "足跟疼痛的足底筋膜劳损缓解与康复",
    origin: "中国",
    tags: ["骨骼", "足部", "康复"]
}

impl PlantarFasciitisCareRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "晨起下床足跟刺痛",
            "走动片刻后减轻",
            "久站久走后加重",
            "足底内侧压痛",
        ]
    }

    /// 急性缓解
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "冰敷滚动按摩足底",
            "减少久站与长距离行走",
            "抬高患足休息",
            "避免赤脚硬地行走",
        ]
    }

    /// 牵伸锻炼
    pub fn stretching(&self) -> Vec<&'static str> {
        vec![
            "足底筋膜牵伸",
            "小腿腓肠肌拉伸",
            "起床前先做牵伸",
            "循序渐进持之以恒",
        ]
    }

    /// 鞋具调整
    pub fn footwear(&self) -> Vec<&'static str> {
        vec![
            "选有足弓支撑的鞋",
            "加软垫减轻足跟压力",
            "避免平底硬底鞋",
            "控制体重减少负荷",
        ]
    }
}

impl Rule for PlantarFasciitisCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("plantar_fasciitis_care")
    }

    fn explain(&self) -> String {
        format!(
            "【足底筋膜炎护理】\n{}",
            [
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "急性缓解：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "牵伸锻炼：\\n{}",
                    self.stretching()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "鞋具调整：\\n{}",
                    self.footwear()
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
    fn test_plantarfasciitiscarerules_basic() {
        let rules = PlantarFasciitisCareRules::new();
        assert_eq!(rules.metadata().name, "足底筋膜炎护理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.relief().is_empty());
        assert!(!rules.stretching().is_empty());
        assert!(!rules.footwear().is_empty());
    }

    #[test]
    fn test_plantarfasciitiscarerules_validation() {
        let rules = PlantarFasciitisCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("plantar_fasciitis_care")
        );
    }

    #[test]
    fn test_plantarfasciitiscarerules_explain() {
        let rules = PlantarFasciitisCareRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("急性缓解"));
        assert!(e.contains("牵伸锻炼"));
    }
}
