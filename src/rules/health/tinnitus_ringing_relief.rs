//! 耳鸣护听
//!
//! 耳鸣的诱因、缓解与护听注意事项

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TinnitusRingingReliefRules,
    name: "耳鸣护听",
    desc: "耳鸣的诱因、缓解与护听注意事项",
    origin: "医学",
    tags: ["健康", "耳鸣", "听力", "护耳"]
}

impl TinnitusRingingReliefRules {
    /// 认知诱因
    pub fn causes(&self) -> Vec<&'static str> {
        vec![
            "噪音暴露可致耳鸣",
            "疲劳压力易诱发",
            "药物或耳疾相关",
            "血压异常可能相关",
        ]
    }

    /// 护听习惯
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "远离持续高分贝环境",
            "必要时佩戴护听器",
            "控制耳机音量和时长",
            "避免用硬物掏耳",
        ]
    }

    /// 缓解应对
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "放松身心减轻紧张",
            "白噪音轻放掩盖",
            "规律作息充足睡眠",
            "不因耳鸣过度焦虑",
        ]
    }

    /// 就医提示
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "突发或单侧耳鸣就医",
            "伴眩晕听力下降检查",
            "耳鸣持续影响生活就诊",
            "规范检查排除耳疾",
        ]
    }
}

impl Rule for TinnitusRingingReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("tinnitus")
    }

    fn explain(&self) -> String {
        format!(
            "【耳鸣护听】\n{}",
            [
                format!(
                    "认知诱因：\\n{}",
                    self.causes()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "护听习惯：\\n{}",
                    self.protect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "缓解应对：\\n{}",
                    self.relief()
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
    fn test_tinnitusringingreliefrules_basic() {
        let rules = TinnitusRingingReliefRules::new();
        assert_eq!(rules.metadata().name, "耳鸣护听");
        assert!(!rules.causes().is_empty());
        assert!(!rules.protect().is_empty());
        assert!(!rules.relief().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_tinnitusringingreliefrules_validation() {
        let rules = TinnitusRingingReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("tinnitus"));
    }

    #[test]
    fn test_tinnitusringingreliefrules_explain() {
        let rules = TinnitusRingingReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("认知诱因"));
        assert!(e.contains("护听习惯"));
        assert!(e.contains("缓解应对"));
    }
}
