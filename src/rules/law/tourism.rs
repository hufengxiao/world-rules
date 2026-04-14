//! 旅游法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 旅游法规则
pub struct TourismLawRules {
    metadata: RuleMetadata,
}

impl TourismLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "旅游法规则",
                "中国旅游法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "旅游".into()]),
        }
    }

    /// 旅游者权利
    pub fn tourist_rights(&self) -> Vec<&'static str> {
        vec![
            "知情权: 旅游信息知情",
            "选择权: 旅游自主选择",
            "公平交易权",
            "获得赔偿权",
            "人身财产安全权",
            "人格尊严尊重权",
            "特殊群体优惠权",
            "投诉举报权利",
        ]
    }

    /// 旅游者义务
    pub fn tourist_obligations(&self) -> Vec<&'static str> {
        vec![
            "遵守法律义务",
            "遵守社会公德义务",
            "保护环境义务",
            "爱护旅游资源义务",
            "遵守安全规定义务",
            "文明旅游义务",
            "尊重当地风俗义务",
            "配合管理义务",
        ]
    }

    /// 旅行社规则
    pub fn travel_agency_rules(&self) -> Vec<&'static str> {
        vec![
            "旅行社设立许可",
            "旅行社经营范围",
            "旅行社注册资本",
            "旅行社质量保证金",
            "旅行社分支机构",
            "旅行社责任保险",
            "旅行社业务规范",
            "旅行社信息公示",
        ]
    }

    /// 导游规则
    pub fn tour_guide_rules(&self) -> Vec<&'static str> {
        vec![
            "导游资格许可",
            "导游执业规范",
            "导游服务标准",
            "导游行为规范",
            "导游报酬权利",
            "导游执业保障",
            "导游禁止行为",
            "导游信用管理",
        ]
    }

    /// 旅游合同规则
    pub fn tourism_contract(&self) -> Vec<&'static str> {
        vec![
            "旅游合同内容",
            "旅游合同签订",
            "旅游合同变更",
            "旅游合同解除",
            "旅游合同违约",
            "旅游合同退款",
            "旅游格式合同规范",
            "旅游合同争议解决",
        ]
    }

    /// 旅游安全规则
    pub fn tourism_safety(&self) -> Vec<&'static str> {
        vec![
            "旅游安全责任",
            "旅游安全警示",
            "旅游应急预案",
            "旅游安全检查",
            "旅游突发事件处置",
            "旅游保险要求",
            "旅游救援保障",
            "旅游安全事故报告",
        ]
    }

    /// 旅游资源保护
    pub fn tourism_resource_protection(&self) -> Vec<&'static str> {
        vec![
            "旅游资源规划",
            "旅游资源开发规范",
            "旅游资源保护措施",
            "旅游环境影响评估",
            "旅游景区管理",
            "旅游生态保护",
            "旅游文化遗产保护",
            "旅游承载力管理",
        ]
    }

    /// 旅游违法行为
    pub fn tourism_violations(&self) -> Vec<&'static str> {
        vec![
            "虚假宣传违法行为",
            "强制消费违法行为",
            "不合理低价游",
            "擅自变更行程",
            "甩团甩客行为",
            "旅游欺诈行为",
            "导游违法行为",
            "旅行社违法行为",
        ]
    }
}

impl Default for TourismLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TourismLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("tourism")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【旅游法规则】\n\n旅游者权利:\n{}\n\n旅行社规则:\n{}\n\n旅游合同:\n{}\n",
            self.tourist_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.travel_agency_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.tourism_contract().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tourism_law_rules() {
        let rules = TourismLawRules::new();
        assert!(!rules.tourist_rights().is_empty());
        assert!(!rules.travel_agency_rules().is_empty());
    }
}