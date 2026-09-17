//! 赞美与褒扬礼仪
//!
//! 恰当表达赞美、接受赞美与赞美分寸的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PraiseComplimentEtiquetteRules,
    name: "赞美与褒扬礼仪",
    desc: "恰当表达赞美、接受赞美与赞美分寸的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "赞美", "夸奖", "表达"]
}

impl PraiseComplimentEtiquetteRules {
    /// 真诚赞美
    pub fn sincere(&self) -> Vec<&'static str> {
        vec![
            "由衷表达具体欣赏",
            "针对实际行为与努力",
            "措辞真诚不夸张",
            "适时恰当不过分",
        ]
    }

    /// 赞美场合
    pub fn context(&self) -> Vec<&'static str> {
        vec![
            "公开赞美注意分寸",
            "私下表达更贴心",
            "不借夸人暗贬他人",
            "符合场合与文化",
        ]
    }

    /// 接受赞美
    pub fn receive(&self) -> Vec<&'static str> {
        vec![
            "大方道谢不扭捏",
            "礼尚也能回敬",
            "不过度自谦否定",
            "诚实分享团队功劳",
        ]
    }

    /// 善意反馈
    pub fn feedback(&self) -> Vec<&'static str> {
        vec![
            "把赞美转化为鼓励",
            "对他人优点予以认可",
            "避免场面假捧",
            "真诚有助关系融洽",
        ]
    }
}

impl Rule for PraiseComplimentEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("praise")
    }

    fn explain(&self) -> String {
        format!(
            "【赞美与褒扬礼仪】\n{}",
            [
                format!(
                    "真诚赞美：\\n{}",
                    self.sincere()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "赞美场合：\\n{}",
                    self.context()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "接受赞美：\\n{}",
                    self.receive()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "善意反馈：\\n{}",
                    self.feedback()
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
    fn test_praisecomplimentetiquetterules_basic() {
        let rules = PraiseComplimentEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "赞美与褒扬礼仪");
        assert!(!rules.sincere().is_empty());
        assert!(!rules.context().is_empty());
        assert!(!rules.receive().is_empty());
        assert!(!rules.feedback().is_empty());
    }

    #[test]
    fn test_praisecomplimentetiquetterules_validation() {
        let rules = PraiseComplimentEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("praise"));
    }

    #[test]
    fn test_praisecomplimentetiquetterules_explain() {
        let rules = PraiseComplimentEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("真诚赞美"));
        assert!(e.contains("赞美场合"));
        assert!(e.contains("接受赞美"));
    }
}
