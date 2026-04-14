//! 建设工程法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 建设工程法规则
pub struct ConstructionLawRules {
    metadata: RuleMetadata,
}

impl ConstructionLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "建设工程法规则",
                "中国建设工程法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "建设".into()]),
        }
    }

    /// 建设工程许可
    pub fn construction_permit(&self) -> Vec<&'static str> {
        vec![
            "立项审批: 项目立项批准",
            "规划许可: 建设用地规划许可",
            "施工许可: 建筑工程施工许可",
            "用地许可: 建设用地批准",
            "环评审批: 环境影响评价审批",
            "消防审批: 消防设计审核",
            "人防审批: 人民防空审批",
            "其他专项审批",
        ]
    }

    /// 建设工程招投标
    pub fn tendering_rules(&self) -> Vec<&'static str> {
        vec![
            "必须招标项目: 公共项目招标",
            "招标方式: 公开招标邀请招标",
            "投标资格: 投标人资质",
            "招标文件: 招标文件编制",
            "投标文件: 投标文件编制",
            "评标程序: 评标委员会评审",
            "中标确定: 中标结果确定",
            "招标禁止行为: 招标违规禁止",
        ]
    }

    /// 建设工程合同
    pub fn construction_contract(&self) -> Vec<&'static str> {
        vec![
            "勘察合同: 地质勘察合同",
            "设计合同: 工程设计合同",
            "施工合同: 工程施工合同",
            "监理合同: 工程监理合同",
            "合同价款: 工程价款约定",
            "合同工期: 工期约定",
            "合同质量: 质量标准约定",
            "合同变更: 合同变更处理",
        ]
    }

    /// 建设工程质量
    pub fn quality_control(&self) -> Vec<&'static str> {
        vec![
            "质量责任主体: 建设设计施工监理",
            "质量标准: 国家质量标准",
            "质量检验: 质量检测检验",
            "质量验收: 工程质量验收",
            "质量保修: 工程保修责任",
            "质量缺陷处理: 质量问题处理",
            "质量监督: 政府质量监督",
            "质量事故: 质量事故处理",
        ]
    }

    /// 建设工程安全
    pub fn safety_control(&self) -> Vec<&'static str> {
        vec![
            "安全生产责任: 安全责任主体",
            "安全生产措施: 安全技术措施",
            "安全生产投入: 安全经费保障",
            "安全教育培训: 安全培训要求",
            "安全检查: 安全监督检查",
            "安全事故处理: 事故报告处理",
            "安全生产许可证: 安全许可",
            "应急救援预案: 应急预案编制",
        ]
    }

    /// 建设工程监理
    pub fn supervision_rules(&self) -> Vec<&'static str> {
        vec![
            "监理资质: 监理企业资质",
            "监理范围: 监理业务范围",
            "监理职责: 监理人员职责",
            "监理权利: 监理权利权限",
            "监理义务: 监理义务要求",
            "监理程序: 监理工作程序",
            "监理文件: 监理文件编制",
            "监理责任: 监理法律责任",
        ]
    }

    /// 建设工程竣工验收
    pub fn completion_acceptance(&self) -> Vec<&'static str> {
        vec![
            "验收条件: 完工验收条件",
            "验收程序: 验收流程规定",
            "验收组织: 验收委员会组成",
            "验收内容: 验收检查内容",
            "验收标准: 验收合格标准",
            "验收文件: 验收资料编制",
            "验收备案: 验收备案登记",
            "验收不合格处理: 不合格整改",
        ]
    }

    /// 建设工程价款
    pub fn payment_rules(&self) -> Vec<&'static str> {
        vec![
            "预付款: 工程预付款",
            "进度款: 工程进度款支付",
            "结算款: 工程结算价款",
            "结算方式: 结算计算方法",
            "价款调整: 价格调整机制",
            "价款争议: 结算争议处理",
            "价款优先权: 工程价款优先受偿",
            "价款拖欠: 欠款处理",
        ]
    }
}

impl Default for ConstructionLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ConstructionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("construction")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【建设工程法规则】\n\n建设许可:\n{}\n\n工程质量:\n{}\n\n工程安全:\n{}\n",
            self.construction_permit().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.quality_control().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_control().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction_law_rules() {
        let rules = ConstructionLawRules::new();
        assert!(!rules.construction_permit().is_empty());
        assert!(!rules.quality_control().is_empty());
    }
}