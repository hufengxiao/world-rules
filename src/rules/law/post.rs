//! 邮政法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 邮政法规则
pub struct PostLawRules {
    metadata: RuleMetadata,
}

impl PostLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "邮政法规则",
                "中国邮政法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "邮政".into()]),
        }
    }

    /// 邮政服务规则
    pub fn postal_services(&self) -> Vec<&'static str> {
        vec![
            "邮政普遍服务",
            "邮政特殊服务",
            "邮政服务标准",
            "邮政服务资费",
            "邮政服务网络",
            "邮政服务时限",
            "邮政服务质量",
            "邮政服务监督检查",
        ]
    }

    /// 邮件寄递规则
    pub fn mail_delivery(&self) -> Vec<&'static str> {
        vec![
            "邮件收寄规则",
            "邮件分拣处理",
            "邮件运输管理",
            "邮件投递服务",
            "邮件查询服务",
            "邮件赔偿规则",
            "邮件保管期限",
            "邮件信息管理",
        ]
    }

    /// 快递服务规则
    pub fn express_services(&self) -> Vec<&'static str> {
        vec![
            "快递经营许可",
            "快递服务标准",
            "快递资费管理",
            "快递时限要求",
            "快递服务质量",
            "快递安全管理",
            "快递信息管理",
            "快递监督检查",
        ]
    }

    /// 邮政设施管理
    pub fn postal_facilities(&self) -> Vec<&'static str> {
        vec![
            "邮政营业场所设置",
            "邮政投递设施建设",
            "邮政信筒信箱设置",
            "邮政设施维护管理",
            "邮政设施保护措施",
            "邮政设施建设规划",
            "邮政设施用地保障",
            "邮政设施监督检查",
        ]
    }

    /// 邮政安全保障
    pub fn postal_security(&self) -> Vec<&'static str> {
        vec![
            "邮件安全检查",
            "邮政安全管理制度",
            "邮政安全培训教育",
            "邮政安全应急处理",
            "邮政信息安全保护",
            "邮政安全监督检查",
            "邮政安全事故报告",
            "邮政安全违法处罚",
        ]
    }

    /// 邮政用户权利
    pub fn postal_user_rights(&self) -> Vec<&'static str> {
        vec![
            "邮政通信自由权利",
            "邮政通信秘密权利",
            "邮政服务选择权利",
            "邮政资费知情权利",
            "邮政服务质量监督权利",
            "邮政投诉申诉权利",
            "邮政赔偿请求权利",
            "邮政信息查询权利",
        ]
    }

    /// 邮政监督管理
    pub fn postal_supervision(&self) -> Vec<&'static str> {
        vec![
            "邮政监督检查",
            "邮政服务质量监督",
            "邮政资费监督检查",
            "邮政安全监督检查",
            "邮政设施监督检查",
            "邮政违法处罚措施",
            "邮政信用管理制度",
            "邮政申诉处理机制",
        ]
    }

    /// 邮政违法行为
    pub fn postal_violations(&self) -> Vec<&'static str> {
        vec![
            "擅自经营邮政业务",
            "擅自经营快递业务",
            "邮件延误丢失损毁",
            "侵犯通信自由秘密",
            "冒领隐匿毁弃邮件",
            "邮政设施破坏行为",
            "邮政服务违法行为",
            "邮政信息安全违法行为",
        ]
    }
}

impl Default for PostLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PostLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("post")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【邮政法规则】\n\n邮政服务:\n{}\n\n邮件寄递:\n{}\n\n快递服务:\n{}\n",
            self.postal_services().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.mail_delivery().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.express_services().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_law_rules() {
        let rules = PostLawRules::new();
        assert!(!rules.postal_services().is_empty());
        assert!(!rules.mail_delivery().is_empty());
    }
}
