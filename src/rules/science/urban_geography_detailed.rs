//! 城市地理规则
//!
//! 城市地理学研究城市的形成、发展、结构和功能，
//! 包括城市化进程、城市空间结构、城市系统和城市规划。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 城市地理规则集合
pub struct UrbanGeographyDetailedRules {
    metadata: RuleMetadata,
}

impl UrbanGeographyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("城市地理规则", "城市发展和空间结构规律")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "城市".into()]),
        }
    }

    /// 城市化规则
    pub fn urbanization_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("城市化阶段定律", "发展阶段", "城市化发展阶段划分"),
            ("城市化速度定律", "发展速度", "城市化速度影响因素"),
            ("城市增长定律", "城市扩张", "城市空间扩张规律"),
            ("人口城市化定律", "人口迁移", "人口向城市迁移规律"),
            ("经济城市化定律", "经济转型", "城市经济结构转型"),
            ("社会城市化定律", "社会变迁", "城市社会结构变化"),
            ("景观城市化定律", "景观变化", "城市景观演变过程"),
            ("逆城市化定律", "郊区化", "逆城市化现象和原因"),
        ]
    }

    /// 城市空间结构规则
    pub fn urban_structure_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("同心圆模型定律", "同心结构", "同心圆土地利用模式"),
            ("扇形模型定律", "扇形分布", "扇形土地利用结构"),
            ("多核心模型定律", "多中心", "多核心城市结构"),
            ("城市功能分区定律", "功能分区", "城市功能分区布局"),
            ("城市密度定律", "密度分布", "城市密度梯度变化"),
            ("城市土地利用定律", "土地类型", "城市土地利用类型"),
            ("城市交通定律", "交通网络", "城市交通网络结构"),
            ("城市形态定律", "城市形状", "城市形态类型特征"),
        ]
    }

    /// 城市系统规则
    pub fn urban_system_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("城市规模定律", "规模分布", "城市规模分布规律"),
            ("城市等级定律", "等级体系", "城市等级体系结构"),
            ("中心地理论定律", "中心地", "中心地分布理论"),
            ("城市引力定律", "相互作用", "城市间引力作用"),
            ("城市首位定律", "首位度", "首位城市规模分布"),
            ("城市群定律", "城市群发展", "城市群形成和演化"),
            ("都市区定律", "都市区范围", "都市区范围界定"),
            ("城市网络定律", "网络结构", "城市网络连接结构"),
        ]
    }

    /// 城市内部功能规则
    pub fn urban_function_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("CBD定律", "中心商务区", "CBD特征和功能"),
            ("居住区定律", "居住空间", "城市居住空间分布"),
            ("工业区定律", "工业布局", "城市工业布局演变"),
            ("商业区定律", "商业分布", "城市商业设施分布"),
            ("交通枢纽定律", "枢纽布局", "城市交通枢纽分布"),
            ("公共设施定律", "设施分布", "城市公共设施布局"),
            ("绿地系统定律", "绿地分布", "城市绿地系统结构"),
            ("基础设施定律", "基础设施", "城市基础设施布局"),
        ]
    }

    /// 城市社会地理规则
    pub fn urban_social_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("社会空间定律", "社会分异", "城市社会空间分异"),
            ("居住分异定律", "居住分化", "城市居住空间分化"),
            ("阶层分布定律", "阶层分化", "城市社会阶层分布"),
            ("种族分布定律", "种族聚居", "城市种族聚居分布"),
            ("人口流动定律", "人口迁移", "城市人口流动规律"),
            ("就业分布定律", "就业空间", "城市就业空间分布"),
            ("通勤定律", "通勤模式", "城市通勤空间模式"),
            ("住房定律", "住房类型", "城市住房类型分布"),
        ]
    }

    /// 城市经济地理规则
    pub fn urban_economic_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("城市产业定律", "产业结构", "城市产业结构演变"),
            ("城市就业定律", "就业结构", "城市就业结构分布"),
            ("城市收入定律", "收入分布", "城市收入空间差异"),
            ("城市投资定律", "投资分布", "城市投资空间分布"),
            ("城市消费定律", "消费空间", "城市消费空间分布"),
            ("城市创新定律", "创新集聚", "城市创新空间集聚"),
            ("城市集聚定律", "集聚效应", "城市集聚经济效应"),
            ("城市分工定律", "城市分工", "城市间分工协作"),
        ]
    }

    /// 城市环境规则
    pub fn urban_environment_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("城市气候定律", "城市气候", "城市气候特征效应"),
            ("城市水文定律", "水文变化", "城市化水文影响"),
            ("城市生态定律", "生态系统", "城市生态系统特征"),
            ("城市污染定律", "污染分布", "城市污染空间分布"),
            ("城市绿地定律", "绿地功能", "城市绿地生态功能"),
            ("城市噪音定律", "噪音分布", "城市噪音空间分布"),
            ("城市热岛定律", "热岛效应", "城市热岛空间分布"),
            ("城市灾害定律", "灾害风险", "城市灾害风险评估"),
        ]
    }

    /// 城市规划规则
    pub fn urban_planning_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("规划编制定律", "规划体系", "城市规划编制体系"),
            ("规划实施定律", "规划执行", "城市规划实施管理"),
            ("规划评估定律", "规划评价", "城市规划效果评估"),
            ("分区规划定律", "分区管制", "城市分区管制方法"),
            ("总体规划定律", "总体规划", "城市总体规划内容"),
            ("详细规划定律", "详细规划", "城市详细规划类型"),
            ("交通规划定律", "交通规划", "城市交通规划方法"),
            ("环境规划定律", "环境规划", "城市环境规划内容"),
        ]
    }

    /// 主要城市类型
    pub fn major_city_types(&self) -> Vec<&'static str> {
        vec![
            "特大城市: 人口超千万的超大城市",
            "大城市: 人口百万以上的大城市",
            "中等城市: 人口五十万左右中等城市",
            "小城市: 人口二十万以下小城市",
            "首都城市: 国家首都政治中心城市",
            "港口城市: 港口贸易交通枢纽城市",
            "工业城市: 工业生产基地城市",
            "旅游城市: 旅游休闲度假城市",
            "历史名城: 文化历史名城",
            "新城: 新规划建设城市",
        ]
    }

    /// 城市问题
    pub fn urban_problems(&self) -> Vec<&'static str> {
        vec![
            "交通拥堵: 城市交通拥堵问题",
            "住房紧张: 城市住房供需矛盾",
            "环境污染: 城市环境污染问题",
            "就业压力: 城市就业岗位不足",
            "公共服务: 公共服务设施不足",
            "城市贫困: 城市贫困人口问题",
            "社会安全: 城市社会安全问题",
            "城市蔓延: 城市无序扩张问题",
        ]
    }
}

impl Default for UrbanGeographyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for UrbanGeographyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("urban_geography_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【城市地理规则】\n\n\
            城市化规则:\n{}\n\n\
            城市空间结构规则:\n{}\n\n\
            城市系统规则:\n{}\n\n\
            城市内部功能规则:\n{}\n\n\
            城市社会地理规则:\n{}\n\n\
            城市经济地理规则:\n{}\n\n\
            城市环境规则:\n{}\n\n\
            城市规划规则:\n{}\n\n\
            主要城市类型:\n{}\n\n\
            城市问题:\n{}",
            self.urbanization_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.urban_structure_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.urban_system_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.urban_function_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.urban_social_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.urban_economic_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.urban_environment_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.urban_planning_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.major_city_types()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.urban_problems()
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
    fn test_urban_geography_detailed_rules() {
        let rules = UrbanGeographyDetailedRules::new();
        assert_eq!(rules.urbanization_rules().len(), 8);
        assert_eq!(rules.urban_structure_rules().len(), 8);
        assert_eq!(rules.urban_system_rules().len(), 8);
        assert_eq!(rules.urban_function_rules().len(), 8);
        assert_eq!(rules.urban_social_rules().len(), 8);
        assert_eq!(rules.urban_economic_rules().len(), 8);
        assert_eq!(rules.urban_environment_rules().len(), 8);
        assert_eq!(rules.urban_planning_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_city_types() {
        let rules = UrbanGeographyDetailedRules::new();
        assert_eq!(rules.major_city_types().len(), 10);
    }

    #[test]
    fn test_structure_rules() {
        let rules = UrbanGeographyDetailedRules::new();
        let laws = rules.urban_structure_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("同心")));
    }

    #[test]
    fn test_problems() {
        let rules = UrbanGeographyDetailedRules::new();
        assert_eq!(rules.urban_problems().len(), 8);
    }
}
