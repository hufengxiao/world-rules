//! 电信条例基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 电信条例规则
pub struct TelecommunicationsLawRules {
    metadata: RuleMetadata,
}

impl TelecommunicationsLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "电信条例规则",
                "中国电信条例基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "电信".into()]),
        }
    }

    /// 电信业务类型
    pub fn telecom_services(&self) -> Vec<&'static str> {
        vec![
            "基础电信业务",
            "增值电信业务",
            "固定电话业务",
            "移动电话业务",
            "互联网接入业务",
            "数据通信业务",
            "卫星通信业务",
            "其他电信业务",
        ]
    }

    /// 电信市场准入
    pub fn telecom_market_access(&self) -> Vec<&'static str> {
        vec![
            "电信业务经营许可",
            "基础电信业务审批",
            "增值电信业务许可",
            "电信业务许可条件",
            "电信业务许可程序",
            "电信业务许可期限",
            "电信业务许可变更",
            "电信业务许可撤销",
        ]
    }

    /// 电信资源管理
    pub fn telecom_resources(&self) -> Vec<&'static str> {
        vec![
            "电信码号资源管理",
            "电信频率资源管理",
            "电信号码分配原则",
            "电信号码使用规范",
            "电信号码收回机制",
            "电信号码有偿使用",
            "电信号码转让规则",
            "电信号码监督检查",
        ]
    }

    /// 电信服务质量
    pub fn telecom_service_quality(&self) -> Vec<&'static str> {
        vec![
            "电信服务标准规范",
            "电信服务质量考核",
            "电信服务质量监测",
            "电信服务投诉处理",
            "电信服务信息公开",
            "电信服务合同规则",
            "电信服务资费管理",
            "电信服务监督检查",
        ]
    }

    /// 电信安全保障
    pub fn telecom_security(&self) -> Vec<&'static str> {
        vec![
            "电信网络安全管理",
            "电信信息安全保护",
            "电信应急通信保障",
            "电信灾难备份恢复",
            "电信安全监督检查",
            "电信安全责任追究",
            "电信安全违法处罚",
            "电信安全报告制度",
        ]
    }

    /// 电信互联互通
    pub fn telecom_interconnection(&self) -> Vec<&'static str> {
        vec![
            "电信网间互联互通",
            "互联互通技术标准",
            "互联互通协议签订",
            "互联互通费用结算",
            "互联互通争议处理",
            "互联互通监督检查",
            "互联互通违法处罚",
            "互联互通协调机制",
        ]
    }

    /// 电信用户权利
    pub fn telecom_user_rights(&self) -> Vec<&'static str> {
        vec![
            "电信服务选择权利",
            "电信服务知情权利",
            "电信资费查询权利",
            "电信投诉申诉权利",
            "电信信息保密权利",
            "电信号码携带权利",
            "电信服务质量监督权利",
            "电信损害赔偿权利",
        ]
    }

    /// 电信违法行为
    pub fn telecom_violations(&self) -> Vec<&'static str> {
        vec![
            "擅自经营电信业务",
            "违规使用电信资源",
            "电信服务质量违规",
            "电信信息安全违规",
            "电信互联互通违规",
            "电信资费违规行为",
            "电信设施破坏行为",
            "电信消费者权益侵害",
        ]
    }
}

impl Default for TelecommunicationsLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TelecommunicationsLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("telecommunications")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【电信条例规则】\n\n电信业务:\n{}\n\n电信资源:\n{}\n\n电信服务:\n{}\n",
            self.telecom_services().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.telecom_resources().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.telecom_service_quality().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telecommunications_law_rules() {
        let rules = TelecommunicationsLawRules::new();
        assert!(!rules.telecom_services().is_empty());
        assert!(!rules.telecom_market_access().is_empty());
    }
}
