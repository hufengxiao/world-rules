//! 睡眠呼吸暂停
//!
//! 打鼾与睡眠呼吸暂停的识别、干预与就医规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SleepApneaRules,
    name: "睡眠呼吸暂停",
    desc: "打鼾与睡眠呼吸暂停的识别、干预与就医规则",
    origin: "国际",
    tags: ["健康", "睡眠", "呼吸", "打鼾", "呼吸暂停"]
}

impl SleepApneaRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "响亮的打鼾伴间歇性憋气留意",
            "醒来口干或头痛留意",
            "白天嗜睡影响专注要评估",
            "晨起疲惫却已睡足应就诊",
        ]
    }

    /// 生活方式
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "肥胖者减重有助于改善",
            "避免仰睡采用侧卧",
            "睡前避免饮酒与安眠药",
            "规律作息保证睡眠",
        ]
    }

    /// 干预治疗
    pub fn treatment(&self) -> Vec<&'static str> {
        vec![
            "就医进行睡眠监测评估",
            "遵医嘱考虑持续气道正压治疗",
            "纠正气道相关咽倾斜问题",
            "不自行滥用促醒药物",
        ]
    }

    /// 就医提示
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "常伴高血压者同步评估",
            "晨起头痛心慌可考虑监测",
            "儿童打鼾也建议评估",
            "呼吸暂停明显伴闷醒就医",
        ]
    }
}

impl Rule for SleepApneaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sleep_apnea")
    }

    fn explain(&self) -> String {
        format!(
            "【睡眠呼吸暂停】\n{}",
            [
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
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
                    "干预治疗：\\n{}",
                    self.treatment()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seek_care()
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
    fn test_sleepapnearules_basic() {
        let rules = SleepApneaRules::new();
        assert_eq!(rules.metadata().name, "睡眠呼吸暂停");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.treatment().is_empty());
        assert!(!rules.seek_care().is_empty());
    }

    #[test]
    fn test_sleepapnearules_validation() {
        let rules = SleepApneaRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("sleep_apnea"));
    }

    #[test]
    fn test_sleepapnearules_explain() {
        let rules = SleepApneaRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("生活方式"));
        assert!(e.contains("干预治疗"));
    }
}
