//! 文化地理规则
//!
//! 文化地理学研究文化的空间分布、文化区域和文化景观，
//! 包括文化传播、文化扩散、文化区域和文化与环境关系。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 文化地理规则集合
pub struct CulturalGeographyDetailedRules {
    metadata: RuleMetadata,
}

impl CulturalGeographyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("文化地理规则", "文化空间分布和文化景观规律")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "文化".into()]),
        }
    }

    /// 文化区域规则
    pub fn cultural_region_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("文化区定律", "文化区域", "文化区域划分特征"),
            ("文化核心定律", "文化中心", "文化核心区域特征"),
            ("文化边界定律", "文化边界", "文化边界过渡地带"),
            ("文化地带定律", "文化地带", "文化地带分布规律"),
            ("文化扩散定律", "文化传播", "文化扩散方式类型"),
            ("文化源地定律", "文化起源", "文化起源中心区域"),
            ("文化区系定律", "区系类型", "文化区系类型划分"),
            ("文化景观定律", "文化景观", "文化景观特征类型"),
        ]
    }

    /// 文化传播规则
    pub fn cultural_diffusion_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("扩展扩散定律", "接触传播", "文化接触邻近传播"),
            ("迁移扩散定律", "人口迁移", "文化随人口迁移传播"),
            ("等级扩散定律", "等级传播", "文化沿等级传播"),
            ("传染扩散定律", "人际传播", "文化人际接触传播"),
            ("刺激扩散定律", "刺激传播", "文化刺激传播方式"),
            ("文化阻力定律", "传播阻力", "文化传播阻力因素"),
            ("文化路径定律", "传播路径", "文化传播路径规律"),
            ("文化速度定律", "传播速度", "文化传播速度因素"),
        ]
    }

    /// 文化生态规则
    pub fn cultural_ecology_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("文化适应定律", "环境适应", "文化适应自然环境"),
            ("环境影响定律", "环境影响", "环境对文化的影响"),
            ("文化决定定律", "环境决定", "环境决定文化特征"),
            ("文化可能定律", "可能主义", "环境可能影响文化"),
            ("文化生态定律", "生态系统", "文化生态系统关系"),
            ("文化承载力定律", "文化承载", "环境文化承载力"),
            ("文化变迁定律", "文化变化", "文化变迁环境因素"),
            ("文化多样性定律", "多样形成", "文化多样性成因"),
        ]
    }

    /// 语言地理规则
    pub fn language_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("语言分布定律", "语言分布", "语言地理分布格局"),
            ("语言区定律", "语言区域", "语言区域划分特征"),
            ("语言扩散定律", "语言传播", "语言扩散传播规律"),
            ("语言接触定律", "语言接触", "语言接触影响变化"),
            ("语言灭绝定律", "语言消失", "语言灭绝消失原因"),
            ("方言定律", "方言分布", "方言地理分布特征"),
            ("语言地图定律", "语言地图", "语言地图绘制方法"),
            ("语言多样性定律", "语言丰富", "语言多样性分布"),
        ]
    }

    /// 宗教地理规则
    pub fn religion_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("宗教分布定律", "宗教分布", "宗教地理分布格局"),
            ("宗教区定律", "宗教区域", "宗教区域划分特征"),
            ("宗教扩散定律", "宗教传播", "宗教传播扩散规律"),
            ("宗教圣地定律", "圣地分布", "宗教圣地地理分布"),
            ("宗教建筑定律", "宗教建筑", "宗教建筑地理特征"),
            ("宗教冲突定律", "宗教冲突", "宗教冲突地理因素"),
            ("宗教多元定律", "多元宗教", "宗教多元地区特征"),
            ("宗教世俗化定律", "世俗变化", "宗教世俗化地理"),
        ]
    }

    /// 民族地理规则
    pub fn ethnic_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("民族分布定律", "民族分布", "民族地理分布格局"),
            ("民族区定律", "民族区域", "民族区域划分特征"),
            ("民族迁移定律", "民族迁移", "民族迁移历史规律"),
            ("民族聚居定律", "聚居分布", "民族聚居分布特征"),
            ("民族散居定律", "散居分布", "民族散居分布特征"),
            ("民族边界定律", "民族边界", "民族边界划分特征"),
            ("民族冲突定律", "民族冲突", "民族冲突地理因素"),
            ("民族融合定律", "民族融合", "民族融合地理过程"),
        ]
    }

    /// 人口地理规则
    pub fn population_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("人口分布定律", "人口分布", "人口地理分布格局"),
            ("人口密度定律", "密度差异", "人口密度空间差异"),
            ("人口迁移定律", "人口迁移", "人口迁移原因规律"),
            ("人口增长定律", "人口增长", "人口增长空间差异"),
            ("人口结构定律", "人口结构", "人口结构空间特征"),
            ("人口城市化定律", "城市人口", "人口城市化过程"),
            ("人口老龄化定律", "老龄分布", "人口老龄化分布"),
            ("人口承载力定律", "人口承载", "人口承载力评估"),
        ]
    }

    /// 聚落地理规则
    pub fn settlement_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("聚落分布定律", "聚落分布", "聚落地理分布规律"),
            ("聚落类型定律", "聚落类型", "聚落类型划分特征"),
            ("聚落选址定律", "聚落选址", "聚落选址影响因素"),
            ("聚落形态定律", "聚落形态", "聚落空间形态特征"),
            ("聚落演变定律", "聚落变化", "聚落演变发展过程"),
            ("乡村聚落定律", "乡村分布", "乡村聚落分布特征"),
            ("城镇聚落定律", "城镇分布", "城镇聚落分布规律"),
            ("聚落功能定律", "聚落功能", "聚落功能类型特征"),
        ]
    }

    /// 主要文化区域
    pub fn major_cultural_regions(&self) -> Vec<&'static str> {
        vec![
            "东亚文化区: 中华文化影响区域",
            "东南亚文化区: 多元文化交汇区域",
            "南亚文化区: 印度文化影响区域",
            "西亚文化区: 阿拉伯伊斯兰文化区",
            "欧洲文化区: 西方基督教文化区",
            "非洲文化区: 多元传统非洲文化",
            "北美文化区: 美国加拿大文化区",
            "拉美文化区: 拉丁美洲文化区",
            "大洋洲文化区: 澳大利亚新西兰文化",
            "极地文化区: 极地原住民文化",
        ]
    }

    /// 文化地理研究方法
    pub fn research_methods(&self) -> Vec<&'static str> {
        vec![
            "文化调查: 文化现象野外调查方法",
            "文化地图: 文化分布地图绘制方法",
            "文化比较: 文化比较分析方法",
            "文化史研究: 文化历史地理研究",
            "文化统计: 文化统计数据分析",
            "文化GIS: GIS文化空间分析",
            "文化访谈: 文化访谈调查方法",
            "文化文献: 文化文献资料分析",
        ]
    }
}

impl Default for CulturalGeographyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CulturalGeographyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("cultural_geography_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【文化地理规则】\n\n\
            文化区域规则:\n{}\n\n\
            文化传播规则:\n{}\n\n\
            文化生态规则:\n{}\n\n\
            语言地理规则:\n{}\n\n\
            宗教地理规则:\n{}\n\n\
            民族地理规则:\n{}\n\n\
            人口地理规则:\n{}\n\n\
            聚落地理规则:\n{}\n\n\
            主要文化区域:\n{}\n\n\
            文化地理研究方法:\n{}",
            self.cultural_region_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_diffusion_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_ecology_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.language_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.religion_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.ethnic_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.population_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.settlement_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.major_cultural_regions()
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
    fn test_cultural_geography_detailed_rules() {
        let rules = CulturalGeographyDetailedRules::new();
        assert_eq!(rules.cultural_region_rules().len(), 8);
        assert_eq!(rules.cultural_diffusion_rules().len(), 8);
        assert_eq!(rules.cultural_ecology_rules().len(), 8);
        assert_eq!(rules.language_geography_rules().len(), 8);
        assert_eq!(rules.religion_geography_rules().len(), 8);
        assert_eq!(rules.ethnic_geography_rules().len(), 8);
        assert_eq!(rules.population_geography_rules().len(), 8);
        assert_eq!(rules.settlement_geography_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_cultural_regions() {
        let rules = CulturalGeographyDetailedRules::new();
        assert_eq!(rules.major_cultural_regions().len(), 10);
    }

    #[test]
    fn test_diffusion_rules() {
        let rules = CulturalGeographyDetailedRules::new();
        let laws = rules.cultural_diffusion_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("扩散")));
    }

    #[test]
    fn test_research_methods() {
        let rules = CulturalGeographyDetailedRules::new();
        assert_eq!(rules.research_methods().len(), 8);
    }
}
