//! 中尺度气象学规则
//!
//! 中尺度气象学研究中等尺度的大气现象和天气系统，
//! 包括强对流、雷暴、暴雨、龙卷风等灾害性天气。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 中尺度气象学规则集合
pub struct MesoscaleMeteorologyRules {
    metadata: RuleMetadata,
}

impl MesoscaleMeteorologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("中尺度气象学规则", "中尺度天气系统和对流天气")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "气象".into(), "中尺度".into()]),
        }
    }

    /// 强对流规则
    pub fn severe_convection_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("对流触发定律", "触发机制", "对流活动触发条件分析"),
            ("对流发展定律", "对流加强", "对流云发展加强过程"),
            ("对流组织定律", "对流系统", "对流系统组织形式"),
            ("对流维持定律", "对流持续", "对流系统维持机制"),
            ("对流消亡定律", "对流减弱", "对流系统减弱过程"),
            ("超级单体定律", "超级单体", "超级单体风暴结构"),
            ("多单体定律", "多单体风暴", "多单体风暴演变"),
            ("飑线定律", "飑线系统", "飑线形成和发展"),
        ]
    }

    /// 雷暴规则
    pub fn thunderstorm_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("雷暴形成定律", "雷暴条件", "雷暴形成的环境条件"),
            ("雷暴发展阶段定律", "发展过程", "雷暴三个发展阶段特征"),
            ("雷暴闪电定律", "放电过程", "雷电放电机制和分布"),
            ("雷暴降水定律", "对流降水", "雷暴降水强度分布"),
            ("雷暴大风定律", "下沉气流", "雷暴引起的下沉大风"),
            ("雷暴冰雹定律", "冰雹形成", "雷暴冰雹产生机制"),
            ("雷暴移动定律", "雷暴路径", "雷暴移动方向和速度"),
            ("雷暴灾害定律", "雷击灾害", "雷电灾害防护措施"),
        ]
    }

    /// 龙卷风规则
    pub fn tornado_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("龙卷风形成定律", "龙卷条件", "龙卷风形成的条件"),
            ("龙卷风结构定律", "漏斗云", "龙卷风漏斗云结构"),
            ("龙卷风强度定律", "EF分级", "龙卷风强度分级标准"),
            ("龙卷风路径定律", "路径特征", "龙卷风移动路径特点"),
            ("龙卷风生命周期定律", "持续时间", "龙卷风生命持续时间"),
            ("龙卷风多发定律", "龙卷走廊", "龙卷风多发区域特征"),
            ("龙卷风灾害定律", "破坏程度", "龙卷风灾害评估分级"),
            ("龙卷风预警定律", "预警方法", "龙卷风预警技术方法"),
        ]
    }

    /// 暴雨规则
    pub fn heavy_rain_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("暴雨形成定律", "暴雨条件", "暴雨形成的天气条件"),
            ("暴雨强度定律", "降水强度", "暴雨降水强度标准"),
            ("暴雨持续时间定律", "持续时间", "暴雨持续时间分析"),
            ("暴雨分布定律", "时空分布", "暴雨时空分布特征"),
            ("暴雨诱发定律", "暴雨触发", "暴雨触发机制分析"),
            ("暴雨增幅定律", "降水增幅", "暴雨降水增幅因素"),
            ("暴雨移动定律", "雨带移动", "暴雨雨带移动规律"),
            ("暴雨灾害定律", "洪涝灾害", "暴雨引发的洪涝灾害"),
        ]
    }

    /// 冰雹规则
    pub fn hail_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("冰雹形成定律", "雹胚生成", "冰雹胚胎形成条件"),
            ("冰雹增长定律", "雹块增长", "冰雹增长过程机制"),
            ("冰雹大小定律", "雹径分级", "冰雹直径大小分级"),
            ("冰雹分布定律", "落雹分布", "冰雹降落分布规律"),
            ("冰雹频率定律", "发生频率", "冰雹发生频率统计"),
            ("冰雹路径定律", "雹云路径", "冰雹云移动路径"),
            ("冰雹灾害定律", "雹灾评估", "冰雹灾害损失评估"),
            ("冰雹识别定律", "雹云识别", "冰雹云雷达识别方法"),
        ]
    }

    /// 下击暴流规则
    pub fn downburst_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("下击暴流形成定律", "下沉气流", "下击暴流形成机制"),
            ("微下击暴流定律", "微暴流", "微下击暴流特征分析"),
            ("宏下击暴流定律", "宏暴流", "宏下击暴流特征"),
            ("下击暴流强度定律", "风速强度", "下击暴流风速分布"),
            ("下击暴流持续时间定律", "持续时间", "下击暴流持续时间"),
            ("下击暴流灾害定律", "风害评估", "下击暴流灾害影响"),
            ("下击暴流探测定律", "雷达探测", "下击暴流雷达识别"),
            ("下击暴流预警定律", "预警技术", "下击暴流预警方法"),
        ]
    }

    /// 中尺度对流系统规则
    pub fn mesoscale_convective_system_rules(
        &self,
    ) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("MCS形成定律", "对流系统", "中尺度对流系统形成"),
            ("MCS结构定律", "系统结构", "MCS的结构特征分析"),
            ("MCS演变定律", "系统演变", "MCS的发展演变过程"),
            ("MCS移动定律", "系统移动", "MCS的移动规律"),
            ("MCS降水定律", "降水特征", "MCS降水分布特征"),
            ("MCS维持定律", "系统维持", "MCS维持机制分析"),
            ("MCS消亡定律", "系统消散", "MCS减弱消散过程"),
            ("MCS灾害定律", "灾害影响", "MCS灾害性天气影响"),
        ]
    }

    /// 中尺度环流规则
    pub fn mesoscale_circulation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("中尺度涡旋定律", "涡旋系统", "中尺度涡旋形成分析"),
            ("中尺度辐合线定律", "辐合线", "中尺度辐合线特征"),
            ("中尺度切变线定律", "切变系统", "中尺度切变线分析"),
            ("中尺度低压定律", "低压系统", "中尺度低压形成"),
            ("中尺度高压定律", "高压系统", "中尺度高压特征"),
            ("中尺度锋定律", "中尺度锋", "中尺度锋面分析"),
            ("中尺度急流定律", "中尺度急流", "中尺度急流特征"),
            ("中尺度对流涡旋定律", "MCV", "中尺度对流涡旋分析"),
        ]
    }

    /// 中尺度对流复合体规则
    pub fn mesoscale_convective_complex_rules(
        &self,
    ) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("MCC形成定律", "复合体形成", "MCC形成条件分析"),
            ("MCC结构定律", "复合体结构", "MCC的结构特征"),
            ("MCC演变定律", "复合体演变", "MCC演变过程分析"),
            ("MCC移动定律", "复合体移动", "MCC移动规律分析"),
            ("MCC降水定律", "复合体降水", "MCC降水特征分析"),
            ("MCC识别定律", "复合体识别", "MCC卫星识别标准"),
            ("MCC环境定律", "环境条件", "MCC环境条件分析"),
            ("MCC灾害定律", "复合体灾害", "MCC灾害天气影响"),
        ]
    }

    /// 中尺度观测方法
    pub fn observation_methods(&self) -> Vec<&'static str> {
        vec![
            "雷达观测: 雷达探测中尺度对流系统",
            "卫星观测: 卫星监测中尺度云系",
            "闪电定位: 闪电定位监测对流活动",
            "风廓线雷达: 风廓线观测风场变化",
            "微波辐射计: 微波辐射计观测大气",
            "自动站网: 自动气象站密集观测",
            "移动观测: 移动观测平台跟踪系统",
            "飞机探测: 飞机穿入对流系统探测",
        ]
    }

    /// 中尺度预报方法
    pub fn forecast_methods(&self) -> Vec<&'static str> {
        vec![
            "中尺度模式: 中尺度数值天气预报模式",
            "临近预报: 0-6小时对流临近预报",
            "短时预报: 6-12小时短时天气预报",
            "雷达外推: 雷达回波外推预报方法",
            "对流诊断: 对流活动诊断分析方法",
            "集合预报: 中尺度集合预报方法",
            "概率预报: 中尺度天气概率预报",
            "预警系统: 中尺度灾害天气预警",
        ]
    }
}

impl Default for MesoscaleMeteorologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MesoscaleMeteorologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("mesoscale_meteorology")
    }

    fn explain(&self) -> String {
        format!(
            "【中尺度气象学规则】\n\n\
            强对流规则:\n{}\n\n\
            雷暴规则:\n{}\n\n\
            龙卷风规则:\n{}\n\n\
            暴雨规则:\n{}\n\n\
            冰雹规则:\n{}\n\n\
            下击暴流规则:\n{}\n\n\
            中尺度对流系统规则:\n{}\n\n\
            中尺度环流规则:\n{}\n\n\
            中尺度对流复合体规则:\n{}\n\n\
            中尺度观测方法:\n{}\n\n\
            中尺度预报方法:\n{}",
            self.severe_convection_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.thunderstorm_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tornado_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.heavy_rain_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hail_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.downburst_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.mesoscale_convective_system_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.mesoscale_circulation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.mesoscale_convective_complex_rules()
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
    fn test_mesoscale_meteorology_rules() {
        let rules = MesoscaleMeteorologyRules::new();
        assert_eq!(rules.severe_convection_rules().len(), 8);
        assert_eq!(rules.thunderstorm_rules().len(), 8);
        assert_eq!(rules.tornado_rules().len(), 8);
        assert_eq!(rules.heavy_rain_rules().len(), 8);
        assert_eq!(rules.hail_rules().len(), 8);
        assert_eq!(rules.downburst_rules().len(), 8);
        assert_eq!(rules.mesoscale_convective_system_rules().len(), 8);
        assert_eq!(rules.mesoscale_circulation_rules().len(), 8);
        assert_eq!(rules.mesoscale_convective_complex_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_tornado_rules() {
        let rules = MesoscaleMeteorologyRules::new();
        let laws = rules.tornado_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("龙卷")));
    }

    #[test]
    fn test_heavy_rain_rules() {
        let rules = MesoscaleMeteorologyRules::new();
        assert_eq!(rules.heavy_rain_rules().len(), 8);
    }

    #[test]
    fn test_observation_methods() {
        let rules = MesoscaleMeteorologyRules::new();
        assert_eq!(rules.observation_methods().len(), 8);
    }
}
