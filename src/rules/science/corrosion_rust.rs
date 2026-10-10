//! 金属腐蚀原理
//!
//! 金属锈蚀的电化学过程与防护方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CorrosionRustRules,
    name: "金属腐蚀原理",
    desc: "金属锈蚀的电化学过程与防护方法",
    origin: "科学常识",
    tags: ["化学", "材料", "防护"]
}

impl CorrosionRustRules {
    /// 腐蚀原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "铁与水氧发生电化学腐蚀",
            "形成微电池反应",
            "铁失电子被氧化",
            "生成铁锈疏松多孔",
        ]
    }

    /// 腐蚀条件
    pub fn conditions(&self) -> Vec<&'static str> {
        vec![
            "需水与氧气共存",
            "电解质加速腐蚀",
            "酸性环境腐蚀更快",
            "盐分促进锈蚀",
        ]
    }

    /// 防护方法
    pub fn protection(&self) -> Vec<&'static str> {
        vec![
            "涂漆隔绝水氧",
            "镀锌镀铬保护层",
            "保持干燥防潮",
            "牺牲阳极保护法",
        ]
    }

    /// 生活应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "不锈钢含铬抗蚀",
            "船体用锌块保护",
            "铁器涂油防锈",
            "镀层破损须及时修补",
        ]
    }
}

impl Rule for CorrosionRustRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("corrosion_rust")
    }

    fn explain(&self) -> String {
        format!(
            "【金属腐蚀原理】\n{}",
            [
                format!(
                    "腐蚀原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "腐蚀条件：\\n{}",
                    self.conditions()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "防护方法：\\n{}",
                    self.protection()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活应用：\\n{}",
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
    fn test_corrosionrustrules_basic() {
        let rules = CorrosionRustRules::new();
        assert_eq!(rules.metadata().name, "金属腐蚀原理");
        assert!(!rules.principle().is_empty());
        assert!(!rules.conditions().is_empty());
        assert!(!rules.protection().is_empty());
        assert!(!rules.applications().is_empty());
    }

    #[test]
    fn test_corrosionrustrules_validation() {
        let rules = CorrosionRustRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("corrosion_rust"));
    }

    #[test]
    fn test_corrosionrustrules_explain() {
        let rules = CorrosionRustRules::new();
        let e = rules.explain();
        assert!(e.contains("腐蚀原理"));
        assert!(e.contains("腐蚀条件"));
        assert!(e.contains("防护方法"));
    }
}
