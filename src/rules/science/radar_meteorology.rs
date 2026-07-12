//! 气象雷达学规则
//!
//! 气象雷达学研究雷达探测大气的方法和技术，
//! 包括雷达原理、信号处理和气象雷达产品应用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 气象雷达学规则集合
pub struct RadarMeteorologyRules {
    metadata: RuleMetadata,
}

impl RadarMeteorologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("气象雷达学规则", "雷达探测大气和天气系统技术")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "气象".into(), "雷达".into()]),
        }
    }

    /// 雷达原理规则
    pub fn radar_principle_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("雷达发射定律", "电磁波发射", "雷达发射电磁波原理"),
            ("雷达接收定律", "回波接收", "雷达接收回波信号"),
            ("雷达距离定律", "距离测量", "雷达测量目标距离方法"),
            ("雷达方位定律", "方位测量", "雷达测量目标方位"),
            ("雷达分辨定律", "分辨能力", "雷达分辨率分析"),
            ("雷达灵敏度定律", "探测灵敏度", "雷达最小探测能力"),
            ("雷达噪声定律", "信噪比", "雷达噪声和信噪比分析"),
            ("雷达衰减定律", "信号衰减", "雷达信号衰减规律"),
        ]
    }

    /// 反射率因子规则
    pub fn reflectivity_factor_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("反射率定义定律", "Z定义", "反射率因子定义和计算"),
            ("dBZ定律", "dBZ单位", "反射率dBZ单位换算"),
            ("反射率降水定律", "Z-R关系", "反射率与降水强度关系"),
            ("反射率分布定律", "回波强度", "反射率时空分布特征"),
            ("反射率阈值定律", "强度阈值", "降水反射率阈值标准"),
            ("反射率异常定律", "异常回波", "异常反射率回波分析"),
            ("反射率衰减定律", "衰减订正", "反射率衰减订正方法"),
            ("反射率质量控制定律", "质量控制", "反射率质量控制方法"),
        ]
    }

    /// 多普勒雷达规则
    pub fn doppler_radar_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("多普勒效应定律", "频率偏移", "多普勒频移原理分析"),
            ("速度测量定律", "径向速度", "多普勒雷达测速原理"),
            ("速度模糊定律", "模糊速度", "多普勒速度模糊问题"),
            ("退模糊定律", "速度退模糊", "多普勒速度退模糊方法"),
            ("速度谱定律", "速度谱宽", "多普勒速度谱宽分析"),
            ("速度产品定律", "速度产品", "多普勒速度产品类型"),
            ("速度应用定律", "应用分析", "多普勒速度产品应用"),
            ("双PRF定律", "双频率", "双PRF速度退模糊技术"),
        ]
    }

    /// 双偏振雷达规则
    pub fn dual_polarization_radar_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("偏振波定律", "偏振模式", "水平和垂直偏振波发射"),
            ("差分反射率定律", "ZDR", "ZDR参数和粒子形状"),
            ("差分相位定律", "ΦDP", "差分相位传播测量"),
            ("比差分相位定律", "KDP", "KDP参数和降水强度"),
            ("相关系数定律", "CC", "共极相关系数分析"),
            ("线性退极振比定律", "LDR", "LDR退极振比分析"),
            ("偏振产品定律", "偏振产品", "双偏振雷达产品类型"),
            ("偏振应用定律", "应用分析", "双偏振雷达产品应用"),
        ]
    }

    /// 雷达定量降水规则
    pub fn radar_qpe_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Z-R关系定律", "降水反演", "反射率降水关系反演"),
            ("动态Z-R定律", "动态关系", "动态Z-R关系调整"),
            ("雨量校准定律", "雨量订正", "雷达雨量计校准方法"),
            ("降水率定律", "降水计算", "雷达降水率计算方法"),
            ("降水累积定律", "累积降水", "雷达累积降水计算"),
            ("降水订正定律", "偏差订正", "雷达降水偏差订正"),
            ("降水分布定律", "降水场", "雷达降水分布分析"),
            ("降水检验定律", "检验评估", "雷达降水检验方法"),
        ]
    }

    /// 雷达回波识别规则
    pub fn echo_identification_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("降水回波定律", "降水识别", "降水回波识别方法"),
            ("对流回波定律", "对流识别", "对流回波识别特征"),
            ("层状回波定律", "层状识别", "层状云回波特征分析"),
            ("冰雹回波定律", "冰雹识别", "冰雹回波识别方法"),
            ("大风回波定律", "大风识别", "大风回波特征识别"),
            ("晴空回波定律", "晴空回波", "晴空回波识别分析"),
            ("杂波识别定律", "杂波剔除", "雷达杂波识别剔除"),
            ("鸟类回波定律", "生物回波", "鸟类等生物回波识别"),
        ]
    }

    /// 雷达质量控制规则
    pub fn radar_qc_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地物杂波定律", "地物抑制", "地物杂波抑制方法"),
            ("电磁干扰定律", "干扰剔除", "电磁干扰杂波剔除"),
            ("噪声处理定律", "噪声抑制", "雷达噪声抑制方法"),
            ("衰减订正定律", "衰减校正", "雷达衰减订正技术"),
            ("阻挡订正定律", "阻挡校正", "雷达阻挡订正方法"),
            ("速度订正定律", "速度校正", "多普勒速度订正"),
            ("偏振订正定律", "偏振校正", "双偏振参数订正"),
            ("综合质量定律", "质量控制", "雷达数据质量控制"),
        ]
    }

    /// 雷达组网规则
    pub fn radar_network_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("雷达拼图定律", "拼图技术", "多雷达拼图技术方法"),
            ("雷达组合定律", "组合产品", "多雷达组合产品分析"),
            ("雷达覆盖定律", "覆盖范围", "雷达网覆盖范围分析"),
            ("雷达重叠定律", "重叠区域", "雷达重叠区域处理"),
            ("雷达同步定律", "同步观测", "多雷达同步观测方法"),
            ("雷达融合定律", "数据融合", "多雷达数据融合技术"),
            ("雷达补盲定律", "补盲雷达", "雷达网补盲部署"),
            ("雷达协调定律", "协调观测", "雷达网协调观测策略"),
        ]
    }

    /// 雷达产品规则
    pub fn radar_products_rules(&self) -> Vec<&'static str> {
        vec![
            "基本反射率产品: 雷达基本反射率产品类型",
            "组合反射率产品: 组合反射率产品分析",
            "速度产品: 多普勒速度产品类型",
            "谱宽产品: 速度谱宽产品分析",
            "降水产品: 雷达定量降水产品",
            "回波顶产品: 回波顶高产品分析",
            "垂直累积液态水: VIL产品分析",
            "风暴追踪产品: 风暴追踪和识别产品",
        ]
    }

    /// 雷达应用领域
    pub fn application_areas(&self) -> Vec<&'static str> {
        vec![
            "天气监测: 雷达监测天气系统发展",
            "降水估计: 雷达定量降水估计应用",
            "灾害预警: 雷达灾害天气预警服务",
            "对流分析: 雷达对流天气分析识别",
            "风场分析: 雷达风场分析应用",
            "数值模式: 雷达资料数值模式应用",
            "临近预报: 雷达临近预报技术",
            "研究应用: 雷达气象科研应用",
        ]
    }
}

impl Default for RadarMeteorologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RadarMeteorologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("radar_meteorology")
    }

    fn explain(&self) -> String {
        format!(
            "【气象雷达学规则】\n\n\
            雷达原理规则:\n{}\n\n\
            反射率因子规则:\n{}\n\n\
            多普勒雷达规则:\n{}\n\n\
            双偏振雷达规则:\n{}\n\n\
            雷达定量降水规则:\n{}\n\n\
            雷达回波识别规则:\n{}\n\n\
            雷达质量控制规则:\n{}\n\n\
            雷达组网规则:\n{}\n\n\
            雷达产品规则:\n{}\n\n\
            雷达应用领域:\n{}",
            self.radar_principle_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.reflectivity_factor_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.doppler_radar_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dual_polarization_radar_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.radar_qpe_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.echo_identification_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.radar_qc_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.radar_network_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.radar_products_rules()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            self.application_areas()
                .iter()
                .map(|a| format!("  • {}", a))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radar_meteorology_rules() {
        let rules = RadarMeteorologyRules::new();
        assert_eq!(rules.radar_principle_rules().len(), 8);
        assert_eq!(rules.reflectivity_factor_rules().len(), 8);
        assert_eq!(rules.doppler_radar_rules().len(), 8);
        assert_eq!(rules.dual_polarization_radar_rules().len(), 8);
        assert_eq!(rules.radar_qpe_rules().len(), 8);
        assert_eq!(rules.echo_identification_rules().len(), 8);
        assert_eq!(rules.radar_qc_rules().len(), 8);
        assert_eq!(rules.radar_network_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_doppler_rules() {
        let rules = RadarMeteorologyRules::new();
        let laws = rules.doppler_radar_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("多普勒")));
    }

    #[test]
    fn test_polarization_rules() {
        let rules = RadarMeteorologyRules::new();
        assert_eq!(rules.dual_polarization_radar_rules().len(), 8);
    }

    #[test]
    fn test_products_rules() {
        let rules = RadarMeteorologyRules::new();
        assert_eq!(rules.radar_products_rules().len(), 8);
    }
}