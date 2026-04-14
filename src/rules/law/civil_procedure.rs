//! 民事诉讼法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 民事诉讼法规则
pub struct CivilProcedureLawRules {
    metadata: RuleMetadata,
}

impl CivilProcedureLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "民事诉讼法规则",
                "中国民事诉讼法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "民事诉讼".into()]),
        }
    }

    /// 民事诉讼基本原则
    pub fn basic_principles(&self) -> Vec<&'static str> {
        vec![
            "当事人平等原则",
            "辩论原则: 诉讼辩论",
            "处分原则: 权利处分",
            "诚实信用原则",
            "公平原则: 公正审判",
            "公开审判原则",
            "两审终审原则",
            "司法独立原则",
        ]
    }

    /// 民事诉讼管辖
    pub fn jurisdiction(&self) -> Vec<&'static str> {
        vec![
            "级别管辖: 各级法院管辖",
            "地域管辖: 地域管辖规则",
            "专属管辖: 专属管辖规定",
            "协议管辖: 约定管辖",
            "移送管辖: 案件移送",
            "指定管辖: 法院指定",
            "管辖权异议",
            "管辖争议处理",
        ]
    }

    /// 民事诉讼当事人
    pub fn parties(&self) -> Vec<&'static str> {
        vec![
            "原告: 起诉当事人",
            "被告: 应诉当事人",
            "共同诉讼人",
            "第三人: 有独立请求权第三人",
            "无独立请求权第三人",
            "诉讼代表人",
            "法定代表人",
            "代理人: 诉讼代理",
        ]
    }

    /// 民事诉讼程序
    pub fn litigation_procedure(&self) -> Vec<&'static str> {
        vec![
            "起诉条件: 起诉要件",
            "受理审查: 案件受理",
            "立案登记: 立案程序",
            "送达程序: 法律文书送达",
            "审理程序: 开庭审理",
            "举证程序: 证据提交",
            "质证程序: 证据质证",
            "判决程序: 裁判作出",
        ]
    }

    /// 证据规则
    pub fn evidence_rules(&self) -> Vec<&'static str> {
        vec![
            "证据种类: 证据类型",
            "举证责任: 举证义务",
            "举证时限: 举证期限",
            "证据保全: 证据保护",
            "质证规则: 证据质证",
            "证据认定: 证据采信",
            "证明标准: 证明程度",
            "证据效力: 证据效力认定",
        ]
    }

    /// 民事审判程序
    pub fn trial_procedure(&self) -> Vec<&'static str> {
        vec![
            "一审程序: 第一次审判",
            "简易程序: 简化审判",
            "小额诉讼程序",
            "普通程序: 标准审判",
            "二审程序: 第二次审判",
            "再审程序: 再审审判",
            "审判监督程序",
            "发回重审程序",
        ]
    }

    /// 民事执行程序
    pub fn execution_procedure(&self) -> Vec<&'static str> {
        vec![
            "执行申请: 申请执行",
            "执行立案: 执行受理",
            "执行措施: 强制执行",
            "执行异议: 执行异议",
            "执行中止: 暂停执行",
            "执行终结: 结束执行",
            "执行和解: 执行和解",
            "执行回转: 执行撤销",
        ]
    }

    /// 特殊程序
    pub fn special_procedures(&self) -> Vec<&'static str> {
        vec![
            "特别程序: 特别审判",
            "督促程序: 简易催告",
            "公示催告程序",
            "破产程序: 破产审判",
            "海事诉讼程序",
            "知识产权诉讼程序",
            "涉外民事诉讼程序",
            "仲裁裁决执行程序",
        ]
    }
}

impl Default for CivilProcedureLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CivilProcedureLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_procedure")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【民事诉讼法规则】\n\n基本原则:\n{}\n\n管辖规则:\n{}\n\n诉讼程序:\n{}\n",
            self.basic_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.jurisdiction().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.litigation_procedure().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_procedure_law_rules() {
        let rules = CivilProcedureLawRules::new();
        assert!(!rules.basic_principles().is_empty());
        assert!(!rules.litigation_procedure().is_empty());
    }
}