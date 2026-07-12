//! 海洋学详细规则
//!
//! 海洋学研究海洋的物理、化学、生物和地质特性，
//! 包括海洋环流、海洋化学、海洋生态系统和海底地质。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 海洋学详细规则集合
pub struct OceanographyDetailedRules {
    metadata: RuleMetadata,
}

impl OceanographyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("海洋学详细规则", "海洋学基本定律和海洋系统")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "海洋".into(), "地球".into()]),
        }
    }

    /// 海洋环流规则
    pub fn ocean_circulation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("风生环流定律", "风力驱动", "表层海水受风应力驱动形成环流"),
            ("热盐环流定律", "密度驱动", "海水密度差异驱动的深层环流"),
            ("暖流定律", "暖水输送", "暖流向高纬度输送热量"),
            ("寒流定律", "冷水输送", "寒流从高纬度向低纬度输送冷水"),
            ("西边界流定律", "强化流", "西边界强化的洋流如黑潮"),
            ("东边界流定律", "宽弱流", "东边界宽阔缓慢的洋流"),
            ("赤道流定律", "纬向流", "赤道地区的纬向洋流系统"),
            ("深层环流定律", "底层运动", "深海底层的缓慢环流"),
        ]
    }

    /// 海洋化学规则
    pub fn ocean_chemistry_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("盐度定律", "35‰", "海洋平均盐度约为千分之三十五"),
            ("溶解氧定律", "氧气分布", "海水中溶解氧的垂直分布"),
            ("二氧化碳定律", "碳循环", "海洋吸收大气二氧化碳"),
            ("营养盐定律", "生物必需", "氮磷硅等营养盐分布"),
            ("酸碱度定律", "pH值", "海水pH值约为8.1"),
            ("微量元素定律", "痕量元素", "海水中微量金属分布"),
            ("有机物定律", "有机碳", "海洋有机物的来源和循环"),
            ("海水成分定律", "元素比例", "海水主要元素的比例关系"),
        ]
    }

    /// 海洋物理规则
    pub fn ocean_physics_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("海水密度定律", "温盐决定", "密度由温度和盐度决定"),
            ("海水温度定律", "温度分布", "海水温度垂直和水平分布"),
            ("海水压力定律", "深度增加", "压力随深度线性增加"),
            ("声速定律", "声道分布", "声速随深度变化的声道"),
            ("光衰减定律", "光穿透", "光线在海水中衰减规律"),
            ("海水运动定律", "波动流动", "海水的波动和流动特性"),
            ("海冰定律", "冰形成", "海冰的形成和演变过程"),
            ("混合定律", "水体混合", "不同水团的混合过程"),
        ]
    }

    /// 海洋波浪规则
    pub fn ocean_wave_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("风浪定律", "风生成", "风直接作用于海面产生波浪"),
            ("涌浪定律", "传播衰减", "离开生成区传播的波浪"),
            ("波高定律", "能量度量", "波浪高度的统计特征"),
            ("波周期定律", "时间特征", "波浪周期的分布规律"),
            ("波浪折射定律", "地形影响", "波浪遇地形变化方向"),
            ("波浪绕射定律", "障碍影响", "波浪绕过障碍物传播"),
            ("波浪破碎定律", "浅水效应", "波浪进入浅水区破碎"),
            ("内波定律", "内部波动", "海水密度界面上的波动"),
        ]
    }

    /// 海洋潮汐规则
    pub fn ocean_tide_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("潮汐引力定律", "天体引力", "月球和太阳引力产生潮汐"),
            ("潮汐周期定律", "周期性", "潮汐的日周期和月周期"),
            ("潮差定律", "高度差异", "高潮和低潮的高度差"),
            ("潮汐类型定律", "潮型分类", "半日潮、全日潮和混合潮"),
            ("潮汐预报定律", "预测方法", "潮汐时间高度的预报技术"),
            ("潮流定律", "潮汐流", "潮汐引起的海水流动"),
            ("潮汐摩擦定律", "能量损耗", "潮汐摩擦消耗能量"),
            ("潮汐共振定律", "海湾共振", "海湾潮汐的共振放大"),
        ]
    }

    /// 海洋生态系统规则
    pub fn marine_ecosystem_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("海洋生产力定律", "初级生产", "海洋浮游植物初级生产力"),
            ("海洋食物链定律", "营养级", "海洋食物链和食物网结构"),
            ("珊瑚礁定律", "珊瑚生态", "珊瑚礁的形成和生态系统"),
            ("深海生态定律", "深海环境", "深海生物群落特征"),
            ("海洋保护区定律", "保护策略", "海洋生态保护区设计"),
            ("海洋污染定律", "污染影响", "海洋污染物和生态影响"),
            ("海洋缺氧定律", "缺氧区", "海洋缺氧区形成和影响"),
            ("海洋富营养化定律", "营养过剩", "营养盐过剩导致的问题"),
        ]
    }

    /// 海底地质规则
    pub fn seabed_geology_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("大陆架定律", "浅水区", "大陆边缘的浅水平台"),
            ("大陆坡定律", "陡坡区", "大陆架到深海盆的过渡"),
            ("深海平原定律", "平坦区", "深海盆地的平坦海底"),
            ("海底山脉定律", "海山分布", "海底火山和山脉系统"),
            ("海沟定律", "最深区", "板块碰撞形成的深海沟"),
            ("海底沉积定律", "沉积类型", "海底沉积物的类型分布"),
            ("海底扩张定律", "新地壳", "洋中脊产生新海底地壳"),
            ("海底热液定律", "热液喷口", "海底热液喷口生态系统"),
        ]
    }

    /// 海洋观测规则
    pub fn marine_observation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("海洋调查定律", "调查方法", "海洋科学调查的基本方法"),
            ("海洋遥感定律", "卫星观测", "卫星遥感海洋观测技术"),
            ("海洋浮标定律", "定点观测", "浮标定点海洋观测系统"),
            ("海洋潜标定律", "水下观测", "水下潜标观测系统"),
            ("海洋船舶定律", "船基观测", "海洋调查船观测方法"),
            ("海洋声学定律", "声学探测", "声学方法探测海洋"),
            ("海洋钻探定律", "海底钻探", "海底科学钻探技术"),
            ("海洋自动化定律", "智能观测", "自动化海洋观测技术"),
        ]
    }
}

impl Default for OceanographyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OceanographyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("oceanography_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_titled_sections(
            "海洋学详细规则",
            &[
                ("海洋环流", &self.ocean_circulation_rules()),
                ("海洋化学", &self.ocean_chemistry_rules()),
                ("海洋物理", &self.ocean_physics_rules()),
                ("海洋波浪", &self.ocean_wave_rules()),
                ("海洋潮汐", &self.ocean_tide_rules()),
                ("海洋生态", &self.marine_ecosystem_rules()),
                ("海底地质", &self.seabed_geology_rules()),
                ("海洋观测", &self.marine_observation_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oceanography_detailed_rules() {
        let rules = OceanographyDetailedRules::new();
        assert_eq!(rules.metadata().name, "海洋学详细规则");
        assert_eq!(rules.ocean_circulation_rules().len(), 8);
        assert_eq!(rules.ocean_chemistry_rules().len(), 8);
        assert_eq!(rules.ocean_physics_rules().len(), 8);
        assert_eq!(rules.ocean_wave_rules().len(), 8);
        assert_eq!(rules.ocean_tide_rules().len(), 8);
        assert_eq!(rules.marine_ecosystem_rules().len(), 8);
        assert_eq!(rules.seabed_geology_rules().len(), 8);
        assert_eq!(rules.marine_observation_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_oceanography_category() {
        let rules = OceanographyDetailedRules::new();
        assert_eq!(rules.category().to_string(), "Science/oceanography_detailed");
    }

    #[test]
    fn test_oceanography_validate() {
        let rules = OceanographyDetailedRules::new();
        let ctx = crate::rules::core::ValidateContext::generic("test");
        assert!(rules.validate(&ctx).is_ok());
    }
}
