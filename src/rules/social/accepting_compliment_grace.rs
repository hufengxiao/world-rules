//! 大方接受夸奖
//!
//! 被人由衷夸奖时落落大方、不卑不亢的回应

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AcceptingComplimentGraceRules,
    name: "大方接受夸奖",
    desc: "被人由衷夸奖时落落大方、不卑不亢的回应",
    origin: "中国",
    tags: ["社交", "夸奖", "回应", "礼貌"]
}

impl AcceptingComplimentGraceRules {
    /// 坦然接受
    pub fn accept(&self) -> Vec<&'static str> {
        vec!["真诚道谢", "不否认自己成绩", "谦逊一句", "自然回应"]
    }

    /// 得体说法
    pub fn wording(&self) -> Vec<&'static str> {
        vec!["谢谢你的夸奖", "多亏大家帮忙", "我还需努力", "借机互相鼓励"]
    }

    /// 避免姿态
    pub fn avoid(&self) -> Vec<&'static str> {
        vec!["不过度自谦贬低", "不炫耀自吹", "不转移阿谀", "不尴尬推开"]
    }

    /// 看待夸奖
    pub fn attitude(&self) -> Vec<&'static str> {
        vec!["把它当认可", "虚心自励", "不骄傲自满", "保持本心前行"]
    }
}

impl Rule for AcceptingComplimentGraceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("accept_compliment")
    }

    fn explain(&self) -> String {
        format!(
            "【大方接受夸奖】\n{}",
            [
                format!(
                    "坦然接受：\\n{}",
                    self.accept()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "得体说法：\\n{}",
                    self.wording()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "避免姿态：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "看待夸奖：\\n{}",
                    self.attitude()
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
    fn test_acceptingcomplimentgracerules_basic() {
        let rules = AcceptingComplimentGraceRules::new();
        assert_eq!(rules.metadata().name, "大方接受夸奖");
        assert!(!rules.accept().is_empty());
        assert!(!rules.wording().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.attitude().is_empty());
    }

    #[test]
    fn test_acceptingcomplimentgracerules_validation() {
        let rules = AcceptingComplimentGraceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("accept_compliment"));
    }

    #[test]
    fn test_acceptingcomplimentgracerules_explain() {
        let rules = AcceptingComplimentGraceRules::new();
        let e = rules.explain();
        assert!(e.contains("坦然接受"));
        assert!(e.contains("得体说法"));
        assert!(e.contains("避免姿态"));
    }
}
