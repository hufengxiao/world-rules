//! 耳垢护理
//!
//! 耳垢的正常处理与耳道清洁禁忌

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EarWaxCareRules,
    name: "耳垢护理",
    desc: "耳垢的正常处理与耳道清洁禁忌",
    origin: "中国",
    tags: ["五官", "耳道", "护理"]
}

impl EarWaxCareRules {
    /// 正确认识
    pub fn understanding(&self) -> Vec<&'static str> {
        vec![
            "耳垢有保护耳道作用",
            "多可随咀嚼自行排出",
            "并非越干净越好",
            "无需频繁清理",
        ]
    }

    /// 清理禁忌
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "勿用棉签深入掏耳",
            "勿用发卡牙签等硬物",
            "勿过度频繁掏耳",
            "避免用力捅伤鼓膜",
        ]
    }

    /// 日常护理
    pub fn daily_care(&self) -> Vec<&'static str> {
        vec![
            "外耳廓用湿巾擦拭即可",
            "洗澡后倾头排出积水",
            "游泳戴耳塞防进水",
            "保持耳道干燥",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "耳垢堵塞听力下降就医",
            "耳痛耳闷应及时就诊",
            "耳道流液流脓须就医",
            "由专业人员取出耳垢",
        ]
    }
}

impl Rule for EarWaxCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("ear_wax_care")
    }

    fn explain(&self) -> String {
        format!(
            "【耳垢护理】\n{}",
            [
                format!(
                    "正确认识：\\n{}",
                    self.understanding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "清理禁忌：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常护理：\\n{}",
                    self.daily_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seeking_help()
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
    fn test_earwaxcarerules_basic() {
        let rules = EarWaxCareRules::new();
        assert_eq!(rules.metadata().name, "耳垢护理");
        assert!(!rules.understanding().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.daily_care().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_earwaxcarerules_validation() {
        let rules = EarWaxCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("ear_wax_care"));
    }

    #[test]
    fn test_earwaxcarerules_explain() {
        let rules = EarWaxCareRules::new();
        let e = rules.explain();
        assert!(e.contains("正确认识"));
        assert!(e.contains("清理禁忌"));
        assert!(e.contains("日常护理"));
    }
}
