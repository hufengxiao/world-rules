//! 海市蜃楼与折射
//!
//! 海市蜃楼现象的成因与类型，包括空气密度梯度、光的折射与全反射、上现蜃景与下现蜃景的形成条件。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MirageRefractionRules,
    name: "海市蜃楼与折射",
    desc: "海市蜃楼现象的成因与类型，包括空气密度梯度、光的折射与全反射、上现蜃景与下现蜃景的形成条件。",
    origin: "大气光学与光的折射理论",
    tags: ["科学", "光学", "大气", "自然现象"]
}

impl MirageRefractionRules {
    /// 形成原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "空气温度不均形成密度梯度",
            "光在不同密度空气中折射",
            "折射率随高度连续变化",
            "光线弯曲形成虚像",
        ]
    }

    /// 蜃景类型
    pub fn types(&self) -> Vec<&'static str> {
        vec![
            "下现蜃景见于灼热路面",
            "上现蜃景见于冷海面上空",
            "侧现蜃景见于垂直温差",
            "沙漠绿洲幻象多为下现蜃景",
        ]
    }

    /// 观测条件
    pub fn observation(&self) -> Vec<&'static str> {
        vec![
            "需明显温度梯度",
            "大气层结稳定少扰动",
            "观测角度与距离适宜",
            "清晨或特定季节更易见",
        ]
    }

    /// 相关现象
    pub fn related(&self) -> Vec<&'static str> {
        vec![
            "公路远处似有水洼",
            "海面浮岛悬浮空中",
            "与彩虹成因不同但同为光学",
            "全反射是极端折射情形",
        ]
    }
}

impl Rule for MirageRefractionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("mirage")
    }

    fn explain(&self) -> String {
        format!(
            "【海市蜃楼与折射】\n{}",
            [
                format!(
                    "形成原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "蜃景类型：\\n{}",
                    self.types()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观测条件：\\n{}",
                    self.observation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "相关现象：\\n{}",
                    self.related()
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
    fn test_miragerefractionrules_basic() {
        let rules = MirageRefractionRules::new();
        assert_eq!(rules.metadata().name, "海市蜃楼与折射");
        assert!(!rules.principle().is_empty());
        assert!(!rules.types().is_empty());
        assert!(!rules.observation().is_empty());
        assert!(!rules.related().is_empty());
    }

    #[test]
    fn test_miragerefractionrules_validation() {
        let rules = MirageRefractionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("mirage"));
    }

    #[test]
    fn test_miragerefractionrules_explain() {
        let rules = MirageRefractionRules::new();
        let e = rules.explain();
        assert!(e.contains("形成原理"));
        assert!(e.contains("蜃景类型"));
        assert!(e.contains("观测条件"));
    }
}
