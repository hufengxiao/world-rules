//! 防震减灾法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 防震减灾法规则
pub struct EarthquakePreventionLawRules {
    metadata: RuleMetadata,
}

impl EarthquakePreventionLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "防震减灾法规则",
                "中国防震减灾法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "防震减灾".into()]),
        }
    }

    /// 防震减灾原则
    pub fn earthquake_prevention_principles(&self) -> Vec<&'static str> {
        vec![
            "预防为主原则",
            "防御救助结合原则",
            "政府主导原则",
            "社会参与原则",
            "科技支撑原则",
            "规划先行原则",
            "分级负责原则",
            "协调联动原则",
        ]
    }

    /// 地震监测预报
    pub fn earthquake_monitoring(&self) -> Vec<&'static str> {
        vec![
            "地震监测台网建设",
            "地震监测设施保护",
            "地震监测数据管理",
            "地震预测研究",
            "地震预警系统建设",
            "地震信息发布管理",
            "地震监测技术标准",
            "地震监测监督检查",
        ]
    }

    /// 地震灾害预防
    pub fn earthquake_disaster_prevention(&self) -> Vec<&'static str> {
        vec![
            "地震灾害风险评估",
            "地震灾害预防规划",
            "建筑抗震设防要求",
            "地震安全评价制度",
            "地震灾害防御措施",
            "地震应急避难场所",
            "地震防护宣传教育",
            "地震预防监督检查",
        ]
    }

    /// 地震应急救援
    pub fn earthquake_emergency_response(&self) -> Vec<&'static str> {
        vec![
            "地震应急预案编制",
            "地震应急指挥体系",
            "地震应急救援队伍",
            "地震应急物资储备",
            "地震应急演练开展",
            "地震应急信息发布",
            "地震应急响应启动",
            "地震应急救援实施",
        ]
    }

    /// 地震灾后重建
    pub fn earthquake_recovery(&self) -> Vec<&'static str> {
        vec![
            "地震灾害损失评估",
            "灾后重建规划编制",
            "灾后重建资金筹措",
            "灾后重建项目实施",
            "灾后重建质量监督",
            "灾后重建验收管理",
            "灾后重建档案管理",
            "灾后重建监督检查",
        ]
    }

    /// 地震安全评价
    pub fn earthquake_safety_assessment(&self) -> Vec<&'static str> {
        vec![
            "地震安全评价范围",
            "地震安全评价程序",
            "地震安全评价资质",
            "地震安全评价技术",
            "地震安全评价报告",
            "地震安全评价审批",
            "地震安全评价监督",
            "地震安全评价违法",
        ]
    }

    /// 地震灾害保险
    pub fn earthquake_insurance(&self) -> Vec<&'static str> {
        vec![
            "地震灾害保险制度",
            "地震保险产品开发",
            "地震保险费率制定",
            "地震保险理赔规则",
            "地震保险补偿机制",
            "地震保险政府补贴",
            "地震保险推广机制",
            "地震保险监督管理",
        ]
    }

    /// 地震违法行为
    pub fn earthquake_violations(&self) -> Vec<&'static str> {
        vec![
            "破坏地震监测设施行为",
            "传播虚假地震信息行为",
            "违反抗震设防要求行为",
            "地震安全评价违法行为",
            "地震应急救援违法行为",
            "地震灾后重建违法行为",
            "地震灾害信息违法行为",
            "妨碍地震执法行为",
        ]
    }
}

impl Default for EarthquakePreventionLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EarthquakePreventionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("earthquake_prevention")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【防震减灾法规则】\n\n监测预报:\n{}\n\n灾害预防:\n{}\n\n应急救援:\n{}\n",
            self.earthquake_monitoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.earthquake_disaster_prevention().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.earthquake_emergency_response().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_earthquake_prevention_law_rules() {
        let rules = EarthquakePreventionLawRules::new();
        assert!(!rules.earthquake_prevention_principles().is_empty());
        assert!(!rules.earthquake_monitoring().is_empty());
    }
}
