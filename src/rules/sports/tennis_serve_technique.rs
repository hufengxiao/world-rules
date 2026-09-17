//! 网球发球技术
//!
//! 网球发球握拍、抛球与动作要领

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TennisServeTechniqueRules,
    name: "网球发球技术",
    desc: "网球发球握拍、抛球与动作要领",
    origin: "国际",
    tags: ["体育", "网球", "发球", "技术"]
}

impl TennisServeTechniqueRules {
    /// 握拍姿势
    pub fn grip(&self) -> Vec<&'static str> {
        vec!["大陆式握拍", "手腕放松", "持球轻轻放", "连贯击球"]
    }

    /// 抛球要领
    pub fn toss(&self) -> Vec<&'static str> {
        vec!["稳定上抛", "落点在前上方", "不掉落不偏", "节奏一致"]
    }

    /// 发球动作
    pub fn swing(&self) -> Vec<&'static str> {
        vec!["蹬腿转身挥拍", "拍面触球中上", "随挥自然", "发向对角区"]
    }

    /// 失误处理
    pub fn fault(&self) -> Vec<&'static str> {
        vec!["一发失败二发稳", "落点调整", "脚跟勿踩线", "稳中求攻"]
    }
}

impl Rule for TennisServeTechniqueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("tennis_serve")
    }

    fn explain(&self) -> String {
        format!(
            "【网球发球技术】\n{}",
            [
                format!(
                    "握拍姿势：\\n{}",
                    self.grip()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "抛球要领：\\n{}",
                    self.toss()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发球动作：\\n{}",
                    self.swing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "失误处理：\\n{}",
                    self.fault()
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
    fn test_tennisservetechniquerules_basic() {
        let rules = TennisServeTechniqueRules::new();
        assert_eq!(rules.metadata().name, "网球发球技术");
        assert!(!rules.grip().is_empty());
        assert!(!rules.toss().is_empty());
        assert!(!rules.swing().is_empty());
        assert!(!rules.fault().is_empty());
    }

    #[test]
    fn test_tennisservetechniquerules_validation() {
        let rules = TennisServeTechniqueRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("tennis_serve"));
    }

    #[test]
    fn test_tennisservetechniquerules_explain() {
        let rules = TennisServeTechniqueRules::new();
        let e = rules.explain();
        assert!(e.contains("握拍姿势"));
        assert!(e.contains("抛球要领"));
        assert!(e.contains("发球动作"));
    }
}
