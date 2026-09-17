//! 乘机礼貌规范
//!
//! 登机就座、机舱安静与体谅邻座的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AirplaneMannerRules,
    name: "乘机礼貌规范",
    desc: "登机就座、机舱安静与体谅邻座的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "乘机", "机舱"]
}

impl AirplaneMannerRules {
    /// 登机就座
    pub fn boarding(&self) -> Vec<&'static str> {
        vec![
            "有序排队登机",
            "快速入座不堵机舱",
            "行李依规摆放",
            "礼让老人儿童",
        ]
    }

    /// 机舱安静
    pub fn quiet(&self) -> Vec<&'static str> {
        vec![
            "压低音量交谈",
            "不喧哗手机外放",
            "娱乐戴耳机",
            "照顾婴儿尽力安抚",
        ]
    }

    /// 体谅邻座
    pub fn considerate(&self) -> Vec<&'static str> {
        vec![
            "起身让邻座方便",
            "不放腿进邻座空间",
            "开合遮光板顾及他人",
            "不抢占扶手过度",
        ]
    }

    /// 整洁守规
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "废弃物放置得当",
            "尊重乘务与广播",
            "系好安全带遵规",
            "礼貌道谢告别",
        ]
    }
}

impl Rule for AirplaneMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("airplane_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【乘机礼貌规范】\n{}",
            [
                format!(
                    "登机就座：\\n{}",
                    self.boarding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "机舱安静：\\n{}",
                    self.quiet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体谅邻座：\\n{}",
                    self.considerate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "整洁守规：\\n{}",
                    self.manner()
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
    fn test_airplanemannerrules_basic() {
        let rules = AirplaneMannerRules::new();
        assert_eq!(rules.metadata().name, "乘机礼貌规范");
        assert!(!rules.boarding().is_empty());
        assert!(!rules.quiet().is_empty());
        assert!(!rules.considerate().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_airplanemannerrules_validation() {
        let rules = AirplaneMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("airplane_manner"));
    }

    #[test]
    fn test_airplanemannerrules_explain() {
        let rules = AirplaneMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("登机就座"));
        assert!(e.contains("机舱安静"));
        assert!(e.contains("体谅邻座"));
    }
}
