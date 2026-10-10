//! 鼻腔异物处理
//!
//! 异物进入鼻腔时的家庭处理与就医提示

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NoseForeignBodyRules,
    name: "鼻腔异物处理",
    desc: "异物进入鼻腔时的家庭处理与就医提示",
    origin: "中国",
    tags: ["急救", "五官", "儿童"]
}

impl NoseForeignBodyRules {
    /// 常见情况
    pub fn situation(&self) -> Vec<&'static str> {
        vec![
            "儿童塞入豆类珠子",
            "单侧鼻塞流臭涕",
            "鼻腔疼痛出血",
            "多发生于幼儿",
        ]
    }

    /// 家庭处理
    pub fn home_care(&self) -> Vec<&'static str> {
        vec![
            "勿用镊子强行夹取",
            "可堵住另一侧鼻孔轻擤",
            "保持儿童安静",
            "不可反复抠挖",
        ]
    }

    /// 处理禁忌
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "勿将异物推得更深",
            "光滑异物勿盲目取",
            "膨胀性异物勿用水冲",
            "勿拖延观望",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "无法取出应就医",
            "异物进入气道立即急诊",
            "呼吸困难属急症",
            "由医生专业取出",
        ]
    }
}

impl Rule for NoseForeignBodyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("nose_foreign_body")
    }

    fn explain(&self) -> String {
        format!(
            "【鼻腔异物处理】\n{}",
            [
                format!(
                    "常见情况：\\n{}",
                    self.situation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "家庭处理：\\n{}",
                    self.home_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "处理禁忌：\\n{}",
                    self.avoid()
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
    fn test_noseforeignbodyrules_basic() {
        let rules = NoseForeignBodyRules::new();
        assert_eq!(rules.metadata().name, "鼻腔异物处理");
        assert!(!rules.situation().is_empty());
        assert!(!rules.home_care().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_noseforeignbodyrules_validation() {
        let rules = NoseForeignBodyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("nose_foreign_body"));
    }

    #[test]
    fn test_noseforeignbodyrules_explain() {
        let rules = NoseForeignBodyRules::new();
        let e = rules.explain();
        assert!(e.contains("常见情况"));
        assert!(e.contains("家庭处理"));
        assert!(e.contains("处理禁忌"));
    }
}
