//! 工伤认定法律
//!
//! 工伤认定的情形、程序与待遇规定

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WorkInjuryLawLaborRules,
    name: "工伤认定法律",
    desc: "工伤认定的情形、程序与待遇规定",
    origin: "中国",
    tags: ["法律", "劳动", "工伤"]
}

impl WorkInjuryLawLaborRules {
    /// 认定情形
    pub fn scenarios(&self) -> Vec<&'static str> {
        vec![
            "工作时间场所因工受伤",
            "上下班途中交通事故",
            "患职业病",
            "抢险救灾受伤",
        ]
    }

    /// 认定程序
    pub fn process(&self) -> Vec<&'static str> {
        vec![
            "单位三十日内申请认定",
            "单位不申请职工一年内可申请",
            "提交工伤认定材料",
            "劳动部门作出认定决定",
        ]
    }

    /// 工伤待遇
    pub fn benefits(&self) -> Vec<&'static str> {
        vec![
            "治疗费用由基金支付",
            "停工留薪期原工资待遇",
            "伤残按等级享受待遇",
            "工亡有丧葬补助与抚恤",
        ]
    }

    /// 维权提示
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "及时保留证据",
            "对认定不服可复议",
            "单位未参保由单位担责",
            "可申请劳动能力鉴定",
        ]
    }
}

impl Rule for WorkInjuryLawLaborRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("work_injury_law_labor")
    }

    fn explain(&self) -> String {
        format!(
            "【工伤认定法律】\n{}",
            [
                format!(
                    "认定情形：\\n{}",
                    self.scenarios()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "认定程序：\\n{}",
                    self.process()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "工伤待遇：\\n{}",
                    self.benefits()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权提示：\\n{}",
                    self.remedy()
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
    fn test_workinjurylawlaborrules_basic() {
        let rules = WorkInjuryLawLaborRules::new();
        assert_eq!(rules.metadata().name, "工伤认定法律");
        assert!(!rules.scenarios().is_empty());
        assert!(!rules.process().is_empty());
        assert!(!rules.benefits().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_workinjurylawlaborrules_validation() {
        let rules = WorkInjuryLawLaborRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("work_injury_law_labor"));
    }

    #[test]
    fn test_workinjurylawlaborrules_explain() {
        let rules = WorkInjuryLawLaborRules::new();
        let e = rules.explain();
        assert!(e.contains("认定情形"));
        assert!(e.contains("认定程序"));
        assert!(e.contains("工伤待遇"));
    }
}
