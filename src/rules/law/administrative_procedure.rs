//! 行政诉讼法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 行政诉讼法规则
pub struct AdministrativeProcedureLawRules {
    metadata: RuleMetadata,
}

impl AdministrativeProcedureLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "行政诉讼法规则",
                "中国行政诉讼法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "行政诉讼".into()]),
        }
    }

    /// 行政诉讼原则
    pub fn basic_principles(&self) -> Vec<&'static str> {
        vec![
            "合法性审查原则",
            "合理性审查原则",
            "当事人平等原则",
            "辩论原则: 诉讼辩论",
            "处分原则: 权利处分",
            "公开审判原则",
            "司法独立原则",
            "行政监督原则",
        ]
    }

    /// 受案范围
    pub fn case_scope(&self) -> Vec<&'static str> {
        vec![
            "行政处罚案件",
            "行政许可案件",
            "行政强制案件",
            "行政征收案件",
            "行政裁决案件",
            "政府信息公开案件",
            "行政复议案件",
            "不予受理事项",
        ]
    }

    /// 管辖规则
    pub fn jurisdiction(&self) -> Vec<&'static str> {
        vec![
            "级别管辖: 法院层级管辖",
            "地域管辖: 地区管辖规则",
            "专属管辖: 专属管辖规定",
            "移送管辖: 案件移送",
            "指定管辖: 上级指定",
            "管辖权异议",
            "管辖冲突处理",
            "跨区域管辖",
        ]
    }

    /// 行政诉讼当事人
    pub fn parties(&self) -> Vec<&'static str> {
        vec![
            "原告: 行政相对人",
            "被告: 行政机关",
            "共同诉讼人",
            "第三人: 相关第三人",
            "诉讼代表人",
            "法定代表人",
            "代理人: 诉讼代理",
            "当事人权利义务",
        ]
    }

    /// 行政诉讼程序
    pub fn litigation_procedure(&self) -> Vec<&'static str> {
        vec![
            "起诉条件: 起诉要件",
            "受理审查: 案件受理",
            "立案登记: 立案程序",
            "送达程序: 文书送达",
            "审理程序: 开庭审理",
            "举证程序: 证据提交",
            "质证程序: 证据质证",
            "判决程序: 裁判作出",
        ]
    }

    /// 行政诉讼证据
    pub fn evidence_rules(&self) -> Vec<&'static str> {
        vec![
            "举证责任分配",
            "被告举证责任",
            "证据种类: 证据类型",
            "证据收集: 证据获取",
            "证据保全: 证据保护",
            "证据认定: 证据采信",
            "证明标准: 证明程度",
            "证据效力认定",
        ]
    }

    /// 行政诉讼判决类型
    pub fn judgment_types(&self) -> Vec<&'static str> {
        vec![
            "维持判决: 维持行政行为",
            "撤销判决: 撤销行政行为",
            "变更判决: 变更行政决定",
            "确认判决: 确认效力状态",
            "履行判决:责令履行职责",
            "驳回判决: 驳回诉讼请求",
            "赔偿判决: 行政赔偿",
            "重作判决: 责令重新作出",
        ]
    }

    /// 行政诉讼执行
    pub fn execution(&self) -> Vec<&'static str> {
        vec![
            "判决执行申请",
            "行政机关执行",
            "强制执行措施",
            "执行异议处理",
            "执行中止情形",
            "执行终结情形",
            "执行回转程序",
            "非诉执行程序",
        ]
    }
}

impl Default for AdministrativeProcedureLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AdministrativeProcedureLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("administrative_procedure")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【行政诉讼法规则】\n\n基本原则:\n{}\n\n受案范围:\n{}\n\n诉讼程序:\n{}\n",
            self.basic_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.case_scope().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.litigation_procedure().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_administrative_procedure_law_rules() {
        let rules = AdministrativeProcedureLawRules::new();
        assert!(!rules.basic_principles().is_empty());
        assert!(!rules.case_scope().is_empty());
    }
}