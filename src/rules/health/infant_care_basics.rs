//! 婴儿基础护理
//!
//! 新生儿喂养、安抚、睡眠与常见照护要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InfantCareBasicsRules,
    name: "婴儿基础护理",
    desc: "新生儿喂养、安抚、睡眠与常见照护要点",
    origin: "医学",
    tags: ["健康", "婴儿", "护理", "喂养"]
}

impl InfantCareBasicsRules {
    /// 喂养要点
    pub fn feeding(&self) -> Vec<&'static str> {
        vec![
            "按需母乳或配方喂养",
            "观察吞咽与饱足信号",
            "正确拍嗝防溢奶",
            "关注体重增长",
        ]
    }

    /// 安抚睡眠
    pub fn sleep(&self) -> Vec<&'static str> {
        vec![
            "让婴儿仰卧睡眠",
            "床上不放多余软物",
            "识别哭闹是否饥饿不适",
            "安抚时有规律有耐心",
        ]
    }

    /// 清洁护理
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec![
            "按时更换尿布",
            "正确清洁脐带臀部",
            "剪指甲防抓伤",
            "适宜温度衣着松适",
        ]
    }

    /// 安全警示
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "发热等异常就医",
            "抱婴护住头颈",
            "防摔落防误吞",
            "遵循接种与儿保",
        ]
    }
}

impl Rule for InfantCareBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("infant_care")
    }

    fn explain(&self) -> String {
        format!(
            "【婴儿基础护理】\n{}",
            [
                format!(
                    "喂养要点：\\n{}",
                    self.feeding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安抚睡眠：\\n{}",
                    self.sleep()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "清洁护理：\\n{}",
                    self.hygiene()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全警示：\\n{}",
                    self.safety()
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
    fn test_infantcarebasicsrules_basic() {
        let rules = InfantCareBasicsRules::new();
        assert_eq!(rules.metadata().name, "婴儿基础护理");
        assert!(!rules.feeding().is_empty());
        assert!(!rules.sleep().is_empty());
        assert!(!rules.hygiene().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_infantcarebasicsrules_validation() {
        let rules = InfantCareBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("infant_care"));
    }

    #[test]
    fn test_infantcarebasicsrules_explain() {
        let rules = InfantCareBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("喂养要点"));
        assert!(e.contains("安抚睡眠"));
        assert!(e.contains("清洁护理"));
    }
}
