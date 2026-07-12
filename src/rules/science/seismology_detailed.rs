//! 地震学详细规则
//!
//! 地震学研究地震的发生机制、地震波传播和地震灾害，
//! 包括地震监测、地震预警、地震工程和地震预测。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 地震学详细规则集合
pub struct SeismologyDetailedRules {
    metadata: RuleMetadata,
}

impl SeismologyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("地震学详细规则", "地震学基本定律和地震系统")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地震".into(), "地球".into()]),
        }
    }

    /// 地震波类型规则
    pub fn seismic_wave_types_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("纵波P波定律", "压缩波", "介质压缩振动传播最快的波"),
            ("横波S波定律", "剪切波", "介质剪切振动传播较慢的波"),
            ("面波定律", "表面波", "沿地球表面传播的波"),
            ("瑞利波定律", "滚动波", "椭圆轨迹运动的表面波"),
            ("勒夫波定律", "水平波", "水平横向运动的表面波"),
            ("体波定律", "内部波", "在地球内部传播的地震波"),
            ("波速定律", "速度差异", "P波快于S波约1.7倍"),
            ("波衰减定律", "能量损耗", "地震波传播中的能量衰减"),
        ]
    }

    /// 地震震级规则
    pub fn earthquake_magnitude_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("里氏震级定律", "原始震级", "地震震级的原始定义方法"),
            ("面波震级定律", "Ms震级", "用面波振幅测定的震级"),
            ("体波震级定律", "Mb震级", "用体波振幅测定的震级"),
            ("矩震级定律", "Mw震级", "用地震矩测定的震级"),
            ("震级范围定律", "震级尺度", "震级从负值到9以上"),
            ("震级能量定律", "能量关系", "震级每增1级能量增约32倍"),
            ("震级测定定律", "测定方法", "震级测定的技术方法"),
            ("震级误差定律", "测量误差", "震级测定的误差范围"),
        ]
    }

    /// 地震烈度规则
    pub fn earthquake_intensity_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("烈度定义定律", "影响程度", "地震对地面影响的程度"),
            ("烈度表定律", "等级标准", "烈度划分的标准表格"),
            ("烈度分布定律", "等震线", "烈度在地面的分布图"),
            ("烈度衰减定律", "距离衰减", "烈度随距离增加衰减"),
            ("烈度因素定律", "影响因素", "烈度受多种因素影响"),
            ("烈度评定定律", "评定方法", "烈度评定的调查方法"),
            ("烈度应用定律", "灾害评估", "烈度用于灾害评估"),
            ("烈度历史定律", "历史烈度", "历史地震烈度的确定"),
        ]
    }

    /// 地震成因规则
    pub fn earthquake_origin_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("构造地震定律", "板块运动", "板块运动引起的地震"),
            ("火山地震定律", "火山活动", "火山活动引起的地震"),
            ("诱发地震定律", "人为诱发", "人类活动诱发的地震"),
            ("震源深度定律", "深度分类", "浅源、中源、深源地震"),
            ("震源机制定律", "破裂方式", "地震断层的破裂方式"),
            ("地震序列定律", "序列类型", "主震型、震群型地震"),
            ("前震定律", "前兆地震", "主震前的前兆地震"),
            ("余震定律", "后续地震", "主震后的余震序列"),
        ]
    }

    /// 地震监测规则
    pub fn earthquake_monitoring_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地震台网定律", "监测网络", "地震监测台站网络"),
            ("地震仪定律", "记录仪器", "地震波的记录仪器"),
            ("地震定位定律", "震源定位", "确定震源位置的方法"),
            ("地震速报定律", "快速报告", "地震后的快速报告"),
            ("地震预警定律", "预警系统", "地震预警技术系统"),
            ("地震预报定律", "预报研究", "地震预报的研究方法"),
            ("地震监测定律", "长期监测", "地震活动的长期监测"),
            ("地震数据定律", "数据管理", "地震数据的管理分析"),
        ]
    }

    /// 地震灾害规则
    pub fn earthquake_disaster_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("直接灾害定律", "地震直接", "地震直接造成的破坏"),
            ("次生灾害定律", "间接灾害", "地震诱发的次生灾害"),
            ("地震液化定律", "砂土液化", "饱和砂土液化失稳"),
            ("地震滑坡定律", "边坡失稳", "地震引起的滑坡崩塌"),
            ("地震海啸定律", "海啸灾害", "海底地震引发海啸"),
            ("地震火灾定律", "火灾灾害", "地震引起的火灾"),
            ("地震伤亡定律", "人员伤亡", "地震造成的人员伤亡"),
            ("地震损失定律", "经济损失", "地震造成的经济损失"),
        ]
    }

    /// 地震工程规则
    pub fn earthquake_engineering_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("抗震设计定律", "结构抗震", "建筑结构的抗震设计"),
            ("地震荷载定律", "地震作用", "地震对结构的荷载作用"),
            ("抗震等级定律", "抗震分类", "建筑抗震等级划分"),
            ("场地影响定律", "场地效应", "场地对地震的影响"),
            ("结构响应定律", "动力响应", "结构对地震的动力响应"),
            ("抗震措施定律", "抗震构造", "建筑抗震构造措施"),
            ("减震技术定律", "减震方法", "结构减震的技术方法"),
            ("隔震技术定律", "隔震方法", "基础隔震的技术方法"),
        ]
    }

    /// 地震研究规则
    pub fn earthquake_research_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地震统计定律", "统计分析", "地震活动的统计分析"),
            ("地震模型定律", "数学模型", "地震活动的数学模型"),
            ("地震实验定律", "实验研究", "地震的实验研究方法"),
            ("地震模拟定律", "数值模拟", "地震的数值模拟技术"),
            ("地震反演定律", "震源反演", "震源参数的反演方法"),
            ("地震成像定律", "地球成像", "利用地震波成像地球"),
            ("地震预测定律", "预测研究", "地震预测的研究进展"),
            ("地震风险定律", "风险评估", "地震风险的评估方法"),
        ]
    }
}

impl Default for SeismologyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SeismologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("seismology_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_titled_sections(
            "地震学详细规则",
            &[
                ("地震波类型", &self.seismic_wave_types_rules()),
                ("地震震级", &self.earthquake_magnitude_rules()),
                ("地震烈度", &self.earthquake_intensity_rules()),
                ("地震成因", &self.earthquake_origin_rules()),
                ("地震监测", &self.earthquake_monitoring_rules()),
                ("地震灾害", &self.earthquake_disaster_rules()),
                ("地震工程", &self.earthquake_engineering_rules()),
                ("地震研究", &self.earthquake_research_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seismology_detailed_rules() {
        let rules = SeismologyDetailedRules::new();
        assert_eq!(rules.metadata().name, "地震学详细规则");
        assert_eq!(rules.seismic_wave_types_rules().len(), 8);
        assert_eq!(rules.earthquake_magnitude_rules().len(), 8);
        assert_eq!(rules.earthquake_intensity_rules().len(), 8);
        assert_eq!(rules.earthquake_origin_rules().len(), 8);
        assert_eq!(rules.earthquake_monitoring_rules().len(), 8);
        assert_eq!(rules.earthquake_disaster_rules().len(), 8);
        assert_eq!(rules.earthquake_engineering_rules().len(), 8);
        assert_eq!(rules.earthquake_research_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_seismology_category() {
        let rules = SeismologyDetailedRules::new();
        assert_eq!(rules.category().domain, "science");
        assert_eq!(rules.category().name, "seismology_detailed");
    }

    #[test]
    fn test_seismology_validate() {
        let rules = SeismologyDetailedRules::new();
        let ctx = crate::rules::core::ValidateContext::default();
        assert!(rules.validate(&ctx).is_ok());
    }
}