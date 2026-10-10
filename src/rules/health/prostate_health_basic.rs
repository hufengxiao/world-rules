//! 前列腺保健
//!
//! 中老年男性前列腺健康的日常保健规则，涵盖饮食、运动、排尿习惯与定期筛查，帮助预防前列腺增生与相关疾病。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ProstateHealthBasicRules,
    name: "前列腺保健",
    desc: "中老年男性前列腺健康的日常保健规则，涵盖饮食、运动、排尿习惯与定期筛查，帮助预防前列腺增生与相关疾病。",
    origin: "泌尿外科与老年男性健康管理建议",
    tags: ["健康", "男性", "前列腺", "保健"]
}

impl ProstateHealthBasicRules {
    /// 饮食调理
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "多吃番茄等富含番茄红素食物",
            "适量坚果与深海鱼",
            "少辛辣与高脂饮食",
            "戒烟限酒",
        ]
    }

    /// 生活运动
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "避免久坐定时起身活动",
            "适度有氧运动改善循环",
            "注意下身保暖",
            "规律作息减少劳累",
        ]
    }

    /// 排尿习惯
    pub fn voiding(&self) -> Vec<&'static str> {
        vec![
            "不憋尿及时排空膀胱",
            "睡前适量饮水减少夜尿",
            "避免过量饮酒与浓茶",
            "留意排尿是否顺畅",
        ]
    }

    /// 筛查就医
    pub fn medical(&self) -> Vec<&'static str> {
        vec![
            "中年起定期体检筛查",
            "尿频尿急尿不尽须就诊",
            "血尿或排尿疼痛立即就医",
            "遵医嘱复查不自行停药",
        ]
    }
}

impl Rule for ProstateHealthBasicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("prostate")
    }

    fn explain(&self) -> String {
        format!(
            "【前列腺保健】\n{}",
            [
                format!(
                    "饮食调理：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活运动：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "排尿习惯：\\n{}",
                    self.voiding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "筛查就医：\\n{}",
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
    fn test_prostatehealthbasicrules_basic() {
        let rules = ProstateHealthBasicRules::new();
        assert_eq!(rules.metadata().name, "前列腺保健");
        assert!(!rules.diet().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.voiding().is_empty());
        assert!(!rules.medical().is_empty());
    }

    #[test]
    fn test_prostatehealthbasicrules_validation() {
        let rules = ProstateHealthBasicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("prostate"));
    }

    #[test]
    fn test_prostatehealthbasicrules_explain() {
        let rules = ProstateHealthBasicRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食调理"));
        assert!(e.contains("生活运动"));
        assert!(e.contains("排尿习惯"));
    }
}
