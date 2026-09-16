//! 毕业典礼礼仪
//!
//! 毕业典礼、授证仪式中的着装、观礼与参与礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GraduationCeremonyRules,
    name: "毕业典礼礼仪",
    desc: "毕业典礼、授证仪式中的着装、观礼与参与礼仪",
    origin: "校园",
    tags: ["社交", "礼仪", "毕业", "典礼", "校园"]
}

impl GraduationCeremonyRules {
    /// 着装观礼
    pub fn attire(&self) -> Vec<&'static str> {
        vec![
            "穿着整洁庄重的正装或礼服",
            "学位服规范佩戴",
            "观礼准时入座保持安静",
            "手机调静音",
        ]
    }

    /// 仪式参与
    pub fn ceremony(&self) -> Vec<&'static str> {
        vec![
            "听从司仪指挥起立及坐下",
            "上台或领证时有序",
            "对师长掌声致意",
            "不打断他人发言",
        ]
    }

    /// 尊重氛围
    pub fn respect(&self) -> Vec<&'static str> {
        vec![
            "尊重每位毕业生在名单内",
            "不高声喧哗打断流程",
            "亲友观礼不喧闹扰邻",
            "仪式结束遵守离场次序",
        ]
    }

    /// 结束互动
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "与师长友好道别致谢",
            "合影秩序当礼让",
            "收拾好现场垃圾",
            "与亲友简短合影不占时长",
        ]
    }
}

impl Rule for GraduationCeremonyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("graduation")
    }

    fn explain(&self) -> String {
        format!(
            "【毕业典礼礼仪】\n{}",
            [
                format!(
                    "着装观礼：\\n{}",
                    self.attire()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "仪式参与：\\n{}",
                    self.ceremony()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "尊重氛围：\\n{}",
                    self.respect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结束互动：\\n{}",
                    self.after()
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
    fn test_graduationceremonyrules_basic() {
        let rules = GraduationCeremonyRules::new();
        assert_eq!(rules.metadata().name, "毕业典礼礼仪");
        assert!(!rules.attire().is_empty());
        assert!(!rules.ceremony().is_empty());
        assert!(!rules.respect().is_empty());
        assert!(!rules.after().is_empty());
    }

    #[test]
    fn test_graduationceremonyrules_validation() {
        let rules = GraduationCeremonyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("graduation"));
    }

    #[test]
    fn test_graduationceremonyrules_explain() {
        let rules = GraduationCeremonyRules::new();
        let e = rules.explain();
        assert!(e.contains("着装观礼"));
        assert!(e.contains("仪式参与"));
        assert!(e.contains("尊重氛围"));
    }
}
