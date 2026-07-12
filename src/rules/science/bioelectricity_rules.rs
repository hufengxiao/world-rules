//! 生物电学规则
//!
//! 生物体电学现象和原理，包括神经电活动、心脏电活动、
//! 生物电信号、电感受等核心概念。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 生物电学规则集合
pub struct BioelectricityRules {
    metadata: RuleMetadata,
}

impl BioelectricityRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("生物电学规则", "生物体电学现象和原理")
                .with_origin("生物电学")
                .with_tags(vec!["科学".into(), "生命科学".into(), "电学".into()]),
        }
    }

    /// 神经电活动定律
    pub fn neural_electrical(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("静息电位定律", "-70mV", "神经元静息时膜电位"),
            ("动作电位定律", "去极化", "神经兴奋时膜电位变化"),
            ("离子通道定律", "离子流动", "离子通道控制离子流"),
            ("钠钾泵定律", "电位维持", "钠钾泵维持电位"),
            ("局部电位定律", "局部变化", "局部膜电位变化"),
            ("传导定律", "电传导", "动作电位传导"),
            ("突触电位定律", "突触传递", "突触电位传递"),
        ]
    }

    /// 心脏电活动定律
    pub fn cardiac_electrical(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("起搏点定律", "窦房结", "窦房结是心脏起搏点"),
            ("传导通路定律", "传导系统", "心脏传导系统"),
            ("ECG定律", "心电图", "心电图记录心电"),
            ("P波定律", "心房除极", "P波代表心房活动"),
            ("QRS波定律", "心室除极", "QRS波代表心室活动"),
            ("T波定律", "心室复极", "T波代表心室复极"),
            ("心律定律", "节律规律", "心脏节律规律"),
        ]
    }

    /// 肌肉电活动定律
    pub fn muscle_electrical(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("肌膜电位定律", "肌肉电位", "肌纤维膜电位"),
            ("EMG定律", "肌电图", "肌电图记录肌电"),
            ("运动单位定律", "功能单元", "运动单位放电"),
            ("肌肉兴奋定律", "兴奋收缩", "肌肉兴奋收缩耦联"),
            ("疲劳定律", "疲劳特征", "疲劳肌电特征"),
            ("募集定律", "单位募集", "运动单位募集"),
            ("发放频率定律", "频率调节", "发放频率调节力量"),
        ]
    }

    /// 生物电信号定律
    pub fn bioelectric_signals(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("EEG定律", "脑电图", "脑电图记录脑电"),
            ("脑波定律", "脑波类型", "不同频率脑波"),
            ("诱发电位定律", "刺激诱发", "刺激诱发电位"),
            ("ERP定律", "事件相关", "事件相关电位"),
            ("信号分析定律", "信号处理", "生物电信号分析"),
            ("噪声定律", "信号噪声", "信号中的噪声"),
            ("滤波定律", "信号滤波", "信号滤波处理"),
        ]
    }

    /// 电感受定律
    pub fn electroreception(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("电感受器定律", "电场感受", "感受电场"),
            ("鲨鱼电感受定律", "猎物探测", "鲨鱼探测猎物电"),
            ("电鱼定律", "电定位", "电鱼用电定位"),
            ("电通量定律", "电通量密度", "电通量密度感受"),
            ("低频电定律", "低频感受", "感受低频电"),
            ("高频电定律", "高频感受", "感受高频电"),
            ("主动电定位定律", "主动探测", "主动发电探测"),
        ]
    }

    /// 生物电产生定律
    pub fn bioelectric_generation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("发电器官定律", "发电器官", "某些鱼有发电器官"),
            ("电鱼发电定律", "产生电流", "电鱼产生电流"),
            ("电压定律", "电鱼电压", "电鱼产生电压"),
            ("电流定律", "电鱼电流", "电鱼产生电流"),
            ("频率定律", "发电频率", "发电频率"),
            ("脉冲定律", "电脉冲", "电脉冲模式"),
            ("调制定律", "电调制", "电信号调制"),
        ]
    }

    /// 电信号传递定律
    pub fn electrical_transmission(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("电突触定律", "缝隙连接", "电突触直接传递"),
            ("快速传递定律", "传递速度", "电突触传递快"),
            ("双向传递定律", "双向传导", "电突触双向传导"),
            ("同步定律", "同步放电", "电突触使同步放电"),
            ("电传导定律", "传导特性", "电传导特性"),
            ("衰减定律", "信号衰减", "电信号衰减"),
            ("放大定律", "信号放大", "电信号放大"),
        ]
    }

    /// 电学测量定律
    pub fn electrical_measurement(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("膜电位测量定律", "电位记录", "膜电位记录"),
            ("电压钳定律", "电位控制", "电压钳控制电位"),
            ("电流钳定律", "电流控制", "电流钳控制电流"),
            ("细胞内记录定律", "细胞内测量", "细胞内记录"),
            ("细胞外记录定律", "细胞外测量", "细胞外记录"),
            ("多点记录定律", "多电极", "多电极记录"),
            ("实时记录定律", "实时监测", "实时监测电位"),
        ]
    }

    /// 电学调节定律
    pub fn electrical_modulation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("电刺激定律", "刺激效应", "电刺激效应"),
            ("电疗定律", "电治疗", "电疗应用"),
            ("深部脑刺激定律", "DBS治疗", "深部脑刺激治疗"),
            ("心脏起搏定律", "心脏起搏", "心脏起搏器"),
            ("电休克定律", "电休克治疗", "电休克治疗"),
            ("电麻醉定律", "电麻醉", "电麻醉"),
            ("电调节定律", "功能调节", "电调节生物功能"),
        ]
    }

    /// 生物电学应用定律
    pub fn bioelectricity_applications(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("诊断定律", "电诊断", "生物电诊断疾病"),
            ("监测定律", "电监测", "生物电监测"),
            ("治疗定律", "电治疗", "电学治疗"),
            ("研究定律", "电研究", "生物电研究"),
            ("仿生定律", "电仿生", "电学仿生"),
            ("传感定律", "电传感", "电学生物传感"),
            ("通信定律", "电通信", "生物电通信"),
        ]
    }
}

impl Default for BioelectricityRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BioelectricityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("bioelectricity")
    }

    fn explain(&self) -> String {
        format!(
            "【生物电学规则】\n\n\
            生物电学研究生物体的电学现象，是神经科学和医学的基础。\n\n\
            神经电活动:\n{}\n\n\
            心脏电活动:\n{}\n\n\
            肌肉电活动:\n{}\n\n\
            生物电信号:\n{}\n\n\
            电感受:\n{}\n\n\
            生物电产生:\n{}\n\n\
            电信号传递:\n{}\n\n\
            电学测量:\n{}\n\n\
            电学调节:\n{}\n\n\
            生物电学应用:\n{}",
            self.neural_electrical()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cardiac_electrical()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.muscle_electrical()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bioelectric_signals()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.electroreception()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bioelectric_generation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.electrical_transmission()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.electrical_measurement()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.electrical_modulation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bioelectricity_applications()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bioelectricity_rules() {
        let rules = BioelectricityRules::new();
        assert_eq!(rules.neural_electrical().len(), 7);
        assert_eq!(rules.cardiac_electrical().len(), 7);
        assert_eq!(rules.muscle_electrical().len(), 7);
        assert_eq!(rules.bioelectric_signals().len(), 7);
        assert_eq!(rules.electroreception().len(), 7);
        assert_eq!(rules.bioelectric_generation().len(), 7);
        assert_eq!(rules.electrical_transmission().len(), 7);
        assert_eq!(rules.electrical_measurement().len(), 7);
        assert_eq!(rules.electrical_modulation().len(), 7);
        assert_eq!(rules.bioelectricity_applications().len(), 7);
    }

    #[test]
    fn test_bioelectricity_metadata() {
        let rules = BioelectricityRules::new();
        assert_eq!(rules.metadata().name, "生物电学规则");
    }
}
