//! 壁球基本规则
//!
//! 壁球打法、发球、计分与安全的基本规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SquashBasicsRules,
    name: "壁球基本规则",
    desc: "壁球打法、发球、计分与安全的基本规则",
    origin: "国际",
    tags: ["体育", "壁球", "规则", "球类"]
}

impl SquashBasicsRules {
    /// 场地与击球
    pub fn court(&self) -> Vec<&'static str> {
        vec![
            "球击前墙有效区",
            "击球须先触前墙",
            "落点须在有效界内",
            "球出界或击地墙界判失分",
        ]
    }

    /// 发球
    pub fn serve(&self) -> Vec<&'static str> {
        vec![
            "发球区任一侧开球",
            "球须达前墙上部",
            "发球须过头顶区",
            "发球失误对方得分",
        ]
    }

    /// 计分赛制
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "每球得分多点制",
            "先到11分或9分取决赛制",
            "10平领先2分",
            "换发制或不换发制",
        ]
    }

    /// 安全礼让
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "注意挥拍空间防碰撞",
            "阻挡击球判干扰",
            "眼部护具建议戴",
            "受伤立即停赛",
        ]
    }
}

impl Rule for SquashBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("squash")
    }

    fn explain(&self) -> String {
        format!(
            "【壁球基本规则】\n{}",
            [
                format!(
                    "场地与击球：\\n{}",
                    self.court()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发球：\\n{}",
                    self.serve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分赛制：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全礼让：\\n{}",
                    self.safety()
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
    fn test_squashbasicsrules_basic() {
        let rules = SquashBasicsRules::new();
        assert_eq!(rules.metadata().name, "壁球基本规则");
        assert!(!rules.court().is_empty());
        assert!(!rules.serve().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_squashbasicsrules_validation() {
        let rules = SquashBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("squash"));
    }

    #[test]
    fn test_squashbasicsrules_explain() {
        let rules = SquashBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("场地与击球"));
        assert!(e.contains("发球"));
        assert!(e.contains("计分赛制"));
    }
}
