//! 情绪调节方法
//!
//! 觉察与调节情绪的日常方法与表达规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EmotionalRegulationRules,
    name: "情绪调节方法",
    desc: "觉察与调节情绪的日常方法与表达规则",
    origin: "心理学",
    tags: ["健康", "情绪", "调节", "心理健康"]
}

impl EmotionalRegulationRules {
    /// 觉察情绪
    pub fn awareness(&self) -> Vec<&'static str> {
        vec![
            "先识别当下的情绪名称",
            "不急于压制或否定",
            "观察情绪引发的身体反应",
            "给情绪适当的空间",
        ]
    }

    /// 健康表达
    pub fn expression(&self) -> Vec<&'static str> {
        vec![
            "用语言描述自己的感受",
            "以我开头表达而非指责",
            "不压抑也不情绪化发泄",
            "选择合适时机沟通",
        ]
    }

    /// 调节策略
    pub fn strategies(&self) -> Vec<&'static str> {
        vec![
            "暂停争执先冷静片刻",
            "转移注意做感兴趣的事",
            "用腹式呼吸平复激动",
            "适度运动释放负能",
        ]
    }

    /// 长期培养
    pub fn growth(&self) -> Vec<&'static str> {
        vec![
            "练习感恩与正念",
            "建立稳定支持关系",
            "接纳情绪波动是正常",
            "严重的情绪困扰及时求助",
        ]
    }
}

impl Rule for EmotionalRegulationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("emotional_regulation")
    }

    fn explain(&self) -> String {
        format!(
            "【情绪调节方法】\n{}",
            [
                format!(
                    "觉察情绪：\\n{}",
                    self.awareness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健康表达：\\n{}",
                    self.expression()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "调节策略：\\n{}",
                    self.strategies()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "长期培养：\\n{}",
                    self.growth()
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
    fn test_emotionalregulationrules_basic() {
        let rules = EmotionalRegulationRules::new();
        assert_eq!(rules.metadata().name, "情绪调节方法");
        assert!(!rules.awareness().is_empty());
        assert!(!rules.expression().is_empty());
        assert!(!rules.strategies().is_empty());
        assert!(!rules.growth().is_empty());
    }

    #[test]
    fn test_emotionalregulationrules_validation() {
        let rules = EmotionalRegulationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("emotional_regulation")
        );
    }

    #[test]
    fn test_emotionalregulationrules_explain() {
        let rules = EmotionalRegulationRules::new();
        let e = rules.explain();
        assert!(e.contains("觉察情绪"));
        assert!(e.contains("健康表达"));
        assert!(e.contains("调节策略"));
    }
}
