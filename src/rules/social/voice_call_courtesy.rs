//! 语音通话礼仪
//!
//! 语音通话的时机、开场与长篇通话分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VoiceCallCourtesyRules,
    name: "语音通话礼仪",
    desc: "语音通话的时机、开场与长篇通话分寸",
    origin: "国际",
    tags: ["社交", "礼仪", "通话", "语音"]
}

impl VoiceCallCourtesyRules {
    /// 来电时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "白天合理时段通电话",
            "避免深夜清晨打扰",
            "开会休息前先请示",
            "紧急情况例外",
        ]
    }

    /// 开场问候
    pub fn greet(&self) -> Vec<&'static str> {
        vec![
            "接通先自报姓名",
            "询问对方是否方便",
            "简洁说明来电目的",
            "音量语调适度",
        ]
    }

    /// 通话分寸
    pub fn measure(&self) -> Vec<&'static str> {
        vec![
            "说话简洁含礼",
            "不长时间占线啰嗦",
            "重要内容复述确认",
            "环境安静不嘈杂",
        ]
    }

    /// 结束道别
    pub fn end(&self) -> Vec<&'static str> {
        vec![
            "确认说完再挂机",
            "说声谢谢/再见",
            "礼貌结束通话",
            "挂前留机会让对方先说",
        ]
    }
}

impl Rule for VoiceCallCourtesyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("voice_call")
    }

    fn explain(&self) -> String {
        format!(
            "【语音通话礼仪】\n{}",
            [
                format!(
                    "来电时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "开场问候：\\n{}",
                    self.greet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "通话分寸：\\n{}",
                    self.measure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结束道别：\\n{}",
                    self.end()
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
    fn test_voicecallcourtesyrules_basic() {
        let rules = VoiceCallCourtesyRules::new();
        assert_eq!(rules.metadata().name, "语音通话礼仪");
        assert!(!rules.timing().is_empty());
        assert!(!rules.greet().is_empty());
        assert!(!rules.measure().is_empty());
        assert!(!rules.end().is_empty());
    }

    #[test]
    fn test_voicecallcourtesyrules_validation() {
        let rules = VoiceCallCourtesyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("voice_call"));
    }

    #[test]
    fn test_voicecallcourtesyrules_explain() {
        let rules = VoiceCallCourtesyRules::new();
        let e = rules.explain();
        assert!(e.contains("来电时机"));
        assert!(e.contains("开场问候"));
        assert!(e.contains("通话分寸"));
    }
}
