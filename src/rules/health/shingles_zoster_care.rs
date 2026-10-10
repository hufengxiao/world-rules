//! 带状疱疹护理
//!
//! 带状疱疹的护理与康复规则，包括皮疹护理、疼痛管理、休息营养与预防后遗神经痛，强调尽早规范治疗。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ShinglesZosterCareRules,
    name: "带状疱疹护理",
    desc: "带状疱疹的护理与康复规则，包括皮疹护理、疼痛管理、休息营养与预防后遗神经痛，强调尽早规范治疗。",
    origin: "皮肤科与神经内科临床护理建议",
    tags: ["健康", "皮肤", "疱疹", "神经痛"]
}

impl ShinglesZosterCareRules {
    /// 皮疹护理
    pub fn skin_care(&self) -> Vec<&'static str> {
        vec![
            "保持患处清洁干燥",
            "勿抓挠或挑破水疱",
            "穿宽松棉质衣物减少摩擦",
            "遵医嘱外用药物",
        ]
    }

    /// 疼痛管理
    pub fn pain(&self) -> Vec<&'static str> {
        vec![
            "按医嘱使用止痛或抗病毒药",
            "早期用药降低后遗神经痛",
            "冷敷缓解灼痛不适",
            "避免劳累诱发疼痛加重",
        ]
    }

    /// 休息营养
    pub fn rest(&self) -> Vec<&'static str> {
        vec![
            "保证充足睡眠",
            "饮食清淡易消化",
            "补充蛋白质与维生素",
            "避免辛辣刺激与饮酒",
        ]
    }

    /// 就医与预防
    pub fn medical(&self) -> Vec<&'static str> {
        vec![
            "出疹七十二小时内尽早就医",
            "眼部或耳部受累须专科处理",
            "疼痛持续超一月需复诊",
            "适龄人群可接种疫苗预防",
        ]
    }
}

impl Rule for ShinglesZosterCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("shingles")
    }

    fn explain(&self) -> String {
        format!(
            "【带状疱疹护理】\n{}",
            [
                format!(
                    "皮疹护理：\\n{}",
                    self.skin_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "疼痛管理：\\n{}",
                    self.pain()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "休息营养：\\n{}",
                    self.rest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医与预防：\\n{}",
                    self.medical()
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
    fn test_shingleszostercarerules_basic() {
        let rules = ShinglesZosterCareRules::new();
        assert_eq!(rules.metadata().name, "带状疱疹护理");
        assert!(!rules.skin_care().is_empty());
        assert!(!rules.pain().is_empty());
        assert!(!rules.rest().is_empty());
        assert!(!rules.medical().is_empty());
    }

    #[test]
    fn test_shingleszostercarerules_validation() {
        let rules = ShinglesZosterCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("shingles"));
    }

    #[test]
    fn test_shingleszostercarerules_explain() {
        let rules = ShinglesZosterCareRules::new();
        let e = rules.explain();
        assert!(e.contains("皮疹护理"));
        assert!(e.contains("疼痛管理"));
        assert!(e.contains("休息营养"));
    }
}
