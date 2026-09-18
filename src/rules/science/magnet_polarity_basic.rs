//! 磁铁与南北极
//!
//! 磁铁南北极相吸相斥的磁学基本规律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MagnetPolarityBasicRules,
    name: "磁铁与南北极",
    desc: "磁铁南北极相吸相斥的磁学基本规律",
    origin: "中国",
    tags: ["科学", "磁铁", "磁性", "物理"]
}

impl MagnetPolarityBasicRules {
    /// 磁极认知
    pub fn polarity(&self) -> Vec<&'static str> {
        vec![
            "磁铁有南北两极",
            "N极S极两端",
            "断成两半仍有两极",
            "地球亦是磁体",
        ]
    }

    /// 同斥异吸
    pub fn rule(&self) -> Vec<&'static str> {
        vec![
            "同名极相互排斥",
            "异名极相互吸引",
            "磁力隔空作用",
            "力随距离渐弱",
        ]
    }

    /// 天然活用
    pub fn apply(&self) -> Vec<&'static str> {
        vec!["司南指方向", "指南针极指北", "磁铁吸铁钉", "电磁铁变磁场"]
    }

    /// 保存注意
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "勿摔高温去磁",
            "成对相吸保存",
            "远离银行卡手机",
            "强磁留意损坏",
        ]
    }
}

impl Rule for MagnetPolarityBasicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("magnet_polarity")
    }

    fn explain(&self) -> String {
        format!(
            "【磁铁与南北极】\n{}",
            [
                format!(
                    "磁极认知：\\n{}",
                    self.polarity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "同斥异吸：\\n{}",
                    self.rule()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "天然活用：\\n{}",
                    self.apply()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保存注意：\\n{}",
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
    fn test_magnetpolaritybasicrules_basic() {
        let rules = MagnetPolarityBasicRules::new();
        assert_eq!(rules.metadata().name, "磁铁与南北极");
        assert!(!rules.polarity().is_empty());
        assert!(!rules.rule().is_empty());
        assert!(!rules.apply().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_magnetpolaritybasicrules_validation() {
        let rules = MagnetPolarityBasicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("magnet_polarity"));
    }

    #[test]
    fn test_magnetpolaritybasicrules_explain() {
        let rules = MagnetPolarityBasicRules::new();
        let e = rules.explain();
        assert!(e.contains("磁极认知"));
        assert!(e.contains("同斥异吸"));
        assert!(e.contains("天然活用"));
    }
}
