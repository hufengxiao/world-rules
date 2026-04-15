//! 突发事件应对法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 突发事件应对法规则
pub struct EmergencyResponseLawRules {
    metadata: RuleMetadata,
}

impl EmergencyResponseLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "突发事件应对法规则",
                "中国突发事件应对法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "突发事件应对".into()]),
        }
    }

    /// 突发事件类型
    pub fn emergency_types(&self) -> Vec<&'static str> {
        vec![
            "自然灾害突发事件",
            "事故灾难突发事件",
            "公共卫生突发事件",
            "社会安全突发事件",
            "重大突发事件",
            "较大突发事件",
            "一般突发事件",
            "特别重大突发事件",
        ]
    }

    /// 预防预警机制
    pub fn prevention_warning(&self) -> Vec<&'static str> {
        vec![
            "风险评估制度",
            "隐患排查制度",
            "监测预警系统",
            "预警信息发布",
            "预警级别划分",
            "预警响应措施",
            "预防措施落实",
            "预警解除程序",
        ]
    }

    /// 应急准备
    pub fn emergency_preparation(&self) -> Vec<&'static str> {
        vec![
            "应急预案制定",
            "应急演练开展",
            "应急物资储备",
            "应急队伍建设",
            "应急设施建设",
            "应急知识宣传",
            "应急技能培训",
            "应急资源整合",
        ]
    }

    /// 应急响应措施
    pub fn emergency_response_measures(&self) -> Vec<&'static str> {
        vec![
            "应急启动程序",
            "指挥协调机制",
            "人员疏散措施",
            "物资调配措施",
            "交通管控措施",
            "通信保障措施",
            "医疗救治措施",
            "秩序维护措施",
        ]
    }

    /// 应急救援
    pub fn emergency_rescue(&self) -> Vec<&'static str> {
        vec![
            "救援队伍调度",
            "救援物资投放",
            "人员搜救行动",
            "医疗救援行动",
            "救援协调指挥",
            "救援安全保障",
            "救援信息报告",
            "救援效果评估",
        ]
    }

    /// 灾后恢复重建
    pub fn recovery_reconstruction(&self) -> Vec<&'static str> {
        vec![
            "灾情评估统计",
            "恢复重建规划",
            "基础设施修复",
            "生产恢复支持",
            "生活安置保障",
            "心理援助服务",
            "损失补偿机制",
            "重建质量监督",
        ]
    }

    /// 社会动员
    pub fn social_mobilization(&self) -> Vec<&'static str> {
        vec![
            "志愿者动员组织",
            "社会捐赠管理",
            "志愿服务协调",
            "社会力量整合",
            "公众参与引导",
            "信息及时发布",
            "谣言防控治理",
            "社会秩序维护",
        ]
    }

    /// 违法行为
    pub fn emergency_violations(&self) -> Vec<&'static str> {
        vec![
            "隐瞒谎报突发事件",
            "迟报漏报突发事件",
            "阻碍应急救援行为",
            "散布虚假信息行为",
            "挪用应急物资行为",
            "阻碍疏散撤离行为",
            "趁机哄抬物价行为",
            "趁机实施犯罪行为",
        ]
    }
}

impl Default for EmergencyResponseLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EmergencyResponseLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("emergency_response")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【突发事件应对法规则】\n\n事件类型:\n{}\n\n预防预警:\n{}\n\n应急响应:\n{}\n",
            self.emergency_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prevention_warning().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.emergency_response_measures().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergency_response_law_rules() {
        let rules = EmergencyResponseLawRules::new();
        assert!(!rules.emergency_types().is_empty());
        assert!(!rules.prevention_warning().is_empty());
    }
}
