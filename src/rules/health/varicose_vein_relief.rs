//! 静脉曲张护理
//!
//! 腿部静脉曲张的症状观察与日常护理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VaricoseVeinReliefRules,
    name: "静脉曲张护理",
    desc: "腿部静脉曲张的症状观察与日常护理",
    origin: "中国",
    tags: ["健康", "静脉曲张", "护理", "腿部"]
}

impl VaricoseVeinReliefRules {
    /// 认识症状
    pub fn recognize(&self) -> Vec<&'static str> {
        vec![
            "腿部青筋凸起",
            "久立后酸胀沉重",
            "傍晚脚踝水肿",
            "注意皮肤变化",
        ]
    }

    /// 日常缓解
    pub fn relieve(&self) -> Vec<&'static str> {
        vec![
            "久坐久立多走动",
            "休息抬高双腿",
            "穿医用弹力袜",
            "避免长时间压迫",
        ]
    }

    /// 生活方式
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec!["控制体重", "适当运动锻炼小腿", "避免久坐久站", "戒烟限酒"]
    }

    /// 就医指征
    pub fn seek_help(&self) -> Vec<&'static str> {
        vec!["皮肤变硬发红", "破溃渗液疼痛", "突发红肿疼痛", "及时看医生"]
    }
}

impl Rule for VaricoseVeinReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("varicose_vein")
    }

    fn explain(&self) -> String {
        format!(
            "【静脉曲张护理】\n{}",
            [
                format!(
                    "认识症状：\\n{}",
                    self.recognize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常缓解：\\n{}",
                    self.relieve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活方式：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医指征：\\n{}",
                    self.seek_help()
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
    fn test_varicoseveinreliefrules_basic() {
        let rules = VaricoseVeinReliefRules::new();
        assert_eq!(rules.metadata().name, "静脉曲张护理");
        assert!(!rules.recognize().is_empty());
        assert!(!rules.relieve().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.seek_help().is_empty());
    }

    #[test]
    fn test_varicoseveinreliefrules_validation() {
        let rules = VaricoseVeinReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("varicose_vein"));
    }

    #[test]
    fn test_varicoseveinreliefrules_explain() {
        let rules = VaricoseVeinReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("认识症状"));
        assert!(e.contains("日常缓解"));
        assert!(e.contains("生活方式"));
    }
}
