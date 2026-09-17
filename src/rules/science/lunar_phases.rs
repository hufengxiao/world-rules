//! 月相变化
//!
//! 月球绕地、月相周期与潮汐的科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LunarPhasesRules,
    name: "月相变化",
    desc: "月球绕地、月相周期与潮汐的科学常识",
    origin: "国际",
    tags: ["科学", "月亮", "月相", "天文"]
}

impl LunarPhasesRules {
    /// 月相周期
    pub fn phases(&self) -> Vec<&'static str> {
        vec![
            "月亮绕地球运行",
            "新月望月交替",
            "上弦下弦可见半轮",
            "满月最亮圆面",
        ]
    }

    /// 成因
    pub fn cause(&self) -> Vec<&'static str> {
        vec![
            "月相由反射日光角度",
            "地球观察位置变化",
            "不因月自转引起",
            "相对位置决定形状",
        ]
    }

    /// 潮汐联系
    pub fn tide(&self) -> Vec<&'static str> {
        vec![
            "潮汐受月引力影响",
            "农历月初大潮",
            "满月新月潮差大",
            "沿海注意潮汐安全",
        ]
    }

    /// 观月常识
    pub fn observe(&self) -> Vec<&'static str> {
        vec![
            "用望远镜安全观测",
            "辨别月相形状",
            "记录日期与可见",
            "不迷信月相禁忌散事",
        ]
    }
}

impl Rule for LunarPhasesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("lunar")
    }

    fn explain(&self) -> String {
        format!(
            "【月相变化】\n{}",
            [
                format!(
                    "月相周期：\\n{}",
                    self.phases()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "成因：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "潮汐联系：\\n{}",
                    self.tide()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观月常识：\\n{}",
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
    fn test_lunarphasesrules_basic() {
        let rules = LunarPhasesRules::new();
        assert_eq!(rules.metadata().name, "月相变化");
        assert!(!rules.phases().is_empty());
        assert!(!rules.cause().is_empty());
        assert!(!rules.tide().is_empty());
        assert!(!rules.observe().is_empty());
    }

    #[test]
    fn test_lunarphasesrules_validation() {
        let rules = LunarPhasesRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("lunar"));
    }

    #[test]
    fn test_lunarphasesrules_explain() {
        let rules = LunarPhasesRules::new();
        let e = rules.explain();
        assert!(e.contains("月相周期"));
        assert!(e.contains("成因"));
        assert!(e.contains("潮汐联系"));
    }
}
