//! 口腔溃疡护理
//!
//! 口腔溃疡的缓解、饮食与预防护理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OralUlcerCareRules,
    name: "口腔溃疡护理",
    desc: "口腔溃疡的缓解、饮食与预防护理规则",
    origin: "医学",
    tags: ["健康", "口腔", "溃疡", "护理"]
}

impl OralUlcerCareRules {
    /// 缓解不适
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "保持口腔清洁温和漱口",
            "用温热淡盐水漱口",
            "可遵医嘱局部用药",
            "避免舌头反复舔舐",
        ]
    }

    /// 饮食注意
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "清淡饮食减少辛辣刺激",
            "避免过烫过硬食物",
            "少吃硬物咀嚼易伤",
            "多饮水促进恢复",
        ]
    }

    /// 预防复发
    pub fn prevent(&self) -> Vec<&'static str> {
        vec![
            "保证规律作息少熬夜",
            "缓解压力保持心情",
            "补充维生素与均衡营养",
            "避免损伤口腔黏膜",
        ]
    }

    /// 就医提示
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "溃疡超过两周不愈就医",
            "反复多发性溃疡检查",
            "伴发热或明显疼痛加重就医",
            "疑有感染及时诊治",
        ]
    }
}

impl Rule for OralUlcerCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("oral_ulcer")
    }

    fn explain(&self) -> String {
        format!(
            "【口腔溃疡护理】\n{}",
            [
                format!(
                    "缓解不适：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食注意：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防复发：\\n{}",
                    self.prevent()
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
    fn test_oralulcercarerules_basic() {
        let rules = OralUlcerCareRules::new();
        assert_eq!(rules.metadata().name, "口腔溃疡护理");
        assert!(!rules.relief().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.prevent().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_oralulcercarerules_validation() {
        let rules = OralUlcerCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("oral_ulcer"));
    }

    #[test]
    fn test_oralulcercarerules_explain() {
        let rules = OralUlcerCareRules::new();
        let e = rules.explain();
        assert!(e.contains("缓解不适"));
        assert!(e.contains("饮食注意"));
        assert!(e.contains("预防复发"));
    }
}
