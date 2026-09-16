//! 防晒与紫外线防护
//!
//! 户外防晒、减少紫外线损伤的皮肤防护规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SunProtectionRules,
    name: "防晒与紫外线防护",
    desc: "户外防晒、减少紫外线损伤的皮肤防护规则",
    origin: "医学",
    tags: ["健康", "防晒", "紫外线", "皮肤", "户外"]
}

impl SunProtectionRules {
    /// 防晒措施
    pub fn measures(&self) -> Vec<&'static str> {
        vec![
            "外出涂抹足量广谱防晒霜",
            "避开日照最强的午间时段",
            "穿戴防晒衣物、帽与墨镜",
            "遮挡与防晒霜结合使用",
        ]
    }

    /// 正确用防晒
    pub fn application(&self) -> Vec<&'static str> {
        vec![
            "出门前提前涂抹均匀",
            "按需每2小时补涂",
            "出汗游泳后再次补涂",
            "依据肤质选合适防晒系数",
        ]
    }

    /// 儿童与敏感人群
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "婴幼儿避免长时间暴晒",
            "戴宽沿帽保护脸颈",
            "使用儿童适用的温和防晒",
            "留意无遮挡的眼部防护",
        ]
    }

    /// 晒后与防护
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "晒后冷敷舒缓皮肤",
            "多补水防止脱水",
            "留意异常皮肤变化",
            "高热暴晒不适及时阴凉休息",
        ]
    }
}

impl Rule for SunProtectionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sun_protection")
    }

    fn explain(&self) -> String {
        format!(
            "【防晒与紫外线防护】\n{}",
            [
                format!(
                    "防晒措施：\\n{}",
                    self.measures()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确用防晒：\\n{}",
                    self.application()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "儿童与敏感人群：\\n{}",
                    self.special()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "晒后与防护：\\n{}",
                    self.after()
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
    fn test_sunprotectionrules_basic() {
        let rules = SunProtectionRules::new();
        assert_eq!(rules.metadata().name, "防晒与紫外线防护");
        assert!(!rules.measures().is_empty());
        assert!(!rules.application().is_empty());
        assert!(!rules.special().is_empty());
        assert!(!rules.after().is_empty());
    }

    #[test]
    fn test_sunprotectionrules_validation() {
        let rules = SunProtectionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("sun_protection"));
    }

    #[test]
    fn test_sunprotectionrules_explain() {
        let rules = SunProtectionRules::new();
        let e = rules.explain();
        assert!(e.contains("防晒措施"));
        assert!(e.contains("正确用防晒"));
        assert!(e.contains("儿童与敏感人群"));
    }
}
