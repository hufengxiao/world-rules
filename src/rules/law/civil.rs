//! 民法基础知识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 民法规则
pub struct CivilLawRules {
    metadata: RuleMetadata,
}

impl CivilLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "民法规则",
                "中国民法典基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "民法".into()]),
        }
    }

    /// 民法基本原则
    pub fn basic_principles(&self) -> Vec<&'static str> {
        vec![
            "平等原则: 民事主体地位平等",
            "自愿原则: 按自己意愿设立法律关系",
            "公平原则: 合理确定权利义务",
            "诚信原则: 诚实守信履行义务",
            "公序良俗原则: 遵守公共秩序和善良风俗",
            "绿色原则: 有利于节约资源保护环境",
        ]
    }

    /// 民事主体
    pub fn civil_subjects(&self) -> Vec<&'static str> {
        vec![
            "自然人: 出生即具有民事权利能力",
            "法人: 具有民事权利能力和行为能力",
            "非法人组织: 有一定民事权利能力",
            "国家在特殊情况下也可成为民事主体",
        ]
    }

    /// 自然人行为能力
    pub fn capacity_of_person(&self) -> Vec<&'static str> {
        vec![
            "完全民事行为能力: 18周岁以上",
            "视为完全民事行为能力: 16周岁以上有收入",
            "限制民事行为能力: 8-18周岁",
            "无民事行为能力: 不满8周岁",
            "精神障碍者可能为限制或无行为能力",
        ]
    }

    /// 民事权利
    pub fn civil_rights(&self) -> Vec<&'static str> {
        vec![
            "人身权: 生命权、健康权、名誉权、隐私权等",
            "财产权: 物权、债权、继承权等",
            "知识产权: 著作权、专利权、商标权等",
            "股权和其他投资性权利",
            "其他民事权利和利益",
        ]
    }

    /// 民事法律行为
    pub fn civil_acts(&self) -> Vec<&'static str> {
        vec![
            "有效要件: 行为能力、意思真实、内容合法",
            "无效情形: 无行为能力、违法、虚假意思表示",
            "可撤销情形: 胁迫、欺诈、重大误解、显失公平",
            "效力待定: 限制行为能力人超越能力行为",
            "无效和撤销行为自始无效",
        ]
    }

    /// 代理制度
    pub fn agency_system(&self) -> Vec<&'static str> {
        vec![
            "委托代理: 被代理人授权",
            "法定代理: 法律规定(如父母代理子女)",
            "指定代理: 法院或有关机关指定",
            "代理人在代理权限内行为",
            "无权代理需被代理人追认",
        ]
    }

    /// 诉讼时效
    pub fn limitation_of_action(&self) -> Vec<&'static str> {
        vec![
            "普通诉讼时效: 3年",
            "最长保护期: 20年",
            "从知道或应当知道权利受损时起算",
            "时效可中止、中断、延长",
            "时效届满义务人可拒绝履行",
        ]
    }
}

impl Default for CivilLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CivilLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【民法规则】\n\n\
            民法基本原则:\n{}\n\n\
            自然人行为能力:\n{}\n\n\
            民事权利:\n{}\n\n\
            诉讼时效:\n{}\n",
            self.basic_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.capacity_of_person().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.civil_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.limitation_of_action().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_law_rules() {
        let rules = CivilLawRules::new();
        assert!(!rules.basic_principles().is_empty());
    }
}