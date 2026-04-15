//! 民用航空法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 民用航空法规则
pub struct CivilAviationLawRules {
    metadata: RuleMetadata,
}

impl CivilAviationLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "民用航空法规则",
                "中国民用航空法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "民用航空".into()]),
        }
    }

    /// 航空器管理规则
    pub fn aircraft_management(&self) -> Vec<&'static str> {
        vec![
            "航空器适航管理",
            "航空器注册登记",
            "航空器所有权转让",
            "航空器抵押租赁",
            "航空器维修管理",
            "航空器检验认证",
            "航空器档案管理",
            "航空器报废处理",
        ]
    }

    /// 航空人员管理
    pub fn aviation_personnel(&self) -> Vec<&'static str> {
        vec![
            "飞行员执照管理",
            "飞行员资质要求",
            "飞行员培训考核",
            "飞行员体检要求",
            "航空人员执照管理",
            "航空人员培训管理",
            "航空人员资格认证",
            "航空人员违规处罚",
        ]
    }

    /// 机场管理规则
    pub fn airport_management(&self) -> Vec<&'static str> {
        vec![
            "机场建设规划",
            "机场运营许可",
            "机场安全管理",
            "机场服务质量",
            "机场设施维护",
            "机场环境保护",
            "机场应急管理",
            "机场监督检查",
        ]
    }

    /// 航空运输规则
    pub fn air_transport(&self) -> Vec<&'static str> {
        vec![
            "航空运输经营许可",
            "航空运输航线管理",
            "航空运输服务质量",
            "航空运输安全管理",
            "航空运输票价管理",
            "航空运输合同规则",
            "航空运输责任保险",
            "航空运输监督检查",
        ]
    }

    /// 旅客运输规则
    pub fn passenger_transport(&self) -> Vec<&'static str> {
        vec![
            "旅客购票规则",
            "旅客乘机规则",
            "旅客行李运输",
            "旅客安全检查",
            "旅客权益保障",
            "航班延误处理",
            "航班取消处理",
            "旅客投诉处理",
        ]
    }

    /// 航空安全管理
    pub fn aviation_safety(&self) -> Vec<&'static str> {
        vec![
            "航空安全管理体系",
            "航空安全检查制度",
            "航空安全风险评估",
            "航空安全应急管理",
            "航空安全事故报告",
            "航空安全事故调查",
            "航空安全事故处理",
            "航空安全违法处罚",
        ]
    }

    /// 航空损害赔偿
    pub fn aviation_compensation(&self) -> Vec<&'static str> {
        vec![
            "旅客伤亡赔偿",
            "行李损失赔偿",
            "货物损失赔偿",
            "航空器损害赔偿",
            "地面损害赔偿",
            "赔偿限额规定",
            "赔偿程序规则",
            "赔偿争议解决",
        ]
    }

    /// 航空违法行为
    pub fn aviation_violations(&self) -> Vec<&'static str> {
        vec![
            "危及航空安全行为",
            "扰乱机场秩序行为",
            "携带危险品登机行为",
            "冒用证件登机行为",
            "破坏航空设施行为",
            "无票登机行为",
            "航空运输违法行为",
            "航空人员违法行为",
        ]
    }
}

impl Default for CivilAviationLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CivilAviationLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_aviation")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【民用航空法规则】\n\n航空器管理:\n{}\n\n航空运输:\n{}\n\n航空安全:\n{}\n",
            self.aircraft_management().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.air_transport().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.aviation_safety().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_aviation_law_rules() {
        let rules = CivilAviationLawRules::new();
        assert!(!rules.aircraft_management().is_empty());
        assert!(!rules.air_transport().is_empty());
    }
}
