//! 节气与昼夜现象
//!
//! 春分秋分昼夜平分、冬至夏至昼夜极值的成因

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SeasonSolsticeEquinoxRules,
    name: "节气与昼夜现象",
    desc: "春分秋分昼夜平分、冬至夏至昼夜极值的成因",
    origin: "中国",
    tags: ["科学", "节气", "天文", "季节"]
}

impl SeasonSolsticeEquinoxRules {
    /// 四季成因
    pub fn cause(&self) -> Vec<&'static str> {
        vec![
            "地轴倾斜绕日公转",
            "太阳直射点移动",
            "正午太阳高度变化",
            "昼夜长短随之变",
        ]
    }

    /// 春分秋分
    pub fn equinox(&self) -> Vec<&'static str> {
        vec![
            "昼夜平分各十二时",
            "太阳直射赤道",
            "北半球春秋分",
            "全球昼夜相当",
        ]
    }

    /// 至日昼夜
    pub fn solstice(&self) -> Vec<&'static str> {
        vec![
            "北半球冬至昼最短",
            "夏至昼最长",
            "夏至直射北回归线",
            "冬至直射南回归线",
        ]
    }

    /// 观察验证
    pub fn observe(&self) -> Vec<&'static str> {
        vec![
            "正午影长约两至",
            "日出日落方位变化",
            "节气表记录对比",
            "直观感受季节",
        ]
    }
}

impl Rule for SeasonSolsticeEquinoxRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("season_solstice_equinox")
    }

    fn explain(&self) -> String {
        format!(
            "【节气与昼夜现象】\n{}",
            [
                format!(
                    "四季成因：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "春分秋分：\\n{}",
                    self.equinox()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "至日昼夜：\\n{}",
                    self.solstice()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观察验证：\\n{}",
                    self.observe()
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
    fn test_seasonsolsticeequinoxrules_basic() {
        let rules = SeasonSolsticeEquinoxRules::new();
        assert_eq!(rules.metadata().name, "节气与昼夜现象");
        assert!(!rules.cause().is_empty());
        assert!(!rules.equinox().is_empty());
        assert!(!rules.solstice().is_empty());
        assert!(!rules.observe().is_empty());
    }

    #[test]
    fn test_seasonsolsticeequinoxrules_validation() {
        let rules = SeasonSolsticeEquinoxRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("season_solstice_equinox")
        );
    }

    #[test]
    fn test_seasonsolsticeequinoxrules_explain() {
        let rules = SeasonSolsticeEquinoxRules::new();
        let e = rules.explain();
        assert!(e.contains("四季成因"));
        assert!(e.contains("春分秋分"));
        assert!(e.contains("至日昼夜"));
    }
}
