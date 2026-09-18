//! 骨质疏松安全步行
//!
//! 骨质疏松者适度运动与防骨安全

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OsteoporosisWalkingSafeRules,
    name: "骨质疏松安全步行",
    desc: "骨质疏松者适度运动与防骨安全",
    origin: "医学",
    tags: ["健康", "骨质疏松", "步行", "安全"]
}

impl OsteoporosisWalkingSafeRules {
    /// 适度运动
    pub fn moderate(&self) -> Vec<&'static str> {
        vec!["宜温和步行", "适量晒太阳", "补钙食物", "不强负重"]
    }

    /// 防跌倒
    pub fn fall_prevent(&self) -> Vec<&'static str> {
        vec!["选平坦路面", "穿着防滑鞋", "慢走不强求", "扶手随时用"]
    }

    /// 姿势正确
    pub fn posture(&self) -> Vec<&'static str> {
        vec!["挺胸收腹", "避免弯腰搬重", "护腰用力", "站坐端正"]
    }

    /// 监测复查
    pub fn monitor(&self) -> Vec<&'static str> {
        vec!["定期骨密度", "遵医嘱治疗", "不强颈转动", "锻炼安全"]
    }
}

impl Rule for OsteoporosisWalkingSafeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("osteoporosis")
    }

    fn explain(&self) -> String {
        format!(
            "【骨质疏松安全步行】\n{}",
            [
                format!(
                    "适度运动：\\n{}",
                    self.moderate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "防跌倒：\\n{}",
                    self.fall_prevent()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "姿势正确：\\n{}",
                    self.posture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "监测复查：\\n{}",
                    self.monitor()
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
    fn test_osteoporosiswalkingsaferules_basic() {
        let rules = OsteoporosisWalkingSafeRules::new();
        assert_eq!(rules.metadata().name, "骨质疏松安全步行");
        assert!(!rules.moderate().is_empty());
        assert!(!rules.fall_prevent().is_empty());
        assert!(!rules.posture().is_empty());
        assert!(!rules.monitor().is_empty());
    }

    #[test]
    fn test_osteoporosiswalkingsaferules_validation() {
        let rules = OsteoporosisWalkingSafeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("osteoporosis"));
    }

    #[test]
    fn test_osteoporosiswalkingsaferules_explain() {
        let rules = OsteoporosisWalkingSafeRules::new();
        let e = rules.explain();
        assert!(e.contains("适度运动"));
        assert!(e.contains("防跌倒"));
        assert!(e.contains("姿势正确"));
    }
}
