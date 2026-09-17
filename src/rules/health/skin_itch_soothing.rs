//! 皮肤瘙痒舒缓
//!
//! 皮肤发痒的舒缓护理、避免抓伤与健康护理由

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SkinItchSoothingRules,
    name: "皮肤瘙痒舒缓",
    desc: "皮肤发痒的舒缓护理、避免抓伤与健康护理由",
    origin: "医学",
    tags: ["健康", "皮肤", "瘙痒", "护理"]
}

impl SkinItchSoothingRules {
    /// 舒缓瘙痒
    pub fn soothe(&self) -> Vec<&'static str> {
        vec![
            "冷敷或用温水轻洗",
            "保持皮肤清润保湿",
            "遵医嘱使用止痒软膏",
            "尽量不挠抓皮肤",
        ]
    }

    /// 避免刺激
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "选温和无香清洁用品",
            "穿宽松棉质透气衣物",
            "热水澡少泡过久",
            "减少过敏原接触",
        ]
    }

    /// 日常保湿
    pub fn moisturize(&self) -> Vec<&'static str> {
        vec![
            "洗澡后及时涂保湿乳",
            "保持室内湿度适宜",
            "手部勤用护手乳",
            "注意防晒减少损伤",
        ]
    }

    /// 就医提示
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "瘙痒伴明显皮疹就医",
            "抓破感染红肿需处理",
            "影响睡眠或持续加重就医",
            "全身性或长期瘙痒评估",
        ]
    }
}

impl Rule for SkinItchSoothingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("skin_itch")
    }

    fn explain(&self) -> String {
        format!(
            "【皮肤瘙痒舒缓】\n{}",
            [
                format!(
                    "舒缓瘙痒：\\n{}",
                    self.soothe()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "避免刺激：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常保湿：\\n{}",
                    self.moisturize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seek()
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
    fn test_skinitchsoothingrules_basic() {
        let rules = SkinItchSoothingRules::new();
        assert_eq!(rules.metadata().name, "皮肤瘙痒舒缓");
        assert!(!rules.soothe().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.moisturize().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_skinitchsoothingrules_validation() {
        let rules = SkinItchSoothingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("skin_itch"));
    }

    #[test]
    fn test_skinitchsoothingrules_explain() {
        let rules = SkinItchSoothingRules::new();
        let e = rules.explain();
        assert!(e.contains("舒缓瘙痒"));
        assert!(e.contains("避免刺激"));
        assert!(e.contains("日常保湿"));
    }
}
