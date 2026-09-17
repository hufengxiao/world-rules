//! 补钙健骨
//!
//! 钙摄入、维生素D与骨骼健康

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CalciumBoneHealthRules,
    name: "补钙健骨",
    desc: "钙摄入、维生素D与骨骼健康",
    origin: "营养学",
    tags: ["健康", "钙", "骨骼", "营养"]
}

impl CalciumBoneHealthRules {
    /// 补钙来源
    pub fn source(&self) -> Vec<&'static str> {
        vec!["奶制品富钙", "豆制品豆腐", "深绿叶菜含钙", "可适量虾皮"]
    }

    /// 促进吸收
    pub fn absorb(&self) -> Vec<&'static str> {
        vec![
            "维生素D助吸收",
            "晒太阳补维D",
            "适量运动强骨",
            "避免过量咖啡因",
        ]
    }

    /// 多阶段
    pub fn stages(&self) -> Vec<&'static str> {
        vec!["儿童期打基础", "青春期关键", "老年防疏松", "各期足量"]
    }

    /// 合理补充
    pub fn supplement(&self) -> Vec<&'static str> {
        vec!["食补为主", "不足可补充剂", "遵医嘱用", "不过量钙剂"]
    }
}

impl Rule for CalciumBoneHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("calcium")
    }

    fn explain(&self) -> String {
        format!(
            "【补钙健骨】\n{}",
            [
                format!(
                    "补钙来源：\\n{}",
                    self.source()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "促进吸收：\\n{}",
                    self.absorb()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "多阶段：\\n{}",
                    self.stages()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合理补充：\\n{}",
                    self.supplement()
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
    fn test_calciumbonehealthrules_basic() {
        let rules = CalciumBoneHealthRules::new();
        assert_eq!(rules.metadata().name, "补钙健骨");
        assert!(!rules.source().is_empty());
        assert!(!rules.absorb().is_empty());
        assert!(!rules.stages().is_empty());
        assert!(!rules.supplement().is_empty());
    }

    #[test]
    fn test_calciumbonehealthrules_validation() {
        let rules = CalciumBoneHealthRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("calcium"));
    }

    #[test]
    fn test_calciumbonehealthrules_explain() {
        let rules = CalciumBoneHealthRules::new();
        let e = rules.explain();
        assert!(e.contains("补钙来源"));
        assert!(e.contains("促进吸收"));
        assert!(e.contains("多阶段"));
    }
}
