//! 闪电测距
//!
//! 看闪电到听雷声的时间差估算雷电距离

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LightningDistanceCalcRules,
    name: "闪电测距",
    desc: "看闪电到听雷声的时间差估算雷电距离",
    origin: "中国",
    tags: ["科学", "雷电", "测距", "气象"]
}

impl LightningDistanceCalcRules {
    /// 声光速度差
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "光速极快近瞬达",
            "声速每秒约340米",
            "眼见闪电在雷声前",
            "时间差定距离",
        ]
    }

    /// 估算方法
    pub fn estimate(&self) -> Vec<&'static str> {
        vec![
            "秒数乘以百米",
            "每三秒约一公里",
            "数数每次闪光",
            "估出雷电远近",
        ]
    }

    /// 安全判断
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["雷声紧贴闪电近", "三秒内危险近", "空旷处速避", "找安全遮蔽"]
    }

    /// 避险须知
    pub fn avoid(&self) -> Vec<&'static str> {
        vec!["勿站树下高台", "勿近金属水体", "进室内避雷", "关掉电器插头"]
    }
}

impl Rule for LightningDistanceCalcRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("lightning_distance")
    }

    fn explain(&self) -> String {
        format!(
            "【闪电测距】\n{}",
            [
                format!(
                    "声光速度差：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "估算方法：\\n{}",
                    self.estimate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全判断：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "避险须知：\\n{}",
                    self.avoid()
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
    fn test_lightningdistancecalcrules_basic() {
        let rules = LightningDistanceCalcRules::new();
        assert_eq!(rules.metadata().name, "闪电测距");
        assert!(!rules.principle().is_empty());
        assert!(!rules.estimate().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.avoid().is_empty());
    }

    #[test]
    fn test_lightningdistancecalcrules_validation() {
        let rules = LightningDistanceCalcRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("lightning_distance")
        );
    }

    #[test]
    fn test_lightningdistancecalcrules_explain() {
        let rules = LightningDistanceCalcRules::new();
        let e = rules.explain();
        assert!(e.contains("声光速度差"));
        assert!(e.contains("估算方法"));
        assert!(e.contains("安全判断"));
    }
}
