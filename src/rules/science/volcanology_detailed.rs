//! 火山学详细规则
//!
//! 火山学研究火山的活动机制、火山产物和火山灾害，
//! 包括火山类型、喷发机制、火山监测和火山灾害防御。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 火山学详细规则集合
pub struct VolcanologyDetailedRules {
    metadata: RuleMetadata,
}

impl VolcanologyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("火山学详细规则", "火山学基本定律和火山系统")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "火山".into(), "地球".into()]),
        }
    }

    /// 火山类型规则
    pub fn volcano_types_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("盾状火山定律", "宽缓形态", "由低粘度熔岩形成的宽缓火山"),
            ("层状火山定律", "锥形层状", "由多层熔岩和火山灰组成"),
            ("复式火山定律", "复合结构", "多种喷发方式形成的复合火山"),
            ("火山锥定律", "小型锥体", "小型单次喷发形成的锥体"),
            ("火山口定律", "凹陷结构", "火山顶部的凹陷结构"),
            ("破火山口定律", "大型凹陷", "大规模喷发后的凹陷"),
            ("海底火山定律", "水下火山", "海底的火山活动"),
            ("裂隙火山定律", "裂隙喷发", "沿裂隙喷发的火山"),
        ]
    }

    /// 火山喷发规则
    pub fn eruption_types_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("爆炸喷发定律", "剧烈喷发", "猛烈爆炸式的喷发"),
            ("溢流喷发定律", "平静流出", "熔岩平静流出式喷发"),
            ("斯特龙博利式定律", "频繁喷发", "频繁的中小规模喷发"),
            ("夏威夷式定律", "温和喷发", "温和的熔岩溢流喷发"),
            ("普林尼式定律", "强烈爆炸", "极强烈的爆炸喷发"),
            ("火山灰喷发定律", "灰云喷发", "以火山灰为主的喷发"),
            ("熔岩喷发定律", "熔岩流", "以熔岩流为主的喷发"),
            ("混合喷发定律", "多种产物", "多种喷发产物混合"),
        ]
    }

    /// 火山产物规则
    pub fn volcanic_products_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("熔岩定律", "岩浆流出", "流出地表的岩浆物质"),
            ("火山灰定律", "细粒碎屑", "喷发的细粒火山碎屑"),
            ("火山弹定律", "熔岩团块", "喷发的熔岩团块"),
            ("火山渣定律", "粗粒碎屑", "喷发的粗粒火山碎屑"),
            ("火山气体定律", "气体释放", "火山释放的气体成分"),
            ("浮石定律", "轻质岩石", "多孔轻质的火山岩石"),
            ("火山泥流定律", "泥石流", "火山灰和水形成的泥流"),
            ("火山碎屑流定律", "热碎屑流", "高温火山碎屑流动"),
        ]
    }

    /// 火山监测规则
    pub fn volcano_monitoring_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地震监测定律", "火山地震", "监测火山周围的地震活动"),
            ("地形监测定律", "地形变化", "监测火山地形的变化"),
            ("气体监测定律", "气体变化", "监测火山气体的变化"),
            ("温度监测定律", "温度异常", "监测火山地区的温度异常"),
            ("重力监测定律", "重力变化", "监测火山地区的重力变化"),
            ("遥感监测定律", "卫星观测", "利用遥感技术监测火山"),
            ("地下水位定律", "水位变化", "监测火山周围地下水位"),
            ("电磁监测定律", "电磁变化", "监测火山电磁场变化"),
        ]
    }

    /// 火山灾害规则
    pub fn volcanic_disaster_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("熔岩流灾害定律", "熔岩破坏", "熔岩流造成的破坏灾害"),
            ("火山灰灾害定律", "灰降灾害", "火山灰降落造成的灾害"),
            ("碎屑流灾害定律", "热碎屑流", "火山碎屑流造成的灾害"),
            ("火山泥流灾害定律", "泥流灾害", "火山泥流造成的灾害"),
            ("火山气体灾害定律", "气体灾害", "火山气体造成的灾害"),
            ("火山海啸定律", "火山海啸", "火山引发的海啸灾害"),
            ("火山地震灾害定律", "地震灾害", "火山活动引起的地震"),
            ("火山气候影响定律", "气候效应", "火山对气候的影响"),
        ]
    }

    /// 火山预警规则
    pub fn volcano_warning_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("预警级别定律", "级别划分", "火山预警的级别划分"),
            ("预警信号定律", "信号发布", "火山预警信号的发布"),
            ("预警响应定律", "响应措施", "预警后的响应措施"),
            ("预警区域定律", "危险区域", "划定火山危险区域"),
            ("疏散计划定律", "疏散方案", "火山灾害疏散计划"),
            ("预警系统定律", "预警技术", "火山预警技术系统"),
            ("预警准确定律", "准确率", "火山预警的准确率"),
            ("预警时效定律", "预警时间", "火山预警的时间要求"),
        ]
    }

    /// 火山分布规则
    pub fn volcano_distribution_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("火山带定律", "带状分布", "火山沿板块边界带状分布"),
            ("环太平洋定律", "环太平洋带", "环太平洋火山带"),
            ("地中海带定律", "地中海火山", "地中海-喜马拉雅火山带"),
            ("洋中脊定律", "海底火山", "洋中脊火山活动带"),
            ("热点火山定律", "热点分布", "地幔热点形成的火山"),
            ("大陆火山定律", "大陆内部", "大陆内部的火山分布"),
            ("岛屿火山定律", "火山岛屿", "火山形成的岛屿分布"),
            ("活跃火山定律", "活动状态", "全球活跃火山分布"),
        ]
    }

    /// 火山研究规则
    pub fn volcano_research_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("火山地质定律", "地质研究", "火山地质结构和演化研究"),
            ("火山化学定律", "化学研究", "火山物质化学成分研究"),
            ("火山物理定律", "物理研究", "火山活动物理过程研究"),
            ("火山实验定律", "实验研究", "火山活动的实验研究"),
            ("火山模拟定律", "数值模拟", "火山活动的数值模拟"),
            ("火山历史定律", "历史研究", "火山喷发历史研究"),
            ("火山考古定律", "考古研究", "火山考古遗址研究"),
            ("火山预测定律", "预测研究", "火山喷发预测研究"),
        ]
    }
}

impl Default for VolcanologyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for VolcanologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("volcanology_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_titled_sections(
            "火山学详细规则",
            &[
                ("火山类型", &self.volcano_types_rules()),
                ("喷发类型", &self.eruption_types_rules()),
                ("火山产物", &self.volcanic_products_rules()),
                ("火山监测", &self.volcano_monitoring_rules()),
                ("火山灾害", &self.volcanic_disaster_rules()),
                ("火山预警", &self.volcano_warning_rules()),
                ("火山分布", &self.volcano_distribution_rules()),
                ("火山研究", &self.volcano_research_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volcanology_detailed_rules() {
        let rules = VolcanologyDetailedRules::new();
        assert_eq!(rules.metadata().name, "火山学详细规则");
        assert_eq!(rules.volcano_types_rules().len(), 8);
        assert_eq!(rules.eruption_types_rules().len(), 8);
        assert_eq!(rules.volcanic_products_rules().len(), 8);
        assert_eq!(rules.volcano_monitoring_rules().len(), 8);
        assert_eq!(rules.volcanic_disaster_rules().len(), 8);
        assert_eq!(rules.volcano_warning_rules().len(), 8);
        assert_eq!(rules.volcano_distribution_rules().len(), 8);
        assert_eq!(rules.volcano_research_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_volcanology_category() {
        let rules = VolcanologyDetailedRules::new();
        assert_eq!(rules.category().domain, "science");
        assert_eq!(rules.category().name, "volcanology_detailed");
    }

    #[test]
    fn test_volcanology_validate() {
        let rules = VolcanologyDetailedRules::new();
        let ctx = crate::rules::core::ValidateContext::default();
        assert!(rules.validate(&ctx).is_ok());
    }
}