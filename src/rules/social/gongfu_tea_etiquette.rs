//! 功夫茶礼
//!
//! 功夫茶冲泡、奉茶与统一茶杯礼

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GongfuTeaEtiquetteRules,
    name: "功夫茶礼",
    desc: "功夫茶冲泡、奉茶与统一茶杯礼",
    origin: "中国",
    tags: ["社交", "茶", "功夫茶", "礼仪"]
}

impl GongfuTeaEtiquetteRules {
    /// 冲泡讲究
    pub fn brewing(&self) -> Vec<&'static str> {
        vec!["温水烫壶醒茶", "高冲低斟", "分秒掌握火候", "品茶最相宜"]
    }

    /// 奉茶行礼
    pub fn serve(&self) -> Vec<&'static str> {
        vec!["双手捧杯奉客", "先客后主", "斟至七分", "礼敬周全"]
    }

    /// 饮茶礼貌
    pub fn drinking(&self) -> Vec<&'static str> {
        vec!["叩指向人道谢", "细啜慢品", "不咕咚牛饮", "茶凉可请续"]
    }

    /// 交流气氛
    pub fn ambiance(&self) -> Vec<&'static str> {
        vec!["轻声交谈", "评茶留余", "谢主人款待", "宾主尽欢"]
    }
}

impl Rule for GongfuTeaEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("gongfu_tea")
    }

    fn explain(&self) -> String {
        format!(
            "【功夫茶礼】\n{}",
            [
                format!(
                    "冲泡讲究：\\n{}",
                    self.brewing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "奉茶行礼：\\n{}",
                    self.serve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮茶礼貌：\\n{}",
                    self.drinking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "交流气氛：\\n{}",
                    self.ambiance()
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
    fn test_gongfuteaetiquetterules_basic() {
        let rules = GongfuTeaEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "功夫茶礼");
        assert!(!rules.brewing().is_empty());
        assert!(!rules.serve().is_empty());
        assert!(!rules.drinking().is_empty());
        assert!(!rules.ambiance().is_empty());
    }

    #[test]
    fn test_gongfuteaetiquetterules_validation() {
        let rules = GongfuTeaEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("gongfu_tea"));
    }

    #[test]
    fn test_gongfuteaetiquetterules_explain() {
        let rules = GongfuTeaEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("冲泡讲究"));
        assert!(e.contains("奉茶行礼"));
        assert!(e.contains("饮茶礼貌"));
    }
}
