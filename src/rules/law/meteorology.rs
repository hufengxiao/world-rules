//! 气象法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 气象法规则
pub struct MeteorologyLawRules {
    metadata: RuleMetadata,
}

impl MeteorologyLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "气象法规则",
                "中国气象法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "气象".into()]),
        }
    }

    /// 气象工作原则
    pub fn meteorology_principles(&self) -> Vec<&'static str> {
        vec![
            "公益服务原则",
            "统一规划原则",
            "分级管理原则",
            "科学准确原则",
            "及时高效原则",
            "防灾减灾原则",
            "科技创新原则",
            "资源共享原则",
        ]
    }

    /// 气象探测管理
    pub fn meteorological_observation(&self) -> Vec<&'static str> {
        vec![
            "气象探测站网规划",
            "气象探测设施建设",
            "气象探测环境保护",
            "气象探测数据管理",
            "气象探测技术标准",
            "气象探测信息传输",
            "气象探测监督检查",
            "气象探测违法查处",
        ]
    }

    /// 气象预报服务
    pub fn meteorological_forecast(&self) -> Vec<&'static str> {
        vec![
            "气象预报制作发布",
            "气象预报信息传播",
            "气象预报质量评估",
            "气象预报技术研发",
            "灾害天气预报预警",
            "专业气象预报服务",
            "气象预报收费标准",
            "气象预报监督检查",
        ]
    }

    /// 气象灾害防御
    pub fn meteorological_disaster_prevention(&self) -> Vec<&'static str> {
        vec![
            "气象灾害监测预警",
            "气象灾害风险评估",
            "气象灾害防御规划",
            "气象灾害应急预案",
            "气象灾害信息发布",
            "气象灾害响应措施",
            "气象灾害损失评估",
            "气象灾害防御责任",
        ]
    }

    /// 气象设施保护
    pub fn meteorological_facility_protection(&self) -> Vec<&'static str> {
        vec![
            "气象设施保护区划定",
            "气象设施保护措施",
            "气象设施维护管理",
            "气象设施安全距离",
            "气象设施巡视检查",
            "气象设施隐患治理",
            "气象设施保护宣传",
            "气象设施违法查处",
        ]
    }

    /// 气象行业管理
    pub fn meteorology_industry(&self) -> Vec<&'static str> {
        vec![
            "气象行业规划管理",
            "气象行业技术标准",
            "气象行业资质管理",
            "气象行业信息服务",
            "气象行业监督指导",
            "气象行业协作机制",
            "气象行业科技创新",
            "气象行业国际合作",
        ]
    }

    /// 气象信息管理
    pub fn meteorological_information(&self) -> Vec<&'static str> {
        vec![
            "气象信息采集管理",
            "气象信息存储管理",
            "气象信息传输管理",
            "气象信息共享机制",
            "气象信息公开制度",
            "气象信息保密管理",
            "气象信息质量控制",
            "气象信息监督检查",
        ]
    }

    /// 气象违法行为
    pub fn meteorology_violations(&self) -> Vec<&'static str> {
        vec![
            "破坏气象设施行为",
            "危害气象探测环境行为",
            "非法发布气象信息行为",
            "传播虚假气象信息行为",
            "气象信息服务违法行为",
            "气象灾害防御违法行为",
            "气象行业管理违法行为",
            "妨碍气象执法行为",
        ]
    }
}

impl Default for MeteorologyLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MeteorologyLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("meteorology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【气象法规则】\n\n气象探测:\n{}\n\n气象预报:\n{}\n\n灾害防御:\n{}\n",
            self.meteorological_observation().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.meteorological_forecast().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.meteorological_disaster_prevention().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meteorology_law_rules() {
        let rules = MeteorologyLawRules::new();
        assert!(!rules.meteorology_principles().is_empty());
        assert!(!rules.meteorological_observation().is_empty());
    }
}
