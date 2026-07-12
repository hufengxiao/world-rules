//! 气候学详细规则
//!
//! 气候学研究地球气候的形成、分布和变化规律，
//! 包括气候分类、气候变化、气候要素和气候系统。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 气候学详细规则集合
pub struct ClimatologyDetailedRules {
    metadata: RuleMetadata,
}

impl ClimatologyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("气候学详细规则", "气候学详细定律和气候系统")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "气候".into()]),
        }
    }

    /// 气候分类规则
    pub fn climate_classification_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("柯本气候分类", "温度降水", "根据温度降水划分气候类型"),
            ("桑斯威特分类", "蒸散发", "根据蒸散发划分气候类型"),
            ("气候成因分类", "环流成因", "根据大气环流成因分类"),
            ("纬度气候带", "纬度分布", "按纬度划分热带温带寒带"),
            ("大陆性气候", "海洋影响", "远离海洋的大陆性气候特征"),
            ("海洋性气候", "海洋调节", "受海洋影响的气候特征"),
            ("山地气候", "垂直分带", "山地气候垂直分布规律"),
            ("季风气候", "季风环流", "季风影响下的气候特征"),
        ]
    }

    /// 气候要素规则
    pub fn climate_elements_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("温度定律", "热量分布", "温度随纬度高度变化规律"),
            ("降水定律", "降水分布", "降水时空分布规律"),
            ("湿度定律", "水汽含量", "大气湿度变化规律"),
            ("风定律", "气流运动", "风向风速分布规律"),
            ("云定律", "云量分布", "云量云型分布规律"),
            ("辐射定律", "太阳辐射", "太阳辐射分布和变化"),
            ("蒸发定律", "蒸发强度", "蒸发速率影响因素"),
            ("日照定律", "日照时长", "日照时数分布规律"),
        ]
    }

    /// 气候变化规则
    pub fn climate_change_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("全球变暖定律", "温度上升", "全球平均温度持续上升"),
            ("温室效应定律", "CO₂增温", "温室气体导致全球变暖"),
            ("冰期循环定律", "冰期间冰期", "地球冰期和间冰期循环"),
            ("气候突变定律", "快速变化", "气候系统突变事件"),
            ("气候振荡定律", "周期变化", "气候周期性振荡现象"),
            ("气候适应定律", "生物适应", "生物适应气候变化"),
            ("气候影响定律", "环境影响", "气候变化对环境影响"),
            ("气候反馈定律", "反馈机制", "气候系统反馈循环"),
        ]
    }

    /// 气候系统规则
    pub fn climate_system_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("大气环流定律", "环流模式", "全球大气环流三圈模式"),
            ("季风系统定律", "季风环流", "季风形成和变化规律"),
            ("厄尔尼诺定律", "ENSO循环", "厄尔尼诺南方涛动循环"),
            ("拉尼娜定律", "反厄尔尼诺", "拉尼娜现象和影响"),
            ("Walker环流定律", "纬向环流", "Walker环流影响气候"),
            ("气候敏感定律", "系统响应", "气候系统对强迫的响应"),
            ("气候阈值定律", "临界点", "气候系统临界阈值"),
            ("气候惯性定律", "滞后响应", "气候系统响应滞后"),
        ]
    }

    /// 气候区域规则
    pub fn climate_regions_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热带气候定律", "热带特征", "热带高温多雨气候"),
            ("温带气候定律", "温带特征", "温带四季分明气候"),
            ("寒带气候定律", "寒带特征", "寒带寒冷干燥气候"),
            ("干旱气候定律", "干旱特征", "干旱区降水稀少气候"),
            ("湿润气候定律", "湿润特征", "湿润区降水充沛气候"),
            ("高原气候定律", "高原特征", "高原低温低氧气候"),
            ("沿海气候定律", "沿海特征", "沿海温和湿润气候"),
            ("内陆气候定律", "内陆特征", "内陆干旱温差大气候"),
        ]
    }

    /// 气候极端事件规则
    pub fn extreme_weather_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热浪定律", "极端高温", "热浪形成和影响规律"),
            ("寒潮定律", "极端低温", "寒潮形成和影响规律"),
            ("台风定律", "热带气旋", "台风形成路径和影响"),
            ("暴雨定律", "强降水", "暴雨形成和分布规律"),
            ("干旱定律", "降水缺乏", "干旱形成和持续规律"),
            ("洪涝定律", "洪水泛滥", "洪涝形成和影响规律"),
            ("沙尘暴定律", "风沙天气", "沙尘暴形成和影响"),
            ("龙卷风定律", "强对流", "龙卷风形成和路径"),
        ]
    }

    /// 气候预测规则
    pub fn climate_prediction_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("气候模型定律", "数值模拟", "气候数值模型预测"),
            ("气候预报定律", "短期预报", "短期气候预测方法"),
            ("气候预估定律", "长期预估", "长期气候变化预估"),
            ("气候情景定律", "情景分析", "未来气候情景分析"),
            ("气候概率定律", "概率预测", "气候概率预测方法"),
            ("气候验证定律", "预测检验", "气候预测验证方法"),
            ("气候订正定律", "偏差订正", "气候预测偏差订正"),
            ("气候集成定律", "多模型集成", "多模型集成预测"),
        ]
    }

    /// 气候影响规则
    pub fn climate_impact_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("气候农业定律", "农业影响", "气候对农业的影响"),
            ("气候植被定律", "植被分布", "气候对植被分布影响"),
            ("气候水文定律", "水文影响", "气候对水文循环影响"),
            ("气候生态定律", "生态系统", "气候对生态系统影响"),
            ("气候健康定律", "人类健康", "气候对健康的影响"),
            ("气候城市定律", "城市气候", "城市气候特征和影响"),
            ("气候灾害定律", "气候灾害", "气候极端事件灾害"),
            ("气候经济定律", "经济影响", "气候变化经济影响"),
        ]
    }

    /// 气候观测方法
    pub fn observation_methods(&self) -> Vec<&'static str> {
        vec![
            "气象站观测: 地面气象要素定点定时观测",
            "高空探测: 探空球雷达探测高空大气",
            "卫星遥感: 卫星遥感观测全球气候要素",
            "雷达探测: 雷达探测降水云系分布",
            "海洋观测: 海洋气象要素观测网络",
            "自动观测: 自动气象站连续观测",
            "气候记录: 长期气候资料记录整理",
            "气候重建: 利用代用资料重建古气候",
        ]
    }

    /// 气候研究主题
    pub fn research_topics(&self) -> Vec<&'static str> {
        vec![
            "气候变化机理: 研究气候变化的原因和过程",
            "气候模拟预测: 发展气候数值模型预测方法",
            "气候影响评估: 评估气候变化的影响和风险",
            "气候适应对策: 制定气候变化适应策略",
            "气候减缓措施: 减少温室气体排放减缓变暖",
            "气候极端事件: 研究气候极端事件规律",
            "气候系统反馈: 研究气候系统反馈机制",
            "气候不确定性: 分析气候预测不确定性",
        ]
    }
}

impl Default for ClimatologyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ClimatologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("climatology_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【气候学详细规则】\n\n\
            气候分类规则:\n{}\n\n\
            气候要素规则:\n{}\n\n\
            气候变化规则:\n{}\n\n\
            气候系统规则:\n{}\n\n\
            气候区域规则:\n{}\n\n\
            气候极端事件规则:\n{}\n\n\
            气候预测规则:\n{}\n\n\
            气候影响规则:\n{}\n\n\
            气候观测方法:\n{}\n\n\
            气候研究主题:\n{}",
            self.climate_classification_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.climate_elements_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.climate_change_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.climate_system_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.climate_regions_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.extreme_weather_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.climate_prediction_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.climate_impact_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.observation_methods()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n"),
            self.research_topics()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climatology_detailed_rules() {
        let rules = ClimatologyDetailedRules::new();
        assert_eq!(rules.climate_classification_rules().len(), 8);
        assert_eq!(rules.climate_elements_rules().len(), 8);
        assert_eq!(rules.climate_change_rules().len(), 8);
        assert_eq!(rules.climate_system_rules().len(), 8);
        assert_eq!(rules.climate_regions_rules().len(), 8);
        assert_eq!(rules.extreme_weather_rules().len(), 8);
        assert_eq!(rules.climate_prediction_rules().len(), 8);
        assert_eq!(rules.climate_impact_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_classification_rules() {
        let rules = ClimatologyDetailedRules::new();
        let laws = rules.climate_classification_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("柯本")));
    }

    #[test]
    fn test_extreme_weather() {
        let rules = ClimatologyDetailedRules::new();
        assert_eq!(rules.extreme_weather_rules().len(), 8);
    }

    #[test]
    fn test_observation_methods() {
        let rules = ClimatologyDetailedRules::new();
        assert_eq!(rules.observation_methods().len(), 8);
    }
}