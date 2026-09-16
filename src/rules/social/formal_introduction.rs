//! 社交场合引荐礼仪
//!
//! 正式场合介绍、引荐他人时的次序与称谓礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FormalIntroductionRules,
    name: "社交场合引荐礼仪",
    desc: "正式场合介绍、引荐他人时的次序与称谓礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "引荐", "称谓", "正式场合"]
}

impl FormalIntroductionRules {
    /// 引荐次序
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "先引卑者见尊者",
            "先男子见女子",
            "先年轻者见长者",
            "介绍中方或客人时按其贵宾身份",
        ]
    }

    /// 介绍方法
    pub fn method(&self) -> Vec<&'static str> {
        vec![
            "介绍姓名、职务或关系",
            "音量清晰让人听清",
            "用对方习惯称谓",
            "不确定多音当众询问",
        ]
    }

    /// 握手致意
    pub fn handshake(&self) -> Vec<&'static str> {
        vec![
            "被介绍后主动握手致意",
            "握手适度不过紧过久",
            "目光注视对方微笑",
            "尊者先伸手时再把握",
        ]
    }

    /// 散场与告辞
    pub fn farewell(&self) -> Vec<&'static str> {
        vec![
            "介绍完不冷落新相识",
            "礼貌互留联系方式",
            "离开时与主人道别",
            "下次主动先打招呼",
        ]
    }
}

impl Rule for FormalIntroductionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("formal_intro")
    }

    fn explain(&self) -> String {
        format!(
            "【社交场合引荐礼仪】\n{}",
            [
                format!(
                    "引荐次序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "介绍方法：\\n{}",
                    self.method()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "握手致意：\\n{}",
                    self.handshake()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "散场与告辞：\\n{}",
                    self.farewell()
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
    fn test_formalintroductionrules_basic() {
        let rules = FormalIntroductionRules::new();
        assert_eq!(rules.metadata().name, "社交场合引荐礼仪");
        assert!(!rules.order().is_empty());
        assert!(!rules.method().is_empty());
        assert!(!rules.handshake().is_empty());
        assert!(!rules.farewell().is_empty());
    }

    #[test]
    fn test_formalintroductionrules_validation() {
        let rules = FormalIntroductionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("formal_intro"));
    }

    #[test]
    fn test_formalintroductionrules_explain() {
        let rules = FormalIntroductionRules::new();
        let e = rules.explain();
        assert!(e.contains("引荐次序"));
        assert!(e.contains("介绍方法"));
        assert!(e.contains("握手致意"));
    }
}
