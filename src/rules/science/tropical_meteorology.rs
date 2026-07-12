//! 热带气象学规则
//!
//! 热带气象学研究热带地区的大气现象和天气系统，
//! 包括台风、热带对流、季风和热带波动。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 热带气象学规则集合
pub struct TropicalMeteorologyRules {
    metadata: RuleMetadata,
}

impl TropicalMeteorologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("热带气象学规则", "热带天气系统和热带气旋")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "气象".into(), "热带".into()]),
        }
    }

    /// 台风规则
    pub fn typhoon_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("台风形成定律", "热带气旋", "台风形成的热力动力条件"),
            ("台风发展定律", "气旋加强", "台风发展加强机制"),
            ("台风路径定律", "移动路径", "台风移动路径预测方法"),
            ("台风强度定律", "中心强度", "台风中心强度变化规律"),
            ("台风结构定律", "内部结构", "台风眼和眼壁结构特征"),
            ("台风降水定律", "降水分布", "台风降水分布和强度"),
            ("台风风场定律", "风场结构", "台风风场分布和变化"),
            ("台风衰减定律", "气旋减弱", "台风减弱和消散过程"),
        ]
    }

    /// 热带对流规则
    pub fn tropical_convection_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热带对流形成定律", "对流发生", "热带对流发生条件"),
            ("对流云团定律", "云团发展", "热带对流云团演变"),
            ("Maddox-Julian振荡定律", "MJO", "MJO传播和影响"),
            ("热带波动定律", "波动传播", "热带波动类型和传播"),
            ("对流层顶冷却定律", "高层冷却", "对流层顶温度变化"),
            ("对流有效位能定律", "CAPE", "热带对流有效位能"),
            ("对流抑制定律", "CIN", "对流抑制能量分析"),
            ("对流触发定律", "触发机制", "热带对流触发机制"),
        ]
    }

    /// 季风规则
    pub fn monsoon_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("季风爆发定律", "季风开始", "季风爆发时间和条件"),
            ("季风撤退定律", "季风结束", "季风撤退过程分析"),
            ("季风降水定律", "季风雨", "季风降水分布特征"),
            ("季风中断定律", "中断期", "季风中断和活跃期"),
            ("季风年际变化定律", "年际差异", "季风年际变化规律"),
            ("亚洲季风定律", "亚洲季风", "亚洲季风系统特征"),
            ("季风环流定律", "季风气流", "季风环流结构分析"),
            ("季风预测定律", "季风预报", "季风预测方法技术"),
        ]
    }

    /// 热带波动规则
    pub fn tropical_waves_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("东风波定律", "波动结构", "东风波结构和移动"),
            ("Kelvin波定律", "赤道Kelvin", "赤道Kelvin波特征"),
            ("Rossby波定律", "热带Rossby", "热带Rossby波分析"),
            ("混合Rossby波定律", "混合波", "混合Rossby重力波"),
            ("非洲波定律", "东风扰动", "非洲东风波扰动"),
            ("波扰传播定律", "波动传播", "热带波动传播特征"),
            ("波扰发展定律", "波动发展", "热带波动发展条件"),
            ("波扰相互作用定律", "波波作用", "波动相互作用机制"),
        ]
    }

    /// 热带环流规则
    pub fn tropical_circulation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Hadley环流定律", "经向环流", "Hadley环流强度变化"),
            ("Walker环流定律", "纬向环流", "Walker环流位置强度"),
            ("热带辐合带定律", "ITCZ", "热带辐合带位置变化"),
            ("信风定律", "热带东风", "信风强度和变化"),
            ("赤道缓冲带定律", "缓冲带", "赤道缓冲带特征"),
            ("越赤道气流定律", "跨赤道", "越赤道气流通道"),
            ("热带高空东风定律", "TEJ", "热带东风急流分析"),
            ("热带西风定律", "季风西风", "热带西风带变化"),
        ]
    }

    /// 热带气旋分类规则
    pub fn tropical_cyclone_classification_rules(
        &self,
    ) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热带低压定律", "TD", "热带低压强度分类标准"),
            ("热带风暴定律", "TS", "热带风暴强度分类"),
            ("强热带风暴定律", "STS", "强热带风暴分类"),
            ("台风定律", "TY", "台风强度分类标准"),
            ("强台风定律", "STY", "强台风强度分类"),
            ("超强台风定律", "SuperTY", "超强台风分类"),
            ("飓风定律", "Hurricane", "大西洋飓风分类"),
            ("气旋风暴定律", "CS", "印度洋气旋风暴"),
        ]
    }

    /// 热带降水规则
    pub fn tropical_precipitation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热带雨林降水定律", "热带雨", "热带雨林降水特征"),
            ("热带风暴降水定律", "风暴降水", "热带风暴降水强度"),
            ("季风降水定律", "季风雨", "季风降水分布规律"),
            ("对流降水定律", "对流雨", "热带对流降水特征"),
            ("热带干旱定律", "干季", "热带干旱季节特征"),
            ("热带雨季定律", "雨季", "热带雨季降水特征"),
            ("热带夜雨定律", "夜雨", "热带夜间降水现象"),
            ("热带极端降水定律", "暴雨", "热带极端降水事件"),
        ]
    }

    /// 热带气旋运动规则
    pub fn tropical_cyclone_motion_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("引导气流定律", "气流引导", "台风引导气流原理"),
            ("β漂移定律", "科氏效应", "β效应导致的漂移"),
            ("相互作用定律", "藤原效应", "双台风相互作用"),
            ("地形影响定律", "地形作用", "地形对台风路径影响"),
            ("转向定律", "路径转向", "台风转向预报方法"),
            ("加速定律", "移动加速", "台风移动加速因素"),
            ("减速定律", "移动减速", "台风移动减速因素"),
            ("异常路径定律", "异常轨迹", "台风异常路径分析"),
        ]
    }

    /// 热带气旋灾害规则
    pub fn tropical_cyclone_disaster_rules(
        &self,
    ) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("台风大风定律", "风力灾害", "台风大风灾害影响"),
            ("台风暴雨定律", "暴雨灾害", "台风暴雨灾害评估"),
            ("台风风暴潮定律", "风暴潮", "台风风暴潮灾害"),
            ("台风巨浪定律", "海浪灾害", "台风引起的巨浪灾害"),
            ("台风洪水定律", "洪水灾害", "台风降水引发洪水"),
            ("台风次生灾害定律", "次生灾害", "台风引发次生灾害"),
            ("台风灾害评估定律", "灾情评估", "台风灾害损失评估"),
            ("台风防御定律", "防灾减灾", "台风灾害防御措施"),
        ]
    }

    /// 热带观测方法
    pub fn observation_methods(&self) -> Vec<&'static str> {
        vec![
            "卫星观测: 气象卫星监测热带气旋和云系",
            "雷达观测: 雷达探测热带对流和降水",
            "船舶观测: 海上气象观测站资料",
            "浮标观测: 海洋浮标气象观测",
            "飞机探测: 飞机穿入台风探测",
            "探空观测: 热带地区探空资料",
            "风廓线观测: 风廓线雷达观测风场",
            "闪电定位: 闪电定位系统监测对流",
        ]
    }

    /// 热带预报方法
    pub fn forecast_methods(&self) -> Vec<&'static str> {
        vec![
            "台风路径预报: 台风移动路径预报方法",
            "台风强度预报: 台风强度变化预报技术",
            "热带对流预报: 热带对流活动预报",
            "季风预报: 季风爆发和降水预报",
            "MJO预报: MJO活动预报方法",
            "热带降水预报: 热带地区降水预报",
            "热带气旋生成预报: 热带气旋生成预测",
            "集合预报: 热带天气预报集合方法",
        ]
    }
}

impl Default for TropicalMeteorologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TropicalMeteorologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("tropical_meteorology")
    }

    fn explain(&self) -> String {
        format!(
            "【热带气象学规则】\n\n\
            台风规则:\n{}\n\n\
            热带对流规则:\n{}\n\n\
            季风规则:\n{}\n\n\
            热带波动规则:\n{}\n\n\
            热带环流规则:\n{}\n\n\
            热带气旋分类规则:\n{}\n\n\
            热带降水规则:\n{}\n\n\
            热带气旋运动规则:\n{}\n\n\
            热带气旋灾害规则:\n{}\n\n\
            热带观测方法:\n{}\n\n\
            热带预报方法:\n{}",
            self.typhoon_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tropical_convection_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.monsoon_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tropical_waves_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tropical_circulation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tropical_cyclone_classification_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tropical_precipitation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tropical_cyclone_motion_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tropical_cyclone_disaster_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.observation_methods()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n"),
            self.forecast_methods()
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
    fn test_tropical_meteorology_rules() {
        let rules = TropicalMeteorologyRules::new();
        assert_eq!(rules.typhoon_rules().len(), 8);
        assert_eq!(rules.tropical_convection_rules().len(), 8);
        assert_eq!(rules.monsoon_rules().len(), 8);
        assert_eq!(rules.tropical_waves_rules().len(), 8);
        assert_eq!(rules.tropical_circulation_rules().len(), 8);
        assert_eq!(rules.tropical_cyclone_classification_rules().len(), 8);
        assert_eq!(rules.tropical_precipitation_rules().len(), 8);
        assert_eq!(rules.tropical_cyclone_motion_rules().len(), 8);
        assert_eq!(rules.tropical_cyclone_disaster_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_typhoon_rules() {
        let rules = TropicalMeteorologyRules::new();
        let laws = rules.typhoon_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("台风")));
    }

    #[test]
    fn test_monsoon_rules() {
        let rules = TropicalMeteorologyRules::new();
        assert_eq!(rules.monsoon_rules().len(), 8);
    }

    #[test]
    fn test_observation_methods() {
        let rules = TropicalMeteorologyRules::new();
        assert_eq!(rules.observation_methods().len(), 8);
    }
}