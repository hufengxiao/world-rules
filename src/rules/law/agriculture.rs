//! 农业法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 农业法规则
pub struct AgricultureLawRules {
    metadata: RuleMetadata,
}

impl AgricultureLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "农业法规则",
                "中国农业法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "农业".into()]),
        }
    }

    /// 农业发展原则
    pub fn development_principles(&self) -> Vec<&'static str> {
        vec![
            "粮食安全原则",
            "农业现代化原则",
            "可持续发展原则",
            "科技兴农原则",
            "农民权益保障原则",
            "城乡统筹原则",
            "市场导向原则",
            "绿色发展原则",
        ]
    }

    /// 农业生产规则
    pub fn production_rules(&self) -> Vec<&'static str> {
        vec![
            "粮食生产保障",
            "农业种植规划",
            "农业技术推广",
            "农业机械化",
            "农业标准化",
            "农业产业化",
            "农产品加工",
            "农业综合开发",
        ]
    }

    /// 农业资源保护
    pub fn resource_protection(&self) -> Vec<&'static str> {
        vec![
            "耕地资源保护",
            "水资源农业利用",
            "农业生态保护",
            "农业生物多样性",
            "农业环境保护",
            "农业防灾减灾",
            "农业气象服务",
            "农业资源节约",
        ]
    }

    /// 农业投入保障
    pub fn investment_support(&self) -> Vec<&'static str> {
        vec![
            "农业财政投入",
            "农业补贴政策",
            "农业信贷支持",
            "农业保险保障",
            "农业投资引导",
            "农业基础设施建设",
            "农业科技投入",
            "农业人才投入",
        ]
    }

    /// 农民权益保障
    pub fn farmer_rights(&self) -> Vec<&'static str> {
        vec![
            "土地承包经营权",
            "生产经营自主权",
            "农产品处置权",
            "收益分配权",
            "民主管理权",
            "知情参与权",
            "教育培训权",
            "社会保障权",
        ]
    }

    /// 农业经营主体
    pub fn business_entities(&self) -> Vec<&'static str> {
        vec![
            "家庭农场: 家庭经营",
            "农民专业合作社",
            "农业企业: 农业公司",
            "农业产业化龙头",
            "种养大户: 规模经营",
            "农业社会化服务",
            "农业联合组织",
            "农村集体经济组织",
        ]
    }

    /// 农产品质量安全
    pub fn quality_safety(&self) -> Vec<&'static str> {
        vec![
            "农产品质量标准",
            "农产品检验检测",
            "农产品认证制度",
            "农产品追溯体系",
            "农产品标识管理",
            "农产品质量安全责任",
            "农产品市场准入",
            "农产品安全监管",
        ]
    }

    /// 农业违法行为
    pub fn agriculture_violations(&self) -> Vec<&'static str> {
        vec![
            "破坏耕地违法行为",
            "农业环境污染行为",
            "假劣农资违法行为",
            "农产品质量违法行为",
            "侵犯农民权益行为",
            "农业资源违法行为",
            "农业补贴违规行为",
            "农业许可违法行为",
        ]
    }
}

impl Default for AgricultureLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AgricultureLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("agriculture")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【农业法规则】\n\n发展原则:\n{}\n\n生产规则:\n{}\n\n农民权益:\n{}\n",
            self.development_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.production_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.farmer_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agriculture_law_rules() {
        let rules = AgricultureLawRules::new();
        assert!(!rules.development_principles().is_empty());
        assert!(!rules.production_rules().is_empty());
    }
}