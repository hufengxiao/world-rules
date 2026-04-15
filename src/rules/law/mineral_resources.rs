//! 矿产资源法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 矿产资源法规则
pub struct MineralResourcesLawRules {
    metadata: RuleMetadata,
}

impl MineralResourcesLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "矿产资源法规则",
                "中国矿产资源法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "矿产资源".into()]),
        }
    }

    /// 矿产资源管理
    pub fn mineral_management(&self) -> Vec<&'static str> {
        vec![
            "矿产资源规划管理",
            "矿产资源勘查管理",
            "矿产资源开采管理",
            "矿产资源储量管理",
            "矿产资源统计管理",
            "矿产资源保护措施",
            "矿产资源综合利用",
            "矿产资源监督检查",
        ]
    }

    /// 矿业权管理
    pub fn mining_rights(&self) -> Vec<&'static str> {
        vec![
            "探矿权申请管理",
            "采矿权申请管理",
            "矿业权出让转让",
            "矿业权登记管理",
            "矿业权抵押融资",
            "矿业权争议处理",
            "矿业权保护措施",
            "矿业权监督检查",
        ]
    }

    /// 矿产勘查规则
    pub fn mineral_exploration(&self) -> Vec<&'static str> {
        vec![
            "矿产勘查许可管理",
            "勘查作业规范",
            "勘查环境保护",
            "勘查资料管理",
            "勘查成果管理",
            "勘查费用管理",
            "勘查监督检查",
            "勘查违法处罚",
        ]
    }

    /// 矿产开采规则
    pub fn mineral_extraction(&self) -> Vec<&'static str> {
        vec![
            "矿产开采许可管理",
            "开采作业规范",
            "开采环境保护",
            "开采安全管理",
            "开采资源综合利用",
            "开采费用管理",
            "开采监督检查",
            "开采违法处罚",
        ]
    }

    /// 矿山安全管理
    pub fn mining_safety(&self) -> Vec<&'static str> {
        vec![
            "矿山安全管理制度",
            "矿山安全设施配置",
            "矿山安全培训教育",
            "矿山安全应急预案",
            "矿山安全事故报告",
            "矿山安全事故调查",
            "矿山安全违法处罚",
            "矿山安全监督检查",
        ]
    }

    /// 矿山环境保护
    pub fn mining_environment(&self) -> Vec<&'static str> {
        vec![
            "矿山环境影响评估",
            "矿山环境污染防治",
            "矿山生态修复治理",
            "矿山土地复垦管理",
            "矿山环境监测管理",
            "矿山环境保护责任",
            "矿山环境违法处罚",
            "矿山环境监督检查",
        ]
    }

    /// 矿产资源税费
    pub fn mineral_taxes(&self) -> Vec<&'static str> {
        vec![
            "矿业权出让收益",
            "矿产资源补偿费",
            "资源税征收管理",
            "矿业税费减免政策",
            "矿业税费缴纳程序",
            "矿业税费监督检查",
            "矿业税费违法处罚",
            "矿业税费争议处理",
        ]
    }

    /// 矿产违法行为
    pub fn mineral_violations(&self) -> Vec<&'static str> {
        vec![
            "无证勘查开采行为",
            "越界勘查开采行为",
            "破坏矿产资源行为",
            "矿山安全违法行为",
            "矿山环境违法行为",
            "矿业权违法行为",
            "矿业税费违法行为",
            "妨碍矿业执法行为",
        ]
    }
}

impl Default for MineralResourcesLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MineralResourcesLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("mineral_resources")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【矿产资源法规则】\n\n矿产资源管理:\n{}\n\n矿业权管理:\n{}\n\n矿产开采:\n{}\n",
            self.mineral_management().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.mining_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.mineral_extraction().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mineral_resources_law_rules() {
        let rules = MineralResourcesLawRules::new();
        assert!(!rules.mineral_management().is_empty());
        assert!(!rules.mining_rights().is_empty());
    }
}
