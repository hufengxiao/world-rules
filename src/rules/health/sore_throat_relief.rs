//! 喉咙痛缓解
//!
//! 咽痛喉咙不适的缓解护理与就医提示

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SoreThroatReliefRules,
    name: "喉咙痛缓解",
    desc: "咽痛喉咙不适的缓解护理与就医提示",
    origin: "医学",
    tags: ["健康", "喉咙", "咽痛", "护理"]
}

impl SoreThroatReliefRules {
    /// 日常缓解
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "多饮温水润喉",
            "用温盐水漱口",
            "温润食物减轻刺激",
            "保持口鼻湿润",
        ]
    }

    /// 护嗓习惯
    pub fn voice_care(&self) -> Vec<&'static str> {
        vec![
            "避免大声喊叫伤咽",
            "减少烟酒刺激",
            "少食辛辣过烫",
            "不强行清嗓",
        ]
    }

    /// 休息调养
    pub fn rest(&self) -> Vec<&'static str> {
        vec![
            "保证充足休息睡眠",
            "注意防寒保暖",
            "多喝水助恢复",
            "观察体温变化",
        ]
    }

    /// 何时就医
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "吞咽困难或气促就医",
            "高热或颈部肿痛需检查",
            "声音嘶哑持续超过两周就诊",
            "怀疑链球菌感染就医评估",
        ]
    }
}

impl Rule for SoreThroatReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sore_throat")
    }

    fn explain(&self) -> String {
        format!(
            "【喉咙痛缓解】\n{}",
            [
                format!(
                    "日常缓解：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "护嗓习惯：\\n{}",
                    self.voice_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "休息调养：\\n{}",
                    self.rest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "何时就医：\\n{}",
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
    fn test_sorethroatreliefrules_basic() {
        let rules = SoreThroatReliefRules::new();
        assert_eq!(rules.metadata().name, "喉咙痛缓解");
        assert!(!rules.relief().is_empty());
        assert!(!rules.voice_care().is_empty());
        assert!(!rules.rest().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_sorethroatreliefrules_validation() {
        let rules = SoreThroatReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("sore_throat"));
    }

    #[test]
    fn test_sorethroatreliefrules_explain() {
        let rules = SoreThroatReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("日常缓解"));
        assert!(e.contains("护嗓习惯"));
        assert!(e.contains("休息调养"));
    }
}
