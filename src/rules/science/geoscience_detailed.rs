//! 地球科学综合详细规则
//!
//! 地球科学综合研究地球系统的整体特性，
//! 包括地球系统科学、全球变化、人地关系和可持续发展。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 地球科学综合详细规则集合
pub struct GeoscienceDetailedRules {
    metadata: RuleMetadata,
}

impl GeoscienceDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("地球科学综合详细规则", "地球系统科学和全球变化")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地球".into(), "系统".into()]),
        }
    }

    /// 地球系统科学规则
    pub fn earth_system_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地球圈层定律", "五大圈层", "大气圈水圈岩石圈生物圈冰雪圈"),
            ("圈层相互作用定律", "相互影响", "各圈层之间的相互作用"),
            ("地球系统定律", "整体系统", "地球作为整体系统运作"),
            ("系统耦合定律", "紧密联系", "系统各部分的紧密联系"),
            ("反馈机制定律", "反馈作用", "系统的正负反馈机制"),
            ("临界点定律", "临界转变", "系统临界点突变"),
            ("系统稳态定律", "动态平衡", "系统的动态平衡状态"),
            ("系统演化定律", "演化过程", "地球系统的演化过程"),
        ]
    }

    /// 全球变化规则
    pub fn global_change_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("气候变化定律", "全球变暖", "全球气候系统的变化"),
            ("温室效应定律", "增温效应", "温室气体引起的增温"),
            ("碳排放定律", "CO2排放", "人类活动的碳排放"),
            ("气候变化影响定律", "多方面影响", "气候变化的多方面影响"),
            ("气候适应定律", "适应措施", "适应气候变化的措施"),
            ("气候减缓定律", "减缓措施", "减缓气候变化的措施"),
            ("气候预测定律", "预测方法", "气候变化的预测方法"),
            ("气候不确定性定律", "不确定性", "气候预测的不确定性"),
        ]
    }

    /// 环境变化规则
    pub fn environmental_change_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("环境污染定律", "污染扩散", "环境污染的扩散规律"),
            ("生态退化定律", "生态系统", "生态系统退化过程"),
            ("土地退化定律", "土地质量", "土地质量退化过程"),
            ("水资源变化定律", "水资源量", "水资源数量质量变化"),
            ("生物多样性定律", "生物变化", "生物多样性的变化"),
            ("空气质量定律", "大气污染", "空气质量的变化"),
            ("海洋污染定律", "海洋环境", "海洋环境污染"),
            ("环境恢复定律", "环境修复", "环境修复的可能性"),
        ]
    }

    /// 人地关系规则
    pub fn human_environment_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("人地互动定律", "双向影响", "人类与环境相互影响"),
            ("人口压力定律", "人口增长", "人口增长对环境压力"),
            ("资源消耗定律", "资源利用", "人类对资源的消耗"),
            ("环境影响定律", "环境效应", "人类活动的环境影响"),
            ("环境容量定律", "承载力", "环境的承载能力"),
            ("可持续发展定律", "持续发展", "可持续的发展模式"),
            ("生态文明定律", "生态理念", "生态文明的理念实践"),
            ("环境伦理定律", "伦理责任", "人类的环境伦理责任"),
        ]
    }

    /// 自然灾害规则
    pub fn natural_disasters_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("灾害类型定律", "灾害分类", "自然灾害的类型分类"),
            ("灾害成因定律", "灾害原因", "自然灾害的形成原因"),
            ("灾害链定律", "灾害链生", "灾害的链生效应"),
            ("灾害风险定律", "风险评估", "灾害风险的评估方法"),
            ("灾害预警定律", "预警系统", "灾害预警技术系统"),
            ("灾害防御定律", "防御措施", "灾害防御的措施方法"),
            ("灾害响应定律", "应急响应", "灾害的应急响应"),
            ("灾害恢复定律", "灾后恢复", "灾害后的恢复重建"),
        ]
    }

    /// 地球观测规则
    pub fn earth_observation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("卫星遥感定律", "遥感技术", "卫星遥感观测地球"),
            ("地面观测定律", "地面站点", "地面观测站点网络"),
            ("航空观测定律", "航空测量", "航空遥感观测方法"),
            ("海洋观测定律", "海洋监测", "海洋观测技术方法"),
            ("大气观测定律", "大气探测", "大气探测技术方法"),
            ("地球物理探测定律", "地球物理", "地球物理探测技术"),
            ("综合观测定律", "多技术结合", "多种观测技术综合"),
            ("地球大数据定律", "数据管理", "地球观测数据管理"),
        ]
    }

    /// 全球治理规则
    pub fn global_governance_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("国际环境协定定律", "国际协议", "国际环境协议框架"),
            ("全球合作定律", "国际合作", "全球环境国际合作"),
            ("环境治理定律", "治理体系", "环境治理的制度体系"),
            ("低碳发展定律", "低碳转型", "全球低碳发展路径"),
            ("绿色经济定律", "经济转型", "绿色经济转型模式"),
            ("技术转让定律", "技术合作", "环境技术转让合作"),
            ("资金支持定律", "资金机制", "环境治理的资金机制"),
            ("能力建设定律", "能力提升", "环境治理能力建设"),
        ]
    }

    /// 未来地球规则
    pub fn future_earth_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地球未来定律", "未来研究", "地球未来发展趋势研究"),
            ("情景分析定律", "情景方法", "未来情景分析方法"),
            ("风险评估定律", "风险研究", "未来风险的评估研究"),
            ("适应策略定律", "适应方法", "适应未来变化的策略"),
            ("减缓策略定律", "减缓方法", "减缓负面影响的策略"),
            ("转型路径定律", "转型研究", "社会转型路径研究"),
            ("技术创新定律", "技术进步", "未来技术进步方向"),
            ("人类福祉定律", "福祉目标", "人类福祉的发展目标"),
        ]
    }
}

impl Default for GeoscienceDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GeoscienceDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("geoscience_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_titled_sections(
            "地球科学综合详细规则",
            &[
                ("地球系统", &self.earth_system_rules()),
                ("全球变化", &self.global_change_rules()),
                ("环境变化", &self.environmental_change_rules()),
                ("人地关系", &self.human_environment_rules()),
                ("自然灾害", &self.natural_disasters_rules()),
                ("地球观测", &self.earth_observation_rules()),
                ("全球治理", &self.global_governance_rules()),
                ("未来地球", &self.future_earth_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geoscience_detailed_rules() {
        let rules = GeoscienceDetailedRules::new();
        assert_eq!(rules.metadata().name, "地球科学综合详细规则");
        assert_eq!(rules.earth_system_rules().len(), 8);
        assert_eq!(rules.global_change_rules().len(), 8);
        assert_eq!(rules.environmental_change_rules().len(), 8);
        assert_eq!(rules.human_environment_rules().len(), 8);
        assert_eq!(rules.natural_disasters_rules().len(), 8);
        assert_eq!(rules.earth_observation_rules().len(), 8);
        assert_eq!(rules.global_governance_rules().len(), 8);
        assert_eq!(rules.future_earth_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_geoscience_category() {
        let rules = GeoscienceDetailedRules::new();
        assert_eq!(rules.category().domain, "science");
        assert_eq!(rules.category().name, "geoscience_detailed");
    }

    #[test]
    fn test_geoscience_validate() {
        let rules = GeoscienceDetailedRules::new();
        let ctx = crate::rules::core::ValidateContext::default();
        assert!(rules.validate(&ctx).is_ok());
    }
}
