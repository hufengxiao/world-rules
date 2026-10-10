//! 鼻窦炎护理
//!
//! 鼻窦炎的日常护理与缓解规则，包括鼻腔冲洗、湿度调节、休息营养与就医指征，减轻鼻塞头痛等不适。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SinusitisCareRules,
    name: "鼻窦炎护理",
    desc: "鼻窦炎的日常护理与缓解规则，包括鼻腔冲洗、湿度调节、休息营养与就医指征，减轻鼻塞头痛等不适。",
    origin: "耳鼻喉科临床护理与家庭护理建议",
    tags: ["健康", "鼻腔", "炎症", "护理"]
}

impl SinusitisCareRules {
    /// 鼻腔护理
    pub fn nasal_care(&self) -> Vec<&'static str> {
        vec![
            "用生理盐水冲洗鼻腔",
            "保持鼻腔湿润不干燥",
            "正确擤鼻避免用力过猛",
            "避免挖鼻损伤黏膜",
        ]
    }

    /// 环境调节
    pub fn environment(&self) -> Vec<&'static str> {
        vec![
            "室内保持适当湿度",
            "避免冷空气直吹",
            "远离烟雾与粉尘刺激",
            "保持空气流通",
        ]
    }

    /// 休息营养
    pub fn rest(&self) -> Vec<&'static str> {
        vec![
            "保证充足睡眠",
            "多饮温水稀释分泌物",
            "饮食清淡少辛辣",
            "热敷面部缓解胀痛",
        ]
    }

    /// 就医指征
    pub fn medical(&self) -> Vec<&'static str> {
        vec![
            "症状超过一周未缓解须就诊",
            "高热或剧烈头痛及时就医",
            "面部肿胀视力异常速诊",
            "遵医嘱用药不滥用抗生素",
        ]
    }
}

impl Rule for SinusitisCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sinusitis")
    }

    fn explain(&self) -> String {
        format!(
            "【鼻窦炎护理】\n{}",
            [
                format!(
                    "鼻腔护理：\\n{}",
                    self.nasal_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环境调节：\\n{}",
                    self.environment()
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
                    "就医指征：\\n{}",
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
    fn test_sinusitiscarerules_basic() {
        let rules = SinusitisCareRules::new();
        assert_eq!(rules.metadata().name, "鼻窦炎护理");
        assert!(!rules.nasal_care().is_empty());
        assert!(!rules.environment().is_empty());
        assert!(!rules.rest().is_empty());
        assert!(!rules.medical().is_empty());
    }

    #[test]
    fn test_sinusitiscarerules_validation() {
        let rules = SinusitisCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("sinusitis"));
    }

    #[test]
    fn test_sinusitiscarerules_explain() {
        let rules = SinusitisCareRules::new();
        let e = rules.explain();
        assert!(e.contains("鼻腔护理"));
        assert!(e.contains("环境调节"));
        assert!(e.contains("休息营养"));
    }
}
