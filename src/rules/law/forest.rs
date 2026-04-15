//! 森林法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 森林法规则
pub struct ForestLawRules {
    metadata: RuleMetadata,
}

impl ForestLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "森林法规则",
                "中国森林法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "森林".into()]),
        }
    }

    /// 森林资源管理
    pub fn forest_resource_management(&self) -> Vec<&'static str> {
        vec![
            "森林资源调查统计",
            "森林资源规划管理",
            "森林资源分类经营",
            "森林资源监测评估",
            "森林资源档案管理",
            "森林资源保护措施",
            "森林资源开发利用",
            "森林资源监督检查",
        ]
    }

    /// 林权管理
    pub fn forest_ownership(&self) -> Vec<&'static str> {
        vec![
            "森林所有权归属",
            "林地使用权管理",
            "林木所有权管理",
            "林权登记发证",
            "林权流转规则",
            "林权抵押融资",
            "林权争议处理",
            "林权保护措施",
        ]
    }

    /// 森林培育规则
    pub fn forest_cultivation(&self) -> Vec<&'static str> {
        vec![
            "植树造林规划",
            "森林培育技术标准",
            "造林验收管理",
            "森林抚育管理",
            "森林更新管理",
            "林木种苗管理",
            "造林补助政策",
            "造林成果保护",
        ]
    }

    /// 森林采伐规则
    pub fn forest_harvesting(&self) -> Vec<&'static str> {
        vec![
            "森林采伐许可管理",
            "采伐限额制度",
            "采伐作业规范",
            "采伐迹地更新",
            "采伐监督检查",
            "采伐违法处罚",
            "采伐信息管理",
            "采伐档案管理",
        ]
    }

    /// 森林保护规则
    pub fn forest_protection(&self) -> Vec<&'static str> {
        vec![
            "森林防火管理",
            "森林病虫害防治",
            "森林自然保护区管理",
            "森林生态保护",
            "森林生物多样性保护",
            "森林水源涵养保护",
            "森林景观资源保护",
            "森林违法行为查处",
        ]
    }

    /// 森林经营管理
    pub fn forest_operation(&self) -> Vec<&'static str> {
        vec![
            "森林经营规划",
            "森林经营方案编制",
            "森林经营技术标准",
            "森林经营效益评估",
            "森林经营认证管理",
            "森林经营合作机制",
            "森林经营补助政策",
            "森林经营监督检查",
        ]
    }

    /// 林业产业发展
    pub fn forestry_industry(&self) -> Vec<&'static str> {
        vec![
            "木材加工产业",
            "林产化工产业",
            "林下经济产业",
            "森林旅游产业",
            "林业生物质能源",
            "林业科技创新",
            "林业产业扶持政策",
            "林业产业统计管理",
        ]
    }

    /// 森林违法行为
    pub fn forest_violations(&self) -> Vec<&'static str> {
        vec![
            "盗伐滥伐森林行为",
            "非法占用林地行为",
            "毁坏森林资源行为",
            "森林火灾违法行为",
            "森林病虫害防治违规",
            "森林采伐违法行为",
            "破坏森林保护设施行为",
            "妨碍林业执法行为",
        ]
    }
}

impl Default for ForestLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ForestLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("forest")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【森林法规则】\n\n森林资源管理:\n{}\n\n森林采伐:\n{}\n\n森林保护:\n{}\n",
            self.forest_resource_management().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.forest_harvesting().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.forest_protection().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forest_law_rules() {
        let rules = ForestLawRules::new();
        assert!(!rules.forest_resource_management().is_empty());
        assert!(!rules.forest_harvesting().is_empty());
    }
}
