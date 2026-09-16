//! 匹克球规则
//!
//! 匹克球(getle)运动的场地、发球、计分与礼仪基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PickleballRules,
    name: "匹克球规则",
    desc: "匹克球(getle)运动的场地、发球、计分与礼仪基础规则",
    origin: "国际",
    tags: ["体育", "匹克球", "球类", "规则"]
}

impl PickleballRules {
    /// 场地与设备
    pub fn court(&self) -> Vec<&'static str> {
        vec![
            "球场为短网双打及单打共用",
            "使用匹克球与专用球拍",
            "了解非截击区（厨房区）规则",
            "场地尺寸符合标准规格",
        ]
    }

    /// 发球与计分
    pub fn serve(&self) -> Vec<&'static str> {
        vec![
            "发球需在底线后进行",
            "使用下手式发球",
            "按得分规则轮换发球",
            "得分与局数按规定判定",
        ]
    }

    /// 双打配合
    pub fn doubles(&self) -> Vec<&'static str> {
        vec![
            "双打双方轮换发球",
            "接发球员站在对区",
            "遵守双方轮流击球规则",
            "不得违例跨区连续击球",
        ]
    }

    /// 运动礼仪
    pub fn etiquette(&self) -> Vec<&'static str> {
        vec![
            "击球失误主动捡球",
            "换场或休息时向对手致意",
            "不指责搭档或对手",
            "尊重计分与判罚",
        ]
    }
}

impl Rule for PickleballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pickleball")
    }

    fn explain(&self) -> String {
        format!(
            "【匹克球规则】\n{}",
            [
                format!(
                    "场地与设备：\\n{}",
                    self.court()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发球与计分：\\n{}",
                    self.serve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "双打配合：\\n{}",
                    self.doubles()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运动礼仪：\\n{}",
                    self.etiquette()
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
    fn test_pickleballrules_basic() {
        let rules = PickleballRules::new();
        assert_eq!(rules.metadata().name, "匹克球规则");
        assert!(!rules.court().is_empty());
        assert!(!rules.serve().is_empty());
        assert!(!rules.doubles().is_empty());
        assert!(!rules.etiquette().is_empty());
    }

    #[test]
    fn test_pickleballrules_validation() {
        let rules = PickleballRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("pickleball"));
    }

    #[test]
    fn test_pickleballrules_explain() {
        let rules = PickleballRules::new();
        let e = rules.explain();
        assert!(e.contains("场地与设备"));
        assert!(e.contains("发球与计分"));
        assert!(e.contains("双打配合"));
    }
}
