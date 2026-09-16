//! 洗手与手部卫生
//!
//! 正确洗手时机与方法以预防传染的基本卫生规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HandWashingHygieneRules,
    name: "洗手与手部卫生",
    desc: "正确洗手时机与方法以预防传染的基本卫生规则",
    origin: "国际",
    tags: ["健康", "洗手", "卫生", "防疫", "传染病"]
}

impl HandWashingHygieneRules {
    /// 洗手时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "饭前便后洗手",
            "外出回家先洗手",
            "接触污染物、垃圾后洗手",
            "用手触摸口鼻眼前洗手",
        ]
    }

    /// 正确方法
    pub fn method(&self) -> Vec<&'static str> {
        vec![
            "用流动水湿润双手",
            "涂抹肥皂搓揉至少20秒",
            "搓洗指缝手背拇指",
            "清洗后用纸巾或净干毛巾",
        ]
    }

    /// 外出替代
    pub fn alternative(&self) -> Vec<&'static str> {
        vec![
            "无水源用免洗洗手液",
            "以含酒精成分产品消毒",
            "补水后仍应尽快正规手洗",
            "不忽视指尖与手腕清洗",
        ]
    }

    /// 护手习惯
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "洗手后适度护手霜保湿",
            "保持指甲短净",
            "特殊岗位按规范洗手",
            "伤口处理前后严格清洁",
        ]
    }
}

impl Rule for HandWashingHygieneRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hand_hygiene")
    }

    fn explain(&self) -> String {
        format!(
            "【洗手与手部卫生】\n{}",
            [
                format!(
                    "洗手时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确方法：\\n{}",
                    self.method()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "外出替代：\\n{}",
                    self.alternative()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "护手习惯：\\n{}",
                    self.care()
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
    fn test_handwashinghygienerules_basic() {
        let rules = HandWashingHygieneRules::new();
        assert_eq!(rules.metadata().name, "洗手与手部卫生");
        assert!(!rules.timing().is_empty());
        assert!(!rules.method().is_empty());
        assert!(!rules.alternative().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_handwashinghygienerules_validation() {
        let rules = HandWashingHygieneRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("hand_hygiene"));
    }

    #[test]
    fn test_handwashinghygienerules_explain() {
        let rules = HandWashingHygieneRules::new();
        let e = rules.explain();
        assert!(e.contains("洗手时机"));
        assert!(e.contains("正确方法"));
        assert!(e.contains("外出替代"));
    }
}
