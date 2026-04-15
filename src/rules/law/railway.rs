//! 铁路法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 铁路法规则
pub struct RailwayLawRules {
    metadata: RuleMetadata,
}

impl RailwayLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "铁路法规则",
                "中国铁路法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "铁路".into()]),
        }
    }

    /// 铁路建设规则
    pub fn railway_construction(&self) -> Vec<&'static str> {
        vec![
            "铁路建设规划",
            "铁路建设项目审批",
            "铁路建设征地拆迁",
            "铁路建设技术标准",
            "铁路建设质量控制",
            "铁路建设环境保护",
            "铁路建设验收程序",
            "铁路建设安全管理",
        ]
    }

    /// 铁路运输规则
    pub fn railway_transport(&self) -> Vec<&'static str> {
        vec![
            "铁路运输组织",
            "铁路运输安全管理",
            "铁路运输服务质量",
            "铁路运输调度指挥",
            "铁路运输能力配置",
            "铁路运输应急管理",
            "铁路运输统计管理",
            "铁路运输监督检查",
        ]
    }

    /// 铁路旅客运输
    pub fn passenger_transport(&self) -> Vec<&'static str> {
        vec![
            "旅客运输服务规范",
            "旅客购票规则",
            "旅客乘车规则",
            "旅客行李运输规则",
            "旅客运输安全检查",
            "旅客运输服务质量",
            "旅客投诉处理机制",
            "旅客权益保障措施",
        ]
    }

    /// 铁路货物运输
    pub fn cargo_transport(&self) -> Vec<&'static str> {
        vec![
            "货物运输服务规范",
            "货物运输合同规则",
            "货物运输安全管理",
            "货物装卸作业规范",
            "货物运输保险规则",
            "货物损失赔偿规则",
            "货物运输时限要求",
            "货物运输监督检查",
        ]
    }

    /// 铁路安全管理
    pub fn railway_safety(&self) -> Vec<&'static str> {
        vec![
            "铁路安全管理体系",
            "铁路安全检查制度",
            "铁路安全防护措施",
            "铁路危险品运输管理",
            "铁路安全应急管理",
            "铁路安全事故报告",
            "铁路安全事故调查",
            "铁路安全事故处理",
        ]
    }

    /// 铁路设施保护
    pub fn railway_facility_protection(&self) -> Vec<&'static str> {
        vec![
            "铁路线路保护",
            "铁路设施保护区划定",
            "铁路设施安全距离",
            "铁路设施巡视检查",
            "铁路设施维护管理",
            "铁路设施隐患治理",
            "铁路设施保护宣传",
            "铁路设施违法查处",
        ]
    }

    /// 铁路价格管理
    pub fn railway_price(&self) -> Vec<&'static str> {
        vec![
            "铁路运价制定原则",
            "铁路运价分类管理",
            "铁路运价调整程序",
            "铁路运价信息公开",
            "铁路运价监督检查",
            "铁路票价优惠政策",
            "铁路运费收缴管理",
            "铁路运价违法行为查处",
        ]
    }

    /// 铁路违法行为
    pub fn railway_violations(&self) -> Vec<&'static str> {
        vec![
            "破坏铁路设施行为",
            "危害铁路安全行为",
            "扰乱铁路秩序行为",
            "携带危险品乘车行为",
            "无票乘车行为",
            "铁路乘车违法行为",
            "妨碍铁路工作行为",
            "铁路货物违法行为",
        ]
    }
}

impl Default for RailwayLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RailwayLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("railway")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【铁路法规则】\n\n铁路建设:\n{}\n\n铁路运输:\n{}\n\n铁路安全:\n{}\n",
            self.railway_construction().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.railway_transport().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.railway_safety().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_railway_law_rules() {
        let rules = RailwayLawRules::new();
        assert!(!rules.railway_construction().is_empty());
        assert!(!rules.railway_transport().is_empty());
    }
}
