//! 天气学规则
//!
//! 天气学研究天气系统的形成、发展和移动规律，
//! 包括锋面分析、气旋识别、天气图分析和天气预报。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 天气学规则集合
pub struct SynopticMeteorologyRules {
    metadata: RuleMetadata,
}

impl SynopticMeteorologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("天气学规则", "天气系统分析和天气预报方法")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "气象".into(), "天气".into()]),
        }
    }

    /// 锋面分析规则
    pub fn frontal_analysis_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("冷锋识别定律", "锋面分析", "冷锋位置和强度识别方法"),
            ("暖锋识别定律", "锋面分析", "暖锋位置和特征识别"),
            ("准静止锋定律", "静止分析", "准静止锋形成和维持机制"),
            ("锢囚锋定律", "锢囚分析", "锢囚锋类型和形成过程"),
            ("锋面坡度定律", "坡度计算", "锋面坡度与气象要素关系"),
            ("锋生定律", "锋面生成", "锋面形成和加强机制"),
            ("锋消定律", "锋面消散", "锋面减弱和消散过程"),
            ("锋面天气定律", "天气分布", "锋面附近天气分布规律"),
        ]
    }

    /// 气旋分析规则
    pub fn cyclone_analysis_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("温带气旋定律", "气旋发展", "温带气旋形成和发展过程"),
            ("气旋生命周期定律", "生命史", "气旋发展各阶段特征"),
            ("气旋移动定律", "路径预测", "气旋移动方向和速度预测"),
            ("气旋加深定律", "气旋加强", "气旋加深机制和条件"),
            ("气旋填塞定律", "气旋减弱", "气旋减弱和消散过程"),
            ("爆发性气旋定律", "快速加深", "爆发性气旋加深条件"),
            ("气旋族定律", "气旋系列", "气旋族的形成和移动"),
            ("气旋路径分类定律", "路径类型", "气旋移动路径分类"),
        ]
    }

    /// 反气旋分析规则
    pub fn anticyclone_analysis_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("冷高压定律", "冷性反气旋", "冷高压的形成和移动"),
            ("暖高压定律", "暖性反气旋", "暖高压的形成和维持"),
            ("副热带高压定律", "副高活动", "副热带高压季节变化"),
            ("阻塞高压定律", "阻塞形势", "阻塞高压的形成和影响"),
            ("高压移动定律", "高压路径", "高压移动方向和速度"),
            ("高压发展定律", "高压加强", "高压发展和加强条件"),
            ("高压减弱定律", "高压减弱", "高压减弱和消散过程"),
            ("高压天气定律", "天气分布", "高压控制下天气特征"),
        ]
    }

    /// 天气图分析规则
    pub fn weather_map_analysis_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地面天气图定律", "地面分析", "地面天气图分析方法"),
            ("高空天气图定律", "高空分析", "高空天气图分析方法"),
            ("等压面图定律", "等压面分析", "等压面图分析技术"),
            ("剖面图定律", "剖面分析", "垂直剖面图分析方法"),
            ("流线图定律", "流线分析", "流线图绘制和分析"),
            ("天气系统识别定律", "系统识别", "天气系统识别和追踪"),
            ("天气形势分类定律", "形势分型", "天气形势分类方法"),
            ("天气图质量控制定律", "质量控制", "天气图分析质量控制"),
        ]
    }

    /// 气团分析规则
    pub fn air_mass_analysis_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("气团分类定律", "气团识别", "气团类型识别和分类"),
            ("极地气团定律", "冷气团", "极地气团性质和移动"),
            ("热带气团定律", "暖气团", "热带气团性质和演变"),
            ("大陆气团定律", "干燥气团", "大陆气团性质特征"),
            ("海洋气团定律", "湿润气团", "海洋气团性质特征"),
            ("气团变性定律", "性质变化", "气团下垫面变性过程"),
            ("气团源地定律", "源地分析", "气团源地特征和影响"),
            ("气团天气定律", "天气特征", "不同气团控制下天气"),
        ]
    }

    /// 急流分析规则
    pub fn jet_stream_analysis_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("极锋急流定律", "极地急流", "极锋急流位置和强度"),
            ("副热带急流定律", "副高急流", "副热带急流特征分析"),
            ("热带东风急流定律", "东风急流", "热带东风急流分析"),
            ("急流核定律", "急流中心", "急流核心位置和强度"),
            ("急流入口定律", "入口区", "急流入口区天气分布"),
            ("急流出口定律", "出口区", "急流出口区天气分布"),
            ("急流波动定律", "急流扰动", "急流波动和发展"),
            ("急流与天气定律", "天气影响", "急流对天气的影响"),
        ]
    }

    /// 涡度分析规则
    pub fn vorticity_analysis_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("涡度定律", "涡度计算", "涡度定义和计算方法"),
            ("相对涡度定律", "相对运动", "相对涡度分析和应用"),
            ("绝对涡度定律", "绝对涡度", "绝对涡度守恒原理"),
            ("位涡定律", "位涡分析", "位涡守恒和应用"),
            ("涡度平流定律", "涡度输送", "涡度平流与天气发展"),
            ("涡度变化定律", "涡度演变", "涡度变化方程分析"),
            ("散度定律", "辐散辐合", "散度与天气系统关系"),
            ("垂直运动定律", "垂直速度", "垂直运动计算方法"),
        ]
    }

    /// 天气过程分析规则
    pub fn weather_process_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("寒潮天气过程定律", "寒潮分析", "寒潮爆发和移动过程"),
            ("暴雨天气过程定律", "暴雨分析", "暴雨形成和发展过程"),
            ("强对流天气过程定律", "对流分析", "强对流天气发展过程"),
            ("大风天气过程定律", "大风分析", "大风天气形成过程"),
            ("降温天气过程定律", "降温分析", "降温天气过程分析"),
            ("降水天气过程定律", "降水分析", "降水天气过程分析"),
            ("雾天过程定律", "雾分析", "雾的形成和消散过程"),
            ("沙尘天气过程定律", "沙尘分析", "沙尘天气形成过程"),
        ]
    }

    /// 天气形势规则
    pub fn weather_pattern_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("纬向环流形势定律", "平直环流", "纬向环流天气特征"),
            ("经向环流形势定律", "波动环流", "经向环流天气特征"),
            ("阻塞形势定律", "阻塞分析", "阻塞高压天气形势"),
            ("切变线形势定律", "切变分析", "切变线天气形势"),
            ("低涡形势定律", "低涡分析", "低涡天气形势特征"),
            ("槽脊形势定律", "槽脊分析", "高空槽脊天气形势"),
            ("低压带形势定律", "低压分析", "低压带天气形势"),
            ("高压带形势定律", "高压分析", "高压带天气形势"),
        ]
    }

    /// 天气诊断规则
    pub fn weather_diagnosis_rules(&self) -> Vec<&'static str> {
        vec![
            "物理量诊断: 计算和分析各种物理量场",
            "能量诊断: 大气能量计算和分析",
            "水汽诊断: 水汽输送和收支分析",
            "动力诊断: 大气动力学诊断分析",
            "热力诊断: 大气热力学诊断分析",
            "稳定度诊断: 大气稳定度分析和判断",
            "散度诊断: 辐散辐合场分析",
            "涡度诊断: 涡度场分析和演变",
        ]
    }

    /// 天气预报规则
    pub fn forecast_methods(&self) -> Vec<&'static str> {
        vec![
            "外推预报: 天气系统外推预报方法",
            "相似预报: 历史相似天气形势预报",
            "统计预报: 统计方法天气预报",
            "概念模型预报: 天气概念模型应用",
            "物理推理预报: 物理过程推理预报",
            "经验预报: 预报员经验方法应用",
            "综合预报: 多种方法综合预报",
            "订正预报: 数值预报订正方法",
        ]
    }
}

impl Default for SynopticMeteorologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SynopticMeteorologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("synoptic_meteorology")
    }

    fn explain(&self) -> String {
        format!(
            "【天气学规则】\n\n\
            锋面分析规则:\n{}\n\n\
            气旋分析规则:\n{}\n\n\
            反气旋分析规则:\n{}\n\n\
            天气图分析规则:\n{}\n\n\
            气团分析规则:\n{}\n\n\
            急流分析规则:\n{}\n\n\
            涡度分析规则:\n{}\n\n\
            天气过程分析规则:\n{}\n\n\
            天气形势规则:\n{}\n\n\
            天气诊断规则:\n{}\n\n\
            天气预报规则:\n{}",
            self.frontal_analysis_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cyclone_analysis_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.anticyclone_analysis_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weather_map_analysis_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.air_mass_analysis_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.jet_stream_analysis_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.vorticity_analysis_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weather_process_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weather_pattern_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weather_diagnosis_rules()
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
    fn test_synoptic_meteorology_rules() {
        let rules = SynopticMeteorologyRules::new();
        assert_eq!(rules.frontal_analysis_rules().len(), 8);
        assert_eq!(rules.cyclone_analysis_rules().len(), 8);
        assert_eq!(rules.anticyclone_analysis_rules().len(), 8);
        assert_eq!(rules.weather_map_analysis_rules().len(), 8);
        assert_eq!(rules.air_mass_analysis_rules().len(), 8);
        assert_eq!(rules.jet_stream_analysis_rules().len(), 8);
        assert_eq!(rules.vorticity_analysis_rules().len(), 8);
        assert_eq!(rules.weather_process_rules().len(), 8);
        assert_eq!(rules.weather_pattern_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_frontal_rules() {
        let rules = SynopticMeteorologyRules::new();
        let laws = rules.frontal_analysis_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("锋")));
    }

    #[test]
    fn test_cyclone_rules() {
        let rules = SynopticMeteorologyRules::new();
        assert_eq!(rules.cyclone_analysis_rules().len(), 8);
    }

    #[test]
    fn test_diagnosis_methods() {
        let rules = SynopticMeteorologyRules::new();
        assert_eq!(rules.weather_diagnosis_rules().len(), 8);
    }
}
