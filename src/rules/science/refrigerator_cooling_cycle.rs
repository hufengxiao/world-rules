//! 冰箱制冷原理
//!
//! 压缩制冷循环通过制冷剂相变搬运热量的原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RefrigeratorCoolingCycleRules,
    name: "冰箱制冷原理",
    desc: "压缩制冷循环通过制冷剂相变搬运热量的原理",
    origin: "科学常识",
    tags: ["物理", "热学", "家电"]
}

impl RefrigeratorCoolingCycleRules {
    /// 制冷循环
    pub fn cycle(&self) -> Vec<&'static str> {
        vec![
            "压缩机压缩制冷剂升温",
            "冷凝器散热转为液态",
            "节流降压温度骤降",
            "蒸发器吸热变回气态",
        ]
    }

    /// 相变吸热
    pub fn phase_change(&self) -> Vec<&'static str> {
        vec![
            "液态蒸发吸收大量热",
            "热量由箱内搬至箱外",
            "并非产生冷而是移热",
            "冰箱背面因此发热",
        ]
    }

    /// 热量搬运
    pub fn heat_transfer(&self) -> Vec<&'static str> {
        vec![
            "热总是自发由热传冷",
            "制冷需外界做功",
            "压缩机耗电做功",
            "热量被逆势搬到外部",
        ]
    }

    /// 使用常识
    pub fn usage(&self) -> Vec<&'static str> {
        vec![
            "背面留出散热空间",
            "少频繁开关门",
            "热食放凉再入箱",
            "结霜过厚影响效率",
        ]
    }
}

impl Rule for RefrigeratorCoolingCycleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("refrigerator_cooling_cycle")
    }

    fn explain(&self) -> String {
        format!(
            "【冰箱制冷原理】\n{}",
            [
                format!(
                    "制冷循环：\\n{}",
                    self.cycle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "相变吸热：\\n{}",
                    self.phase_change()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "热量搬运：\\n{}",
                    self.heat_transfer()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "使用常识：\\n{}",
                    self.usage()
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
    fn test_refrigeratorcoolingcyclerules_basic() {
        let rules = RefrigeratorCoolingCycleRules::new();
        assert_eq!(rules.metadata().name, "冰箱制冷原理");
        assert!(!rules.cycle().is_empty());
        assert!(!rules.phase_change().is_empty());
        assert!(!rules.heat_transfer().is_empty());
        assert!(!rules.usage().is_empty());
    }

    #[test]
    fn test_refrigeratorcoolingcyclerules_validation() {
        let rules = RefrigeratorCoolingCycleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("refrigerator_cooling_cycle")
        );
    }

    #[test]
    fn test_refrigeratorcoolingcyclerules_explain() {
        let rules = RefrigeratorCoolingCycleRules::new();
        let e = rules.explain();
        assert!(e.contains("制冷循环"));
        assert!(e.contains("相变吸热"));
        assert!(e.contains("热量搬运"));
    }
}
