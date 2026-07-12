//! 遥感地理详细规则
//!
//! 遥感地理学研究遥感技术在地理中的应用，
//! 包括遥感原理、遥感分类、遥感解译和遥感应用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 遥感地理详细规则集合
pub struct RemoteSensingDetailedRules {
    metadata: RuleMetadata,
}

impl RemoteSensingDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("遥感地理详细规则", "遥感技术地理应用和解译规律")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "遥感".into()]),
        }
    }

    /// 遥感原理规则
    pub fn remote_sensing_principles(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("电磁波定律", "电磁辐射", "遥感利用电磁波探测"),
            ("光谱定律", "光谱特征", "地物光谱反射特征"),
            ("辐射定律", "辐射传输", "电磁辐射传输规律"),
            ("分辨率定律", "空间分辨率", "遥感影像空间分辨率"),
            ("波段定律", "波段选择", "遥感波段选择原则"),
            ("传感器定律", "传感器类型", "遥感传感器类型特征"),
            ("平台定律", "遥感平台", "遥感平台类型轨道"),
            ("几何定律", "几何校正", "遥感影像几何校正"),
        ]
    }

    /// 遥感分类规则
    pub fn classification_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("监督分类定律", "训练样本", "监督分类训练样本"),
            ("非监督分类定律", "聚类分析", "非监督分类聚类方法"),
            ("分类精度定律", "精度评价", "分类精度评价指标"),
            ("混合像元定律", "亚像元", "混合像元分解方法"),
            ("面向对象分类定律", "对象分割", "面向对象分类方法"),
            ("深度学习分类定律", "神经网络", "深度学习分类方法"),
            ("分类后处理定律", "结果修正", "分类后处理方法"),
            ("分类验证定律", "验证方法", "分类结果验证方法"),
        ]
    }

    /// 遥感解译规则
    pub fn interpretation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("目视解译定律", "人工判读", "目视解译判读方法"),
            ("特征提取定律", "特征识别", "遥感特征提取方法"),
            ("纹理分析定律", "纹理特征", "纹理特征分析方法"),
            ("形状识别定律", "形状特征", "形状特征识别方法"),
            ("光谱解译定律", "光谱判读", "光谱特征解译方法"),
            ("变化检测定律", "多时相", "多时相变化检测"),
            ("异常探测定律", "异常识别", "异常信息探测方法"),
            ("三维解译定律", "三维重建", "三维地形解译方法"),
        ]
    }

    /// 遥感数据规则
    pub fn data_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("数据获取定律", "数据采集", "遥感数据获取方式"),
            ("数据处理定律", "预处理", "遥感数据预处理方法"),
            ("数据融合定律", "多源融合", "多源数据融合方法"),
            ("数据压缩定律", "压缩存储", "遥感数据压缩方法"),
            ("数据格式定律", "标准格式", "遥感数据标准格式"),
            ("数据管理定律", "数据库", "遥感数据管理系统"),
            ("数据共享定律", "共享机制", "遥感数据共享机制"),
            ("数据质量定律", "质量控制", "遥感数据质量控制"),
        ]
    }

    /// 遥感应用规则
    pub fn application_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("土地遥感定律", "土地利用", "土地利用遥感监测"),
            ("植被遥感定律", "植被监测", "植被覆盖遥感监测"),
            ("水体遥感定律", "水体监测", "水体分布遥感监测"),
            ("城市遥感定律", "城市监测", "城市建设遥感监测"),
            ("农业遥感定律", "农业监测", "农业生产遥感监测"),
            ("灾害遥感定律", "灾害监测", "自然灾害遥感监测"),
            ("环境遥感定律", "环境监测", "环境变化遥感监测"),
            ("地质遥感定律", "地质解译", "地质构造遥感解译"),
        ]
    }

    /// 遥感监测规则
    pub fn monitoring_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("实时监测定律", "实时监测", "遥感实时监测系统"),
            ("周期监测定律", "周期观测", "遥感周期观测规律"),
            ("动态监测定律", "动态变化", "遥感动态变化监测"),
            ("大范围监测定律", "区域监测", "大范围区域遥感监测"),
            ("精细监测定律", "精细观测", "高分辨率精细监测"),
            ("多尺度监测定律", "多尺度", "多尺度遥感监测"),
            ("多波段监测定律", "多光谱", "多波段光谱监测"),
            ("全天候监测定律", "全天候", "全天候遥感监测"),
        ]
    }

    /// 遥感分析规则
    pub fn analysis_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("定量分析定律", "定量反演", "遥感定量反演方法"),
            ("统计分析定律", "统计分析", "遥感统计分析方法"),
            ("空间分析定律", "空间分析", "遥感空间分析方法"),
            ("时序分析定律", "时间序列", "时间序列遥感分析"),
            ("模型分析定律", "模型反演", "遥感模型反演方法"),
            ("误差分析定律", "误差评估", "遥感误差分析方法"),
            ("精度分析定律", "精度评估", "遥感精度评估方法"),
            ("敏感性分析定律", "敏感性", "遥感敏感性分析"),
        ]
    }

    /// 遥感产品规则
    pub fn product_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("影像产品定律", "影像类型", "遥感影像产品类型"),
            ("专题产品定律", "专题图", "遥感专题图产品"),
            ("数据产品定律", "数据产品", "遥感数据产品类型"),
            ("分析产品定律", "分析结果", "遥感分析结果产品"),
            ("监测产品定律", "监测报告", "遥感监测报告产品"),
            ("预警产品定律", "预警信息", "遥感预警信息产品"),
            ("评估产品定律", "评估结果", "遥感评估结果产品"),
            ("服务产品定律", "遥感服务", "遥感服务产品类型"),
        ]
    }

    /// 主要遥感类型
    pub fn major_rs_types(&self) -> Vec<&'static str> {
        vec![
            "光学遥感: 可见光红外光学遥感",
            "热红外遥感: 热红外波段遥感",
            "微波遥感: 微波雷达遥感",
            "激光雷达遥感: LiDAR激光雷达",
            "高光谱遥感: 高光谱分辨率遥感",
            "多光谱遥感: 多波段光谱遥感",
            "卫星遥感: 卫星平台遥感",
            "航空遥感: 航空飞机遥感",
            "无人机遥感: 无人机平台遥感",
            "地面遥感: 地面平台遥感",
        ]
    }

    /// 遥感技术发展
    pub fn technology_development(&self) -> Vec<&'static str> {
        vec![
            "高分辨率: 空间分辨率不断提高",
            "高光谱: 光谱分辨率不断提高",
            "多角度: 多角度观测技术发展",
            "实时处理: 实时处理能力增强",
            "智能分析: 人工智能遥感应用",
            "大数据: 遥感大数据处理技术",
            "云服务: 遥感云计算服务",
            "集成应用: 多技术集成应用",
        ]
    }
}

impl Default for RemoteSensingDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RemoteSensingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("remote_sensing_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【遥感地理详细规则】\n\n\
            遥感原理规则:\n{}\n\n\
            遥感分类规则:\n{}\n\n\
            遥感解译规则:\n{}\n\n\
            遥感数据规则:\n{}\n\n\
            遥感应用规则:\n{}\n\n\
            遥感监测规则:\n{}\n\n\
            遥感分析规则:\n{}\n\n\
            遥感产品规则:\n{}\n\n\
            主要遥感类型:\n{}\n\n\
            遥感技术发展:\n{}",
            self.remote_sensing_principles()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.classification_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.interpretation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.data_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.application_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.monitoring_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.analysis_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.product_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.major_rs_types()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.technology_development()
                .iter()
                .map(|d| format!("  • {}", d))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_sensing_detailed_rules() {
        let rules = RemoteSensingDetailedRules::new();
        assert_eq!(rules.remote_sensing_principles().len(), 8);
        assert_eq!(rules.classification_rules().len(), 8);
        assert_eq!(rules.interpretation_rules().len(), 8);
        assert_eq!(rules.data_rules().len(), 8);
        assert_eq!(rules.application_rules().len(), 8);
        assert_eq!(rules.monitoring_rules().len(), 8);
        assert_eq!(rules.analysis_rules().len(), 8);
        assert_eq!(rules.product_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_rs_types() {
        let rules = RemoteSensingDetailedRules::new();
        assert_eq!(rules.major_rs_types().len(), 10);
    }

    #[test]
    fn test_principles() {
        let rules = RemoteSensingDetailedRules::new();
        let laws = rules.remote_sensing_principles();
        assert!(laws.iter().any(|(n, _, _)| n.contains("电磁")));
    }

    #[test]
    fn test_development() {
        let rules = RemoteSensingDetailedRules::new();
        assert_eq!(rules.technology_development().len(), 8);
    }
}