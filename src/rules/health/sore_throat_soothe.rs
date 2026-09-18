//! 嗓子干痛舒缓
//!
//! 咽喉干痒疼痛的舒缓与护理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SoreThroatSootheRules,
    name: "嗓子干痛舒缓",
    desc: "咽喉干痒疼痛的舒缓与护理",
    origin: "医学",
    tags: ["健康", "咽喉", "疼痛", "护理"]
}

impl SoreThroatSootheRules {
    /// 润喉补水
    pub fn moisturize(&self) -> Vec<&'static str> {
        vec!["温水频饮", "蜜柠檬缓", "喉糖润喉", "缓解干痒"]
    }

    /// 少刺激
    pub fn avoid(&self) -> Vec<&'static str> {
        vec!["禁辛辣烟酒", "少大声费喉", "温软食物", "避免烫饮"]
    }

    /// 环境加湿
    pub fn humidify(&self) -> Vec<&'static str> {
        vec!["空气加湿", "开窗通风", "湿润喉咙", "减少干痛"]
    }

    /// 就医判断
    pub fn visit(&self) -> Vec<&'static str> {
        vec!["持续剧痛就医", "伴随高热炎症", "口腔异味找因", "对症治疗"]
    }
}

impl Rule for SoreThroatSootheRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sore_throat")
    }

    fn explain(&self) -> String {
        format!(
            "【嗓子干痛舒缓】\n{}",
            [
                format!(
                    "润喉补水：\\n{}",
                    self.moisturize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "少刺激：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环境加湿：\\n{}",
                    self.humidify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医判断：\\n{}",
                    self.visit()
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
    fn test_sorethroatsootherules_basic() {
        let rules = SoreThroatSootheRules::new();
        assert_eq!(rules.metadata().name, "嗓子干痛舒缓");
        assert!(!rules.moisturize().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.humidify().is_empty());
        assert!(!rules.visit().is_empty());
    }

    #[test]
    fn test_sorethroatsootherules_validation() {
        let rules = SoreThroatSootheRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("sore_throat"));
    }

    #[test]
    fn test_sorethroatsootherules_explain() {
        let rules = SoreThroatSootheRules::new();
        let e = rules.explain();
        assert!(e.contains("润喉补水"));
        assert!(e.contains("少刺激"));
        assert!(e.contains("环境加湿"));
    }
}
