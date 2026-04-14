//! 网络安全法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 网络安全法规则
pub struct CybersecurityLawRules {
    metadata: RuleMetadata,
}

impl CybersecurityLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "网络安全法规则",
                "中国网络安全法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "网络安全".into()]),
        }
    }

    /// 网络安全原则
    pub fn cybersecurity_principles(&self) -> Vec<&'static str> {
        vec![
            "安全优先原则: 网络安全优先",
            "预防为主原则: 预防安全风险",
            "综合治理原则: 多方协同治理",
            "等级保护原则: 分等级保护",
            "权责一致原则: 权利责任对应",
            "公开透明原则: 信息公开透明",
            "技术保障原则: 技术安全保障",
            "法律规范原则: 法律规范依据",
        ]
    }

    /// 网络运营者义务
    pub fn network_operator_obligations(&self) -> Vec<&'static str> {
        vec![
            "安全保护义务: 维护网络安全",
            "等级保护义务: 实施等级保护",
            "实名登记义务: 用户实名登记",
            "信息安全义务: 信息安全保护",
            "日志留存义务: 日志保存6个月",
            "应急处置义务: 安全事件处置",
            "配合执法义务: 配合监督检查",
            "安全评估义务: 安全风险评估",
        ]
    }

    /// 网络安全等级保护
    pub fn level_protection(&self) -> Vec<&'static str> {
        vec![
            "等级划分: 五级保护等级",
            "定级备案: 系统定级备案",
            "等级测评: 等级安全测评",
            "安全建设: 安全建设整改",
            "监督检查: 等级保护检查",
            "保护要求: 各等级保护要求",
            "变更备案: 系统变更备案",
            "测评周期: 定期测评要求",
        ]
    }

    /// 个人信息保护
    pub fn personal_information_protection(&self) -> Vec<&'static str> {
        vec![
            "信息收集原则: 合法正当必要",
            "信息使用限制: 使用范围限制",
            "信息安全保护: 安全技术措施",
            "信息泄露防护: 泄露防范措施",
            "信息删除权: 信息删除请求",
            "信息更正权: 信息更正请求",
            "信息知情权: 信息处理知情",
            "信息跨境传输: 跨境传输规则",
        ]
    }

    /// 关键信息基础设施保护
    pub fn critical_infrastructure(&self) -> Vec<&'static str> {
        vec![
            "设施认定: 关键设施认定",
            "安全保护: 强化保护措施",
            "安全审查: 安全审查制度",
            "风险评估: 定期风险评估",
            "应急预案: 应急预案制定",
            "演练培训: 安全演练培训",
            "供应链安全: 供应链审查",
            "数据境内存储: 数据存储要求",
        ]
    }

    /// 网络安全事件处置
    pub fn security_incident_handling(&self) -> Vec<&'static str> {
        vec![
            "事件报告: 安全事件报告",
            "应急处置: 应急响应措施",
            "事件调查: 事件调查分析",
            "事件通报: 事件通报制度",
            "漏洞修复: 漏洞修补处理",
            "影响评估: 影响范围评估",
            "补救措施: 补救整改措施",
            "责任追究: 责任认定追究",
        ]
    }

    /// 网络安全法律责任
    pub fn cybersecurity_liability(&self) -> Vec<&'static str> {
        vec![
            "行政责任: 罚款责令整改",
            "民事责任: 损害赔偿责任",
            "刑事责任: 严重违法入刑",
            "警告处罚: 警诫性处罚",
            "罚款处罚: 经济性处罚",
            "责令整改: 整改限期要求",
            "吊销许可: 许可吊销处罚",
            "暂停业务: 业务暂停处罚",
        ]
    }

    /// 网络安全监管
    pub fn cybersecurity_regulation(&self) -> Vec<&'static str> {
        vec![
            "网信办监管",
            "公安部门监管",
            "电信主管部门监管",
            "行业主管部门监管",
            "安全检查制度",
            "安全通报制度",
            "应急处置协调",
            "国际合作机制",
        ]
    }
}

impl Default for CybersecurityLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CybersecurityLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("cybersecurity")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【网络安全法规则】\n\n安全原则:\n{}\n\n运营者义务:\n{}\n\n个人信息保护:\n{}\n",
            self.cybersecurity_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.network_operator_obligations().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.personal_information_protection().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cybersecurity_law_rules() {
        let rules = CybersecurityLawRules::new();
        assert!(!rules.cybersecurity_principles().is_empty());
        assert!(!rules.network_operator_obligations().is_empty());
    }
}