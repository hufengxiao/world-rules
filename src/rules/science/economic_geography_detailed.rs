//! 经济地理规则
//!
//! 经济地理学研究经济活动的空间分布、空间组织和空间关系，
//! 包括产业布局、区域发展、经济空间结构和经济全球化。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 经济地理规则集合
pub struct EconomicGeographyDetailedRules {
    metadata: RuleMetadata,
}

impl EconomicGeographyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("经济地理规则", "经济活动空间分布和布局规律")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "经济".into()]),
        }
    }

    /// 产业布局规则
    pub fn industrial_layout_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("区位选择定律", "区位因素", "企业区位选择影响因素"),
            ("集聚定律", "产业集聚", "产业集聚效应和类型"),
            ("分散定律", "产业分散", "产业分散布局因素"),
            ("区位优势定律", "优势区位", "区位优势类型特征"),
            ("产业集群定律", "集群发展", "产业集群形成演化"),
            ("产业转移定律", "产业迁移", "产业转移规律原因"),
            ("区位理论定律", "区位理论", "区位理论模型类型"),
            ("产业关联定律", "产业链条", "产业关联链条关系"),
        ]
    }

    /// 农业地理规则
    pub fn agricultural_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("农业区位定律", "杜能圈", "农业区位同心圈结构"),
            ("农业类型定律", "类型分布", "农业类型区域分布"),
            ("农业集约定律", "集约程度", "农业集约化程度"),
            ("农业专业化定律", "专业化区", "农业专业化区域"),
            ("农业商品化定律", "商品农业", "农业商品化程度"),
            ("农业现代化定律", "现代化", "农业现代化特征"),
            ("农业可持续定律", "持续发展", "农业可持续发展"),
            ("农业变迁定律", "农业演变", "农业结构和布局变化"),
        ]
    }

    /// 工业地理规则
    pub fn industrial_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("工业区位定律", "韦伯区位", "工业区位最小成本"),
            ("工业类型定律", "工业类型", "工业类型分布特征"),
            ("工业集聚定律", "工业区集聚", "工业区集聚类型"),
            ("工业园区定律", "园区布局", "工业园区布局特征"),
            ("工业扩散定律", "工业扩散", "工业扩散布局因素"),
            ("高新技术定律", "高技术产业", "高技术产业区位"),
            ("传统工业定律", "传统工业", "传统工业改造转型"),
            ("工业生态定律", "生态工业", "生态工业园区建设"),
        ]
    }

    /// 商业地理规则
    pub fn commercial_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("商业区位定律", "商业选址", "商业区位选择因素"),
            ("商业中心定律", "商业中心", "商业中心等级体系"),
            ("零售地理定律", "零售分布", "零售设施空间分布"),
            ("批发地理定律", "批发布局", "批发市场布局"),
            ("电子商务定律", "电商影响", "电子商务地理影响"),
            ("消费地理定律", "消费空间", "消费活动空间特征"),
            ("商业街定律", "商业街道", "商业街布局特征"),
            ("购物中心定律", "购物中心", "购物中心布局规律"),
        ]
    }

    /// 交通地理规则
    pub fn transportation_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("交通网络定律", "网络结构", "交通网络结构类型"),
            ("交通节点定律", "节点枢纽", "交通节点枢纽功能"),
            ("交通走廊定律", "运输走廊", "交通走廊空间分布"),
            ("交通可达定律", "可达性", "交通可达性测度"),
            ("运输成本定律", "运输费用", "运输成本影响因素"),
            ("交通流量定律", "流量分布", "交通流量空间分布"),
            ("交通影响定律", "经济影响", "交通对经济的影响"),
            ("交通规划定律", "规划布局", "交通规划布局方法"),
        ]
    }

    /// 区域发展规则
    pub fn regional_development_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("区域差异定律", "区域差距", "区域发展差异特征"),
            ("区域增长定律", "增长极", "区域增长极理论"),
            ("区域均衡定律", "均衡发展", "区域均衡发展策略"),
            ("区域分工定律", "区域分工", "区域分工协作关系"),
            ("区域合作定律", "区域合作", "区域合作机制模式"),
            ("区域创新定律", "创新系统", "区域创新系统建设"),
            ("区域转型定律", "产业转型", "区域产业结构转型"),
            ("区域政策定律", "区域政策", "区域发展政策工具"),
        ]
    }

    /// 经济全球化规则
    pub fn globalization_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("全球化影响定律", "全球影响", "全球化对地理影响"),
            ("全球生产定律", "全球生产", "全球生产网络布局"),
            ("全球贸易定律", "国际贸易", "国际贸易空间格局"),
            ("全球投资定律", "国际投资", "国际投资流向分布"),
            ("全球金融定律", "金融中心", "全球金融中心分布"),
            ("跨国公司定律", "跨国布局", "跨国公司全球布局"),
            ("全球城市定律", "全球城市", "全球城市等级体系"),
            ("区域一体化定律", "一体化", "区域经济一体化"),
        ]
    }

    /// 经济空间结构规则
    pub fn spatial_structure_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("空间结构定律", "结构类型", "经济空间结构类型"),
            ("核心边缘定律", "核心边缘", "核心边缘结构理论"),
            ("空间扩散定律", "扩散规律", "经济活动空间扩散"),
            ("空间互动定律", "相互作用", "空间相互作用强度"),
            ("空间等级定律", "等级体系", "经济活动等级体系"),
            ("空间极化定律", "极化效应", "经济空间极化过程"),
            ("空间整合定律", "空间整合", "经济空间整合过程"),
            ("空间重构定律", "重构过程", "经济空间重构变化"),
        ]
    }

    /// 主要经济区类型
    pub fn major_economic_regions(&self) -> Vec<&'static str> {
        vec![
            "发达地区: 经济发达的高收入地区",
            "发展中地区: 正在发展的中等收入地区",
            "落后地区: 经济落后的低收入地区",
            "工业区: 工业生产密集的工业区",
            "农业区: 农业生产为主的农业区",
            "商业区: 商业贸易集中的商业区",
            "经济区: 经济联系紧密的经济区",
            "开发区: 政策支持的开发区域",
            "特区: 特殊政策的经济特区",
            "自贸区: 自由贸易的区域",
        ]
    }

    /// 经济地理研究方法
    pub fn research_methods(&self) -> Vec<&'static str> {
        vec![
            "区位分析: 企业区位选择分析方法",
            "空间分析: 经济活动空间分析方法",
            "区域分析: 区域经济发展分析方法",
            "产业分析: 产业结构布局分析方法",
            "网络分析: 经济网络结构分析方法",
            "计量分析: 经济地理计量分析方法",
            "GIS分析: GIS空间经济分析方法",
            "模型模拟: 经济地理模型模拟方法",
        ]
    }
}

impl Default for EconomicGeographyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EconomicGeographyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("economic_geography_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【经济地理规则】\n\n\
            产业布局规则:\n{}\n\n\
            农业地理规则:\n{}\n\n\
            工业地理规则:\n{}\n\n\
            商业地理规则:\n{}\n\n\
            交通地理规则:\n{}\n\n\
            区域发展规则:\n{}\n\n\
            经济全球化规则:\n{}\n\n\
            经济空间结构规则:\n{}\n\n\
            主要经济区类型:\n{}\n\n\
            经济地理研究方法:\n{}",
            self.industrial_layout_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.agricultural_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.industrial_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.commercial_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.transportation_geography_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.regional_development_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.globalization_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.spatial_structure_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.major_economic_regions()
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
    fn test_economic_geography_detailed_rules() {
        let rules = EconomicGeographyDetailedRules::new();
        assert_eq!(rules.industrial_layout_rules().len(), 8);
        assert_eq!(rules.agricultural_geography_rules().len(), 8);
        assert_eq!(rules.industrial_geography_rules().len(), 8);
        assert_eq!(rules.commercial_geography_rules().len(), 8);
        assert_eq!(rules.transportation_geography_rules().len(), 8);
        assert_eq!(rules.regional_development_rules().len(), 8);
        assert_eq!(rules.globalization_rules().len(), 8);
        assert_eq!(rules.spatial_structure_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_industrial_rules() {
        let rules = EconomicGeographyDetailedRules::new();
        let laws = rules.industrial_geography_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("工业")));
    }

    #[test]
    fn test_economic_regions() {
        let rules = EconomicGeographyDetailedRules::new();
        assert_eq!(rules.major_economic_regions().len(), 10);
    }

    #[test]
    fn test_research_methods() {
        let rules = EconomicGeographyDetailedRules::new();
        assert_eq!(rules.research_methods().len(), 8);
    }
}