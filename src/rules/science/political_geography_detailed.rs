//! 政治地理规则
//!
//! 政治地理学研究政治现象的空间分布和政治地理格局，
//! 包括国家领土、政治边界、地缘政治和政治地理格局。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 政治地理规则集合
pub struct PoliticalGeographyDetailedRules {
    metadata: RuleMetadata,
}

impl PoliticalGeographyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("政治地理规则", "政治现象空间分布和地缘政治规律")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "政治".into()]),
        }
    }

    /// 国家领土规则
    pub fn territory_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("领土范围定律", "领土范围", "国家领土范围大小"),
            ("领土形状定律", "领土形状", "领土形状类型特征"),
            ("领土位置定律", "领土位置", "领土地理位置类型"),
            ("领海定律", "海洋领土", "领海范围和权利"),
            ("领空定律", "领空范围", "领空范围和权利"),
            ("领土争端定律", "领土争议", "领土争端类型原因"),
            ("领土变迁定律", "领土变化", "领土历史变迁过程"),
            ("领土完整定律", "领土主权", "领土主权完整原则"),
        ]
    }

    /// 政治边界规则
    pub fn boundary_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("边界类型定律", "边界类型", "政治边界类型划分"),
            ("自然边界定律", "自然边界", "自然地理边界特征"),
            ("人为边界定律", "人为边界", "人为划定边界特征"),
            ("边界争端定律", "边界争议", "边界争端类型原因"),
            ("边界功能定律", "边界功能", "政治边界功能类型"),
            ("边界演变定律", "边界变化", "边界历史演变过程"),
            ("边界开放定律", "边界开放", "边界开放程度影响"),
            ("边界跨境定律", "跨境合作", "边界跨境合作机制"),
        ]
    }

    /// 地缘政治规则
    pub fn geopolitics_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地缘位置定律", "战略位置", "国家地缘战略位置"),
            ("地缘格局定律", "地缘格局", "世界地缘政治格局"),
            ("地缘战略定律", "战略选择", "国家地缘战略选择"),
            ("地缘博弈定律", "大国博弈", "大国地缘政治博弈"),
            ("地缘安全定律", "地缘安全", "地缘安全影响因素"),
            ("地缘利益定律", "利益范围", "地缘利益范围界定"),
            ("地缘同盟定律", "同盟关系", "地缘政治同盟关系"),
            ("地缘冲突定律", "冲突根源", "地缘政治冲突原因"),
        ]
    }

    /// 行政区划规则
    pub fn administrative_division_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("行政区划定律", "区划体系", "行政区划体系层次"),
            ("行政边界定律", "行政边界", "行政边界划分原则"),
            ("行政中心定律", "行政中心", "行政中心选址因素"),
            ("行政管辖定律", "管辖范围", "行政管辖范围划分"),
            ("行政等级定律", "等级体系", "行政区划等级层次"),
            ("行政调整定律", "区划调整", "行政区划调整原因"),
            ("行政效率定律", "行政效率", "行政区划效率评估"),
            ("行政自治定律", "自治区域", "行政自治区域特征"),
        ]
    }

    /// 国际组织规则
    pub fn international_organization_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("联合国定律", "国际组织", "联合国地理功能"),
            ("区域组织定律", "区域组织", "区域国际组织地理"),
            ("条约组织定律", "条约组织", "国际条约组织地理"),
            ("国际法定律", "国际边界", "国际法边界原则"),
            ("国际水域定律", "国际水域", "国际水域地理权利"),
            ("国际领空定律", "国际领空", "国际领空权利范围"),
            ("国际合作定律", "合作地理", "国际地理合作机制"),
            ("国际争端定律", "争端解决", "国际争端地理解决"),
        ]
    }

    /// 军事地理规则
    pub fn military_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("军事位置定律", "战略位置", "军事战略位置价值"),
            ("军事基地定律", "基地布局", "军事基地地理布局"),
            ("军事通道定律", "战略通道", "军事战略通道地理"),
            ("军事边界定律", "军事边界", "军事边界防御特征"),
            ("军事地形定律", "地形影响", "地形对军事的影响"),
            ("军事资源定律", "资源战略", "战略资源军事价值"),
            ("军事安全定律", "地理安全", "地理军事安全因素"),
            ("军事冲突定律", "冲突地理", "军事冲突地理因素"),
        ]
    }

    /// 选区地理规则
    pub fn electoral_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("选区划分定律", "选区划分", "选举区划分原则"),
            ("选区形状定律", "选区形状", "选区形状类型特征"),
            ("选区人口定律", "人口平衡", "选区人口平衡原则"),
            ("选区边界定律", "选区边界", "选区边界划分因素"),
            ("投票地理定律", "投票分布", "投票行为地理分布"),
            ("政党地理定律", "政党分布", "政党支持地理分布"),
            ("选举结果定律", "结果分布", "选举结果地理分布"),
            ("选区改革定律", "选区调整", "选区改革调整原因"),
        ]
    }

    /// 政治地理格局规则
    pub fn political_pattern_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("全球格局定律", "世界格局", "全球政治地理格局"),
            ("区域格局定律", "区域格局", "区域政治地理格局"),
            ("霸权格局定律", "霸权分布", "霸权政治地理格局"),
            ("多极格局定律", "多极世界", "多极政治地理格局"),
            ("冷战格局定律", "冷战地理", "冷战政治地理格局"),
            ("后冷战格局定律", "新格局", "后冷战政治地理格局"),
            ("地缘格局演变定律", "格局变化", "地缘格局演变过程"),
            ("格局重构定律", "格局重构", "政治格局重构过程"),
        ]
    }

    /// 主要政治区域类型
    pub fn major_political_regions(&self) -> Vec<&'static str> {
        vec![
            "主权国家: 完全主权的独立国家",
            "自治区域: 享有自治权的区域",
            "联邦单位: 联邦制国家成员单位",
            "特别行政区: 特殊地位行政区",
            "殖民地: 依附于他国的殖民地",
            "争议地区: 有主权争议的地区",
            "军事占领区: 军事占领控制区",
            "国际共管区: 多国共同管理区",
            "自由区: 特殊自由经济区",
            "缓冲区: 缓冲地带缓冲国",
        ]
    }

    /// 政治地理研究方法
    pub fn research_methods(&self) -> Vec<&'static str> {
        vec![
            "政治地图: 政治地理分布地图绘制",
            "边界分析: 政治边界分析方法",
            "地缘分析: 地缘政治分析方法",
            "区划分析: 行政区划分析方法",
            "冲突分析: 政治冲突地理分析",
            "选举分析: 选区地理分析方法",
            "GIS分析: GIS政治地理分析",
            "模型模拟: 政治地理模型模拟",
        ]
    }
}

impl Default for PoliticalGeographyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PoliticalGeographyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("political_geography_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【政治地理规则】\n\n\
            国家领土规则:\n{}\n\n\
            政治边界规则:\n{}\n\n\
            地缘政治规则:\n{}\n\n\
            行政区划规则:\n{}\n\n\
            国际组织规则:\n{}\n\n\
            军事地理规则:\n{}\n\n\
            选区地理规则:\n{}\n\n\
            政治地理格局规则:\n{}\n\n\
            主要政治区域类型:\n{}\n\n\
            政治地理研究方法:\n{}",
            self.territory_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.boundary_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.geopolitics_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.administrative_division_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.international_organization_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.military_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.electoral_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.political_pattern_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.major_political_regions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.research_methods()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_political_geography_detailed_rules() {
        let rules = PoliticalGeographyDetailedRules::new();
        assert_eq!(rules.territory_rules().len(), 8);
        assert_eq!(rules.boundary_rules().len(), 8);
        assert_eq!(rules.geopolitics_rules().len(), 8);
        assert_eq!(rules.administrative_division_rules().len(), 8);
        assert_eq!(rules.international_organization_rules().len(), 8);
        assert_eq!(rules.military_geography_rules().len(), 8);
        assert_eq!(rules.electoral_geography_rules().len(), 8);
        assert_eq!(rules.political_pattern_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_political_regions() {
        let rules = PoliticalGeographyDetailedRules::new();
        assert_eq!(rules.major_political_regions().len(), 10);
    }

    #[test]
    fn test_boundary_rules() {
        let rules = PoliticalGeographyDetailedRules::new();
        let laws = rules.boundary_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("边界")));
    }

    #[test]
    fn test_research_methods() {
        let rules = PoliticalGeographyDetailedRules::new();
        assert_eq!(rules.research_methods().len(), 8);
    }
}