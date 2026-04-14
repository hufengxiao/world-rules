//! 公证法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 公证法规则
pub struct NotarizationLawRules {
    metadata: RuleMetadata,
}

impl NotarizationLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "公证法规则",
                "中国公证法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "公证".into()]),
        }
    }

    /// 公证业务范围
    pub fn notarization_scope(&self) -> Vec<&'static str> {
        vec![
            "合同公证: 合同公证事项",
            "继承公证: 遗产继承公证",
            "遗嘱公证: 遗嘱公证事项",
            "财产分割公证",
            "婚姻状况公证",
            "学历公证: 学历证明公证",
            "经历公证: 经历证明公证",
            "保全证据公证",
        ]
    }

    /// 公证机构设置
    pub fn notarization_institutions(&self) -> Vec<&'static str> {
        vec![
            "公证机构设立",
            "公证机构审批",
            "公证机构管理",
            "公证员配备",
            "公证执业区域",
            "公证分支机构",
            "公证机构责任",
            "公证机构监管",
        ]
    }

    /// 公证员规则
    pub fn notary_rules(&self) -> Vec<&'static str> {
        vec![
            "公证员资格条件",
            "公证员考核任命",
            "公证员执业证书",
            "公证员执业规范",
            "公证员权利义务",
            "公证员回避制度",
            "公证员责任追究",
            "公证员惩戒措施",
        ]
    }

    /// 公证程序
    pub fn notarization_procedure(&self) -> Vec<&'static str> {
        vec![
            "公证申请: 公证申请程序",
            "公证受理: 公证受理审查",
            "公证审查: 公证审查程序",
            "公证核实: 公证核实程序",
            "公证审批: 公证审批程序",
            "公证出证: 公证证书制作",
            "公证送达: 公证文书送达",
            "公证归档: 公证档案管理",
        ]
    }

    /// 公证效力
    pub fn notarization_effect(&self) -> Vec<&'static str> {
        vec![
            "证据效力: 公证证据效力",
            "强制执行效力",
            "法律行为成立效力",
            "公证证明效力",
            "公证域外效力",
            "公证效力争议处理",
            "公证撤销程序",
            "公证补正程序",
        ]
    }

    /// 公证收费标准
    pub fn fee_standards(&self) -> Vec<&'static str> {
        vec![
            "公证收费原则",
            "公证收费项目",
            "公证收费标准",
            "公证收费计算",
            "公证收费减免",
            "公证收费监督",
            "公证收费争议",
            "公证收费公示",
        ]
    }

    /// 公证法律责任
    pub fn legal_liability(&self) -> Vec<&'static str> {
        vec![
            "公证机构责任",
            "公证员责任",
            "公证赔偿责任",
            "公证过错认定",
            "公证诉讼责任",
            "公证行政责任",
            "公证刑事责任",
            "公证追偿权利",
        ]
    }

    /// 公证违法行为
    pub fn notarization_violations(&self) -> Vec<&'static str> {
        vec![
            "虚假公证行为",
            "违规公证行为",
            "公证失职行为",
            "公证造假行为",
            "公证收费违规",
            "公证程序违规",
            "公证管辖违规",
            "公证回避违规",
        ]
    }
}

impl Default for NotarizationLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NotarizationLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("notarization")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【公证法规则】\n\n公证范围:\n{}\n\n公证程序:\n{}\n\n公证效力:\n{}\n",
            self.notarization_scope().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.notarization_procedure().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.notarization_effect().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notarization_law_rules() {
        let rules = NotarizationLawRules::new();
        assert!(!rules.notarization_scope().is_empty());
        assert!(!rules.notarization_procedure().is_empty());
    }
}