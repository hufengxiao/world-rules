//! 遗传与DNA基础
//!
//! DNA、基因、遗传规律与遗传信息科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DnaGeneticsBasicsRules,
    name: "遗传与DNA基础",
    desc: "DNA、基因、遗传规律与遗传信息科学常识",
    origin: "国际",
    tags: ["科学", "DNA", "遗传", "基因"]
}

impl DnaGeneticsBasicsRules {
    /// DNA与基因
    pub fn structure(&self) -> Vec<&'static str> {
        vec![
            "DNA承载遗传信息",
            "基因是功能片段",
            "DNA双螺旋结构性",
            "基因决定性状趋向",
        ]
    }

    /// 遗传规律
    pub fn inheritance(&self) -> Vec<&'static str> {
        vec![
            "亲代基因传给子代",
            "显隐性状有规律",
            "遗传各占一半来源",
            "环境共同影响表现",
        ]
    }

    /// 染色体与突变
    pub fn mutation(&self) -> Vec<&'static str> {
        vec![
            "染色体携带基因",
            "突变可能改变性状",
            "多数突变中性或微小",
            "突变是演化素材",
        ]
    }

    /// 科学生活
    pub fn science(&self) -> Vec<&'static str> {
        vec![
            "基因检测结果谨慎解读",
            "勿迷信宿命论",
            "健康受多因素影响",
            "尊重基因科学研究",
        ]
    }
}

impl Rule for DnaGeneticsBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("dna_genetics")
    }

    fn explain(&self) -> String {
        format!(
            "【遗传与DNA基础】\n{}",
            [
                format!(
                    "DNA与基因：\\n{}",
                    self.structure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "遗传规律：\\n{}",
                    self.inheritance()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "染色体与突变：\\n{}",
                    self.mutation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "科学生活：\\n{}",
                    self.science()
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
    fn test_dnageneticsbasicsrules_basic() {
        let rules = DnaGeneticsBasicsRules::new();
        assert_eq!(rules.metadata().name, "遗传与DNA基础");
        assert!(!rules.structure().is_empty());
        assert!(!rules.inheritance().is_empty());
        assert!(!rules.mutation().is_empty());
        assert!(!rules.science().is_empty());
    }

    #[test]
    fn test_dnageneticsbasicsrules_validation() {
        let rules = DnaGeneticsBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("dna_genetics"));
    }

    #[test]
    fn test_dnageneticsbasicsrules_explain() {
        let rules = DnaGeneticsBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("DNA与基因"));
        assert!(e.contains("遗传规律"));
        assert!(e.contains("染色体与突变"));
    }
}
