//! 水的表面张力
//!
//! 水分子间引力形成表面张力及其生活现象

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SurfaceTensionWaterRules,
    name: "水的表面张力",
    desc: "水分子间引力形成表面张力及其生活现象",
    origin: "科学常识",
    tags: ["物理", "流体", "生活"]
}

impl SurfaceTensionWaterRules {
    /// 成因原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "水分子间存在氢键吸引",
            "表层分子受内部分子拉扯",
            "表面似有一层弹性膜",
            "使表面积趋于最小",
        ]
    }

    /// 生活现象
    pub fn phenomena(&self) -> Vec<&'static str> {
        vec![
            "水珠呈圆球形",
            "硬币上水可鼓起",
            "昆虫可立于水面",
            "回形针轻放可浮",
        ]
    }

    /// 影响因素
    pub fn factors(&self) -> Vec<&'static str> {
        vec![
            "温度升高张力减小",
            "洗涤剂显著降低张力",
            "溶质影响张力大小",
            "故皂液不易起水珠",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "洗涤剂帮助润湿去污",
            "水面张力影响印染",
            "毛细现象与之相关",
            "表面活性剂广泛使用",
        ]
    }
}

impl Rule for SurfaceTensionWaterRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("surface_tension_water")
    }

    fn explain(&self) -> String {
        format!(
            "【水的表面张力】\n{}",
            [
                format!(
                    "成因原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活现象：\\n{}",
                    self.phenomena()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "影响因素：\\n{}",
                    self.factors()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应用实例：\\n{}",
                    self.applications()
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
    fn test_surfacetensionwaterrules_basic() {
        let rules = SurfaceTensionWaterRules::new();
        assert_eq!(rules.metadata().name, "水的表面张力");
        assert!(!rules.principle().is_empty());
        assert!(!rules.phenomena().is_empty());
        assert!(!rules.factors().is_empty());
        assert!(!rules.applications().is_empty());
    }

    #[test]
    fn test_surfacetensionwaterrules_validation() {
        let rules = SurfaceTensionWaterRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("surface_tension_water")
        );
    }

    #[test]
    fn test_surfacetensionwaterrules_explain() {
        let rules = SurfaceTensionWaterRules::new();
        let e = rules.explain();
        assert!(e.contains("成因原理"));
        assert!(e.contains("生活现象"));
        assert!(e.contains("影响因素"));
    }
}
