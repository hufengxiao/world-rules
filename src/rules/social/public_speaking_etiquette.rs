//! 公开演讲礼仪
//!
//! 会议、致辞等公开发言中的准备、表达与听众礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PublicSpeakingEtiquetteRules,
    name: "公开演讲礼仪",
    desc: "会议、致辞等公开发言中的准备、表达与听众礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "演讲", "发言", "表达"]
}

impl PublicSpeakingEtiquetteRules {
    /// 会前准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "明确听众与场合基调",
            "精心准备讲稿与要旨",
            "掌握发言时长",
            "提前熟悉设备讲台",
        ]
    }

    /// 表达呈现
    pub fn delivery(&self) -> Vec<&'static str> {
        vec![
            "开场问候与自我介绍",
            "语速适中吐字清晰",
            "目光与人交流",
            "内容条理紧扣主题",
        ]
    }

    /// 结尾致谢
    pub fn closing(&self) -> Vec<&'static str> {
        vec![
            "总结要点收束发言",
            "真诚感谢听众与主办",
            "回应提问得体",
            "不随意超时拖沓",
        ]
    }

    /// 听众礼仪
    pub fn audience(&self) -> Vec<&'static str> {
        vec![
            "安静聆听不交头接耳",
            "手机静音不影响",
            "适时鼓掌",
            "提问遵循次序先举手",
        ]
    }
}

impl Rule for PublicSpeakingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("public_speaking")
    }

    fn explain(&self) -> String {
        format!(
            "【公开演讲礼仪】\n{}",
            [
                format!(
                    "会前准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "表达呈现：\\n{}",
                    self.delivery()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结尾致谢：\\n{}",
                    self.closing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "听众礼仪：\\n{}",
                    self.audience()
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
    fn test_publicspeakingetiquetterules_basic() {
        let rules = PublicSpeakingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "公开演讲礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.delivery().is_empty());
        assert!(!rules.closing().is_empty());
        assert!(!rules.audience().is_empty());
    }

    #[test]
    fn test_publicspeakingetiquetterules_validation() {
        let rules = PublicSpeakingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("public_speaking"));
    }

    #[test]
    fn test_publicspeakingetiquetterules_explain() {
        let rules = PublicSpeakingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("会前准备"));
        assert!(e.contains("表达呈现"));
        assert!(e.contains("结尾致谢"));
    }
}
