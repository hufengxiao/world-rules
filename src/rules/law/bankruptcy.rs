//! 破产法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 破产法规则
pub struct BankruptcyLawRules {
    metadata: RuleMetadata,
}

impl BankruptcyLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "破产法规则",
                "中国破产法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "破产法".into()]),
        }
    }

    /// 破产原因
    pub fn bankruptcy_causes(&self) -> Vec<&'static str> {
        vec![
            "不能清偿到期债务",
            "资产不足以清偿全部债务",
            "明显缺乏清偿能力",
            "明显丧失清偿能力可能",
            "债务人申请破产",
            "债权人申请破产",
            "清算责任人申请",
            "强制执行申请",
        ]
    }

    /// 破产程序类型
    pub fn bankruptcy_procedures(&self) -> Vec<&'static str> {
        vec![
            "破产清算: 清算分配财产",
            "破产重整: 重整挽救企业",
            "破产和解: 和解协议",
            "清算程序: 强制清算",
            "重整程序: 企业挽救",
            "和解程序: 债务人和解",
            "程序转换: 期间可转换程序",
            "程序终结: 终结破产程序",
        ]
    }

    /// 破产申请程序
    pub fn bankruptcy_application(&self) -> Vec<&'static str> {
        vec![
            "申请人资格: 债务人债权人",
            "申请材料: 破产申请书证据",
            "管辖法院: 债务人住所地法院",
            "受理审查: 法院审查申请",
            "受理裁定: 裁定受理破产",
            "不予受理: 驳回申请",
            "申请撤回: 撤回申请",
            "上诉权利: 对裁定上诉",
        ]
    }

    /// 破产管理人
    pub fn bankruptcy_administrator(&self) -> Vec<&'static str> {
        vec![
            "管理人指定: 法院指定",
            "管理人资格: 律师事务所会计师等",
            "管理人职责: 接管管理财产",
            "管理人权利: 处分财产",
            "管理人义务: 勤勉尽责",
            "管理人报酬: 法院确定",
            "管理人更换: 不适任更换",
            "管理人报告: 向法院报告",
        ]
    }

    /// 破产财产
    pub fn bankruptcy_property(&self) -> Vec<&'static str> {
        vec![
            "破产财产范围: 债务人全部财产",
            "破产财产接管: 管理人接管",
            "破产财产管理: 管理人管理",
            "破产财产处分: 管理人处分",
            "别除权: 担保物权优先",
            "取回权: 取回不属于破产财产",
            "抵销权: 债权债务抵销",
            "破产撤销权: 撤销不当行为",
        ]
    }

    /// 破产债权
    pub fn bankruptcy_creditor_rights(&self) -> Vec<&'static str> {
        vec![
            "债权申报: 向管理人申报",
            "申报期限: 法院确定期限",
            "债权审查: 管理人审查",
            "债权确认: 确认债权额",
            "债权异议: 异议处理",
            "债权人会议: 会议决策",
            "债权人委员会: 监督机构",
            "债权分类: 不同类别债权",
        ]
    }

    /// 债权清偿顺序
    pub fn payment_order(&self) -> Vec<&'static str> {
        vec![
            "破产费用: 管理人报酬等",
            "共益债务: 程序中发生债务",
            "职工债权: 工资社保等",
            "税收债权: 税款债权",
            "普通债权: 一般债权",
            "劣后债权: 惩罚性债权",
            "担保债权优先清偿",
            "不足清偿按比例分配",
        ]
    }

    /// 破产重整规则
    pub fn reorganization_rules(&self) -> Vec<&'static str> {
        vec![
            "重整申请: 债务人债权人申请",
            "重整计划制定: 债务人或管理人",
            "重整计划内容: 债务调整经营方案",
            "重整计划表决: 债权人表决",
            "重整计划批准: 法院批准",
            "重整计划执行: 债务人执行",
            "重整监督: 管理人监督",
            "重整终止: 失败转入清算",
        ]
    }

    /// 破产和解规则
    pub fn settlement_rules(&self) -> Vec<&'static str> {
        vec![
            "和解申请: 债务人申请",
            "和解协议提出: 债务人提出",
            "和解协议表决: 债权人表决",
            "和解协议通过: 表决通过",
            "和解协议认可: 法院认可",
            "和解协议执行: 债务人执行",
            "和解失败: 转入清算",
            "和解终止: 违反协议终止",
        ]
    }
}

impl Default for BankruptcyLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BankruptcyLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("bankruptcy")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【破产法规则】\n\n破产程序类型:\n{}\n\n破产财产:\n{}\n\n清偿顺序:\n{}\n",
            self.bankruptcy_procedures().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.bankruptcy_property().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.payment_order().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bankruptcy_law_rules() {
        let rules = BankruptcyLawRules::new();
        assert!(!rules.bankruptcy_causes().is_empty());
        assert!(!rules.payment_order().is_empty());
    }
}