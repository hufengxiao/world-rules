//! 土壤地理规则
//!
//! 土壤地理学研究土壤的形成、分类、分布和演化规律，
//! 包括土壤类型、土壤性质、土壤分布和土壤利用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 土壤地理规则集合
pub struct SoilGeographyRules {
    metadata: RuleMetadata,
}

impl SoilGeographyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("土壤地理规则", "土壤形成分布和分类规律")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "土壤".into()]),
        }
    }

    /// 土壤形成规则
    pub fn soilformation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("风化定律", "岩石风化", "岩石风化形成土壤母质"),
            ("成土过程定律", "土壤发育", "土壤形成发育过程"),
            ("土壤成熟定律", "发育阶段", "土壤成熟度判别"),
            ("有机质积累定律", "有机质形成", "土壤有机质积累规律"),
            ("矿物转化定律", "矿物演变", "土壤矿物转化过程"),
            ("淋溶沉积定律", "物质迁移", "土壤物质淋溶沉积"),
            ("土壤剖面定律", "剖面发育", "土壤剖面层次发育"),
            ("土壤演化定律", "演变过程", "土壤随时间演化规律"),
        ]
    }

    /// 土壤分类规则
    pub fn soil_classification_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("土壤诊断层定律", "诊断特征", "土壤诊断层划分依据"),
            ("中国土壤分类", "系统分类", "中国土壤系统分类"),
            ("美国土壤分类", "ST系统", "美国土壤分类系统"),
            ("FAO土壤分类", "世界分类", "FAO世界土壤分类"),
            ("土壤类型定律", "类型特征", "各土壤类型特征"),
            ("土壤命名定律", "命名规则", "土壤命名方法体系"),
            ("土壤等级定律", "分类等级", "土壤分类等级层次"),
            ("土壤相似定律", "相似类型", "相似土壤类型判别"),
        ]
    }

    /// 土壤性质规则
    pub fn soil_properties_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("土壤质地定律", "颗粒组成", "土壤颗粒组成分类"),
            ("土壤结构定律", "团聚体", "土壤结构类型特征"),
            ("土壤孔隙定律", "孔隙度", "土壤孔隙度和分布"),
            ("土壤密度定律", "容重密度", "土壤容重密度特征"),
            ("土壤水分定律", "水分特性", "土壤水分持水特性"),
            ("土壤空气定律", "通气性", "土壤通气特性规律"),
            ("土壤温度定律", "热量特性", "土壤温度热量特征"),
            ("土壤颜色定律", "颜色特征", "土壤颜色判别方法"),
        ]
    }

    /// 土壤化学规则
    pub fn soil_chemical_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("土壤酸碱定律", "pH值", "土壤酸碱度影响因素"),
            ("土壤养分定律", "养分含量", "土壤养分含量分布"),
            ("土壤盐分定律", "盐渍化", "土壤盐分含量规律"),
            ("土壤交换定律", "离子交换", "土壤离子交换吸附"),
            ("土壤有机质定律", "有机含量", "土壤有机质含量"),
            ("土壤矿物定律", "矿物组成", "土壤矿物组成类型"),
            ("土壤微量元素定律", "微量元素", "土壤微量元素分布"),
            ("土壤污染定律", "污染元素", "土壤污染元素含量"),
        ]
    }

    /// 土壤分布规则
    pub fn soil_distribution_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("纬度地带定律", "纬度分布", "土壤随纬度变化分布"),
            ("经度地带定律", "经度分布", "土壤随经度变化分布"),
            ("垂直地带定律", "高度分布", "土壤垂直分布规律"),
            ("区域土壤定律", "区域特征", "区域土壤类型分布"),
            ("地形土壤定律", "地形影响", "地形对土壤分布影响"),
            ("母质土壤定律", "母质影响", "成土母质影响土壤"),
            ("气候土壤定律", "气候影响", "气候对土壤形成影响"),
            ("生物土壤定律", "生物影响", "生物对土壤形成影响"),
        ]
    }

    /// 土壤利用规则
    pub fn soil_utilization_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("土壤农业定律", "农业利用", "土壤农业利用评价"),
            ("土壤适宜定律", "适宜性评价", "土壤适宜性评价方法"),
            ("土壤肥力定律", "肥力等级", "土壤肥力等级划分"),
            ("土壤改良定律", "改良措施", "土壤改良技术方法"),
            ("土壤保护定律", "保护策略", "土壤保护措施方法"),
            ("土壤退化定律", "退化类型", "土壤退化类型特征"),
            ("土壤侵蚀定律", "侵蚀强度", "土壤侵蚀强度分级"),
            ("土壤承载定律", "承载力", "土壤承载力评价"),
        ]
    }

    /// 主要土壤类型
    pub fn major_soil_types(&self) -> Vec<&'static str> {
        vec![
            "红壤: 热带亚热带湿润区酸性土壤",
            "黄壤: 亚热带湿润山地土壤",
            "黄棕壤: 亚热带暖温带过渡区土壤",
            "棕壤: 暖温带湿润区土壤",
            "暗棕壤: 温带湿润区森林土壤",
            "黑土: 温带半湿润草原土壤",
            "黑钙土: 温带半干旱草原土壤",
            "栗钙土: 温带半干旱草原土壤",
            "灰钙土: 温带干旱区土壤",
            "荒漠土: 干旱荒漠区土壤",
        ]
    }

    /// 土壤研究方法
    pub fn research_methods(&self) -> Vec<&'static str> {
        vec![
            "土壤调查: 野外土壤调查和采样",
            "土壤分析: 室内土壤理化性质分析",
            "土壤制图: 编制土壤类型分布图",
            "土壤评价: 土壤质量适宜性评价",
            "土壤监测: 土壤变化动态监测",
            "土壤遥感: 遥感技术土壤识别",
            "土壤GIS: GIS空间分析土壤数据",
            "土壤模型: 土壤形成演化模型",
        ]
    }

    /// 土壤问题
    pub fn soil_problems(&self) -> Vec<&'static str> {
        vec![
            "土壤侵蚀: 水力风力侵蚀土壤流失",
            "土壤退化: 土壤质量下降功能衰退",
            "土壤污染: 重金属农药污染土壤",
            "土壤盐渍化: 盐分积累土壤盐化",
            "土壤酸化: 酸雨和不合理施肥酸化",
            "土壤沙化: 沙漠化土壤沙化过程",
            "土壤板结: 土壤结构破坏板结硬化",
            "土壤贫瘠化: 养分流失土壤贫瘠",
        ]
    }
}

impl Default for SoilGeographyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SoilGeographyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("soil_geography")
    }

    fn explain(&self) -> String {
        format!(
            "【土壤地理规则】\n\n\
            土壤形成规则:\n{}\n\n\
            土壤分类规则:\n{}\n\n\
            土壤性质规则:\n{}\n\n\
            土壤化学规则:\n{}\n\n\
            土壤分布规则:\n{}\n\n\
            土壤利用规则:\n{}\n\n\
            主要土壤类型:\n{}\n\n\
            土壤研究方法:\n{}\n\n\
            土壤问题:\n{}",
            self.soilformation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.soil_classification_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.soil_properties_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.soil_chemical_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.soil_distribution_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.soil_utilization_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.major_soil_types()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.research_methods()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n"),
            self.soil_problems()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soil_geography_rules() {
        let rules = SoilGeographyRules::new();
        assert_eq!(rules.soilformation_rules().len(), 8);
        assert_eq!(rules.soil_classification_rules().len(), 8);
        assert_eq!(rules.soil_properties_rules().len(), 8);
        assert_eq!(rules.soil_chemical_rules().len(), 8);
        assert_eq!(rules.soil_distribution_rules().len(), 8);
        assert_eq!(rules.soil_utilization_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_soil_types() {
        let rules = SoilGeographyRules::new();
        assert_eq!(rules.major_soil_types().len(), 10);
    }

    #[test]
    fn test_classification() {
        let rules = SoilGeographyRules::new();
        let laws = rules.soil_classification_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("分类")));
    }

    #[test]
    fn test_problems() {
        let rules = SoilGeographyRules::new();
        assert_eq!(rules.soil_problems().len(), 8);
    }
}
