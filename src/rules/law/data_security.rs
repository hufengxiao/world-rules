//! 数据安全法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 数据安全法规则
pub struct DataSecurityLawRules {
    metadata: RuleMetadata,
}

impl DataSecurityLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "数据安全法规则",
                "中国数据安全法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "数据安全".into()]),
        }
    }

    /// 数据安全原则
    pub fn data_security_principles(&self) -> Vec<&'static str> {
        vec![
            "安全与发展并重原则",
            "分类分级保护原则",
            "风险预防原则",
            "权责明确原则",
            "全过程管理原则",
            "合法合规原则",
            "公开透明原则",
            "责任追究原则",
        ]
    }

    /// 数据分类分级
    pub fn data_classification(&self) -> Vec<&'static str> {
        vec![
            "一般数据: 普通数据保护",
            "重要数据: 重要数据强化保护",
            "核心数据: 核心数据特殊保护",
            "分类标准: 行业分类标准",
            "分级标准: 安全级别划分",
            "目录管理: 分类分级目录",
            "动态调整: 级别动态调整",
            "标识管理: 级别标识要求",
        ]
    }

    /// 数据处理规则
    pub fn data_processing(&self) -> Vec<&'static str> {
        vec![
            "数据收集: 收集范围限制",
            "数据存储: 安全存储要求",
            "数据使用: 使用范围限制",
            "数据加工: 加工处理规范",
            "数据传输: 安全传输要求",
            "数据提供: 提供安全要求",
            "数据公开: 公开审批要求",
            "数据删除: 删除处理要求",
        ]
    }

    /// 数据安全保护义务
    pub fn data_security_obligations(&self) -> Vec<&'static str> {
        vec![
            "建立安全管理制度",
            "开展安全教育培训",
            "采取安全技术措施",
            "进行风险评估",
            "制定应急预案",
            "开展安全审查",
            "配合监督检查",
            "报告安全事件",
        ]
    }

    /// 重要数据保护
    pub fn important_data_protection(&self) -> Vec<&'static str> {
        vec![
            "识别认定: 重要数据认定",
            "目录编制: 重要数据目录",
            "安全评估: 安全风险评估",
            "应急处置: 应急预案演练",
            "出境评估: 数据出境安全评估",
            "备案管理: 重要数据备案",
            "审计审查: 安全审计制度",
            "监督检查: 重点监督检查",
        ]
    }

    /// 数据出境安全
    pub fn data_export_security(&self) -> Vec<&'static str> {
        vec![
            "出境安全评估: 安全评估程序",
            "评估机构: 指定评估机构",
            "评估标准: 安全评估标准",
            "出境条件: 出境安全条件",
            "合同要求: 数据出境合同",
            "保护措施: 出境保护措施",
            "持续监控: 出境后监控",
            "风险报告: 风险报告制度",
        ]
    }

    /// 数据安全事件处置
    pub fn security_incident_handling(&self) -> Vec<&'static str> {
        vec![
            "事件报告: 安全事件报告",
            "应急处置: 应急响应措施",
            "影响评估: 影响范围评估",
            "通知用户: 用户通知义务",
            "补救措施: 补救整改措施",
            "责任追究: 责任认定追究",
            "事件记录: 事件记录保存",
            "事后总结: 总结改进措施",
        ]
    }

    /// 数据安全法律责任
    pub fn data_security_liability(&self) -> Vec<&'static str> {
        vec![
            "行政责任: 罚款责令整改",
            "民事责任: 损害赔偿责任",
            "刑事责任: 严重违法入刑",
            "警告处罚: 警诫性处罚",
            "罚款处罚: 最高1000万罚款",
            "责令改正: 整改限期要求",
            "暂停业务: 业务暂停处罚",
            "吊销许可: 许可吊销处罚",
        ]
    }
}

impl Default for DataSecurityLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DataSecurityLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("data_security")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【数据安全法规则】\n\n安全原则:\n{}\n\n分类分级:\n{}\n\n保护义务:\n{}\n",
            self.data_security_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.data_classification().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.data_security_obligations().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_security_law_rules() {
        let rules = DataSecurityLawRules::new();
        assert!(!rules.data_security_principles().is_empty());
        assert!(!rules.data_classification().is_empty());
    }
}