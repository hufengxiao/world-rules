//! 仲裁法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 仲裁法规则
pub struct ArbitrationLawRules {
    metadata: RuleMetadata,
}

impl ArbitrationLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "仲裁法规则",
                "中国仲裁法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "仲裁".into()]),
        }
    }

    /// 仲裁适用范围
    pub fn arbitration_scope(&self) -> Vec<&'static str> {
        vec![
            "合同纠纷仲裁",
            "财产权益纠纷",
            "商事仲裁范围",
            "劳动争议仲裁",
            "农村土地承包纠纷",
            "国际商事仲裁",
            "海事仲裁",
            "不予仲裁事项",
        ]
    }

    /// 仲裁协议
    pub fn arbitration_agreement(&self) -> Vec<&'static str> {
        vec![
            "仲裁协议形式",
            "仲裁协议内容",
            "仲裁协议效力",
            "仲裁条款独立性",
            "仲裁协议无效情形",
            "仲裁协议变更",
            "仲裁协议转让",
            "仲裁协议异议",
        ]
    }

    /// 仲裁委员会
    pub fn arbitration_commission(&self) -> Vec<&'static str> {
        vec![
            "仲裁委员会设立",
            "仲裁委员会组成",
            "仲裁员资格要求",
            "仲裁员聘任程序",
            "仲裁员回避制度",
            "仲裁员责任追究",
            "仲裁委员会职责",
            "仲裁委员会管理",
        ]
    }

    /// 仲裁程序
    pub fn arbitration_procedure(&self) -> Vec<&'static str> {
        vec![
            "申请仲裁: 仲裁申请程序",
            "受理仲裁: 仲裁受理条件",
            "仲裁答辩: 答辩期限要求",
            "仲裁庭组成: 仲裁庭组建",
            "仲裁开庭: 开庭审理程序",
            "举证质证: 证据提交审核",
            "仲裁辩论: 庭审辩论程序",
            "最后陈述: 最后陈述环节",
        ]
    }

    /// 仲裁裁决
    pub fn arbitration_decision(&self) -> Vec<&'static str> {
        vec![
            "裁决作出程序",
            "裁决书内容",
            "裁决期限规定",
            "裁决效力认定",
            "裁决执行申请",
            "裁决撤销申请",
            "裁决不予执行",
            "裁决异议处理",
        ]
    }

    /// 仲裁执行
    pub fn arbitration_execution(&self) -> Vec<&'static str> {
        vec![
            "裁决申请执行",
            "执行申请期限",
            "执行管辖法院",
            "执行异议处理",
            "执行中止情形",
            "执行终结情形",
            "强制执行措施",
            "执行救济途径",
        ]
    }

    /// 仲裁特殊程序
    pub fn special_procedures(&self) -> Vec<&'static str> {
        vec![
            "简易仲裁程序",
            "紧急仲裁程序",
            "仲裁调解程序",
            "仲裁和解程序",
            "缺席仲裁程序",
            "追加当事人程序",
            "合并仲裁程序",
            "仲裁证据保全",
        ]
    }

    /// 仲裁费用
    pub fn arbitration_costs(&self) -> Vec<&'static str> {
        vec![
            "仲裁受理费用",
            "仲裁处理费用",
            "费用计算标准",
            "费用缴纳程序",
            "费用减免情形",
            "费用承担方式",
            "费用退还情形",
            "费用争议处理",
        ]
    }
}

impl Default for ArbitrationLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ArbitrationLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("arbitration")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【仲裁法规则】\n\n适用范围:\n{}\n\n仲裁协议:\n{}\n\n仲裁程序:\n{}\n",
            self.arbitration_scope().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.arbitration_agreement().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.arbitration_procedure().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arbitration_law_rules() {
        let rules = ArbitrationLawRules::new();
        assert!(!rules.arbitration_scope().is_empty());
        assert!(!rules.arbitration_procedure().is_empty());
    }
}