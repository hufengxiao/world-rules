//! 生物地理规则
//!
//! 生物地理学研究生物的地理分布、分布成因和分布规律，
//! 包括物种分布、群落分布、生态系统分布和生物多样性。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 生物地理规则集合
pub struct BiogeographyDetailedRules {
    metadata: RuleMetadata,
}

impl BiogeographyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("生物地理规则", "生物分布和生物多样性规律")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "生物".into()]),
        }
    }

    /// 物种分布规则
    pub fn species_distribution_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("物种范围定律", "分布范围", "物种地理分布范围大小"),
            ("物种丰度定律", "种群密度", "物种种群密度分布"),
            ("物种扩散定律", "扩散机制", "物种扩散传播方式"),
            ("物种迁移定律", "迁移规律", "物种迁移路线和时机"),
            ("物种隔离定律", "地理隔离", "地理隔离影响物种分化"),
            ("物种特化定律", "特有种", "特定区域特有物种分布"),
            ("物种入侵定律", "外来物种", "外来物种入侵扩散规律"),
            ("物种灭绝定律", "灭绝风险", "物种灭绝地理因素"),
        ]
    }

    /// 群落分布规则
    pub fn community_distribution_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("群落组成定律", "物种组成", "群落物种组成结构"),
            ("群落结构定律", "空间结构", "群落空间结构层次"),
            ("群落演替定律", "演替过程", "群落演替系列阶段"),
            ("群落分布定律", "地理分布", "群落地理分布格局"),
            ("群落边界定律", "群落交错", "群落交错带特征"),
            ("群落稳定性定律", "稳定性", "群落稳定性维持机制"),
            ("群落多样性定律", "多样性指数", "群落多样性测度"),
            ("群落动态定律", "季节变化", "群落季节动态变化"),
        ]
    }

    /// 生态系统分布规则
    pub fn ecosystem_distribution_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("生态系统类型定律", "类型划分", "生态系统类型划分"),
            ("生态系统分布定律", "空间分布", "生态系统地理分布"),
            ("生态系统边界定律", "系统边界", "生态系统边界划分"),
            ("生态系统功能定律", "生态功能", "生态系统功能类型"),
            ("生态系统服务定律", "生态服务", "生态系统服务类型"),
            ("生态系统脆弱定律", "脆弱性", "生态系统脆弱性评估"),
            ("生态系统退化定律", "退化过程", "生态系统退化阶段"),
            ("生态系统恢复定律", "恢复重建", "生态系统恢复方法"),
        ]
    }

    /// 生物多样性规则
    pub fn biodiversity_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("多样性梯度定律", "纬度梯度", "生物多样性纬度梯度"),
            ("多样性热点定律", "热点区域", "生物多样性热点区域"),
            ("多样性丧失定律", "丧失因素", "生物多样性丧失原因"),
            ("多样性保护定律", "保护策略", "生物多样性保护措施"),
            ("遗传多样性定律", "基因多样性", "种群遗传多样性"),
            ("物种多样性定律", "物种丰富", "物种多样性测度"),
            ("生态系统多样性定律", "系统多样", "生态系统多样性"),
            ("多样性评估定律", "评估方法", "生物多样性评估方法"),
        ]
    }

    /// 植物地理规则
    pub fn plant_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("植物区系定律", "区系组成", "植物区系组成特征"),
            ("植被类型定律", "植被分类", "植被类型划分系统"),
            ("植被分布定律", "分布规律", "植被地理分布规律"),
            ("植物群落定律", "群落类型", "植物群落类型特征"),
            ("植被带定律", "植被带状", "植被垂直水平分布带"),
            ("植物扩散定律", "扩散方式", "植物种子扩散方式"),
            ("植物适应定律", "环境适应", "植物环境适应特征"),
            ("植物地理区定律", "地理区划", "植物地理区划系统"),
        ]
    }

    /// 动物地理规则
    pub fn animal_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("动物区系定律", "区系特征", "动物区系组成特征"),
            ("动物分布定律", "分布格局", "动物地理分布格局"),
            ("动物迁移定律", "迁移路线", "动物迁移路线规律"),
            ("动物栖息地定律", "栖息地", "动物栖息地选择"),
            ("动物扩散定律", "扩散能力", "动物扩散能力差异"),
            ("动物地理区定律", "地理区划", "动物地理区划系统"),
            ("动物适应定律", "环境适应", "动物环境适应特征"),
            ("动物群落定律", "群落结构", "动物群落结构特征"),
        ]
    }

    /// 历史生物地理规则
    pub fn historical_biogeography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("大陆漂移定律", "板块运动", "大陆漂移影响生物分布"),
            ("冰期影响定律", "冰期扩散", "冰期影响生物分布变化"),
            ("生物演化定律", "演化历史", "生物演化地理历史"),
            ("物种形成定律", "地理物种形成", "地理隔离物种形成"),
            ("生物散布定律", "散布历史", "生物散布历史过程"),
            ("灭绝事件定律", "灭绝历史", "历史灭绝事件影响"),
            ("古生物地理定律", "古地理", "古生物地理分布重建"),
            ("岛屿生物定律", "岛屿效应", "岛屿生物地理效应"),
        ]
    }

    /// 保护生物地理规则
    pub fn conservation_biogeography_rules(
        &self,
    ) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("保护区定律", "保护区设计", "保护区设计原理"),
            ("保护区网络定律", "网络布局", "保护区网络布局规划"),
            ("保护区管理定律", "管理策略", "保护区管理方法"),
            ("物种保护定律", "物种保护", "濒危物种保护措施"),
            ("栖息地保护定律", "栖息地", "栖息地保护恢复"),
            ("生态廊道定律", "廊道连接", "生态廊道规划建设"),
            ("迁地保护定律", "迁地保护", "物种迁地保护方法"),
            ("恢复生态定律", "生态恢复", "生态系统恢复重建"),
        ]
    }

    /// 主要生物群落类型
    pub fn major_biomes(&self) -> Vec<&'static str> {
        vec![
            "热带雨林: 高温多雨物种丰富森林",
            "热带季雨林: 季节性干旱热带森林",
            "热带草原: 高温干湿季分明草原",
            "亚热带森林: 温暖湿润常绿森林",
            "温带森林: 四季分明落叶阔叶林",
            "温带草原: 温带半干旱草原",
            "寒带森林: 寒冷针叶林泰加林",
            "苔原: 极寒冻原植被",
            "荒漠: 干旱稀疏植被",
            "高山植被: 高山寒冷植被",
        ]
    }

    /// 生物地理研究方法
    pub fn research_methods(&self) -> Vec<&'static str> {
        vec![
            "野外调查: 生物分布野外调查记录",
            "标本采集: 生物标本采集鉴定",
            "遥感监测: 遥感技术监测生物分布",
            "GIS分析: GIS空间分析生物分布",
            "模型模拟: 生物分布模型模拟预测",
            "分子技术: 分子技术研究生物地理",
            "统计分析: 统计分析生物分布数据",
            "历史重建: 历史生物地理重建方法",
        ]
    }
}

impl Default for BiogeographyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BiogeographyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("biogeography_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【生物地理规则】\n\n\
            物种分布规则:\n{}\n\n\
            群落分布规则:\n{}\n\n\
            生态系统分布规则:\n{}\n\n\
            生物多样性规则:\n{}\n\n\
            植物地理规则:\n{}\n\n\
            动物地理规则:\n{}\n\n\
            历史生物地理规则:\n{}\n\n\
            保护生物地理规则:\n{}\n\n\
            主要生物群落类型:\n{}\n\n\
            生物地理研究方法:\n{}",
            self.species_distribution_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.community_distribution_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.ecosystem_distribution_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.biodiversity_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.plant_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.animal_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.historical_biogeography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.conservation_biogeography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.major_biomes()
                .iter()
                .map(|b| format!("  • {}", b))
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
    fn test_biogeography_detailed_rules() {
        let rules = BiogeographyDetailedRules::new();
        assert_eq!(rules.species_distribution_rules().len(), 8);
        assert_eq!(rules.community_distribution_rules().len(), 8);
        assert_eq!(rules.ecosystem_distribution_rules().len(), 8);
        assert_eq!(rules.biodiversity_rules().len(), 8);
        assert_eq!(rules.plant_geography_rules().len(), 8);
        assert_eq!(rules.animal_geography_rules().len(), 8);
        assert_eq!(rules.historical_biogeography_rules().len(), 8);
        assert_eq!(rules.conservation_biogeography_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_major_biomes() {
        let rules = BiogeographyDetailedRules::new();
        assert_eq!(rules.major_biomes().len(), 10);
    }

    #[test]
    fn test_biodiversity() {
        let rules = BiogeographyDetailedRules::new();
        let laws = rules.biodiversity_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("多样性")));
    }

    #[test]
    fn test_research_methods() {
        let rules = BiogeographyDetailedRules::new();
        assert_eq!(rules.research_methods().len(), 8);
    }
}
