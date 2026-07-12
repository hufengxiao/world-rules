//! 生物热力学规则
//!
//! 生物体热力学现象和原理，包括体温调节、能量代谢、
//! 热产生、热散失等核心概念。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 生物热力学规则集合
pub struct BiothermodynamicsRules {
    metadata: RuleMetadata,
}

impl BiothermodynamicsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("生物热力学规则", "生物体热力学现象和原理")
                .with_origin("生物热力学")
                .with_tags(vec!["科学".into(), "生命科学".into(), "热力学".into()]),
        }
    }

    /// 体温调节定律
    pub fn temperature_regulation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("恒温定律", "体温恒定", "恒温动物维持体温"),
            ("变温定律", "随环境变化", "变温动物体温变化"),
            ("体温设定点定律", "调节目标", "体温调节设定点"),
            ("负反馈定律", "调节机制", "体温负反馈调节"),
            ("中枢调节定律", "中枢控制", "中枢控制体温"),
            ("行为调节定律", "行为调节", "行为调节体温"),
            ("体温范围定律", "正常范围", "正常体温范围"),
        ]
    }

    /// 热产生定律
    pub fn heat_production(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("代谢产热定律", "基础代谢", "代谢产生热量"),
            ("肌肉产热定律", "运动产热", "肌肉运动产热"),
            ("颤抖产热定律", "颤抖生热", "颤抖产生热量"),
            ("非颤抖产热定律", "褐色脂肪", "褐色脂肪产热"),
            ("ATP产热定律", "能量释放", "ATP水解产热"),
            ("氧化产热定律", "氧化反应", "氧化产生热量"),
            ("产热效率定律", "产热效率", "产热效率不同"),
        ]
    }

    /// 热散失定律
    pub fn heat_loss(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("传导散热定律", "热量传导", "热量传导散失"),
            ("对流散热定律", "热量对流", "热量对流散失"),
            ("辐射散热定律", "热量辐射", "热量辐射散失"),
            ("蒸发散热定律", "汗液蒸发", "汗液蒸发散热"),
            ("呼吸散热定律", "呼吸散热", "呼吸散失热量"),
            ("散热调节定律", "散热控制", "控制散热"),
            ("散热效率定律", "散热效率", "散热效率"),
        ]
    }

    /// 能量代谢定律
    pub fn energy_metabolism(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("能量守恒定律", "摄入=消耗+存储", "能量守恒方程"),
            ("基础代谢定律", "BMR", "基础代谢率"),
            ("代谢率定律", "代谢速率", "代谢速率计算"),
            ("ATP能量定律", "能量货币", "ATP是能量货币"),
            ("氧化磷酸化定律", "能量生成", "氧化磷酸化生成ATP"),
            ("糖酵解定律", "无氧代谢", "糖酵解产生能量"),
            ("能量平衡定律", "能量平衡", "能量摄入消耗平衡"),
        ]
    }

    /// 温度适应定律
    pub fn temperature_adaptation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热适应定律", "适应高温", "适应高温环境"),
            ("冷适应定律", "适应低温", "适应低温环境"),
            ("冷驯化定律", "冷驯化", "冷驯化过程"),
            ("热驯化定律", "热驯化", "热驯化过程"),
            ("代谢适应定律", "代谢调整", "代谢适应温度"),
            ("形态适应定律", "形态改变", "形态适应温度"),
            ("行为适应定律", "行为调整", "行为适应温度"),
        ]
    }

    /// 热应激定律
    pub fn thermal_stress(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热应激定律", "高温应激", "高温应激反应"),
            ("冷应激定律", "低温应激", "低温应激反应"),
            ("热休克定律", "热休克蛋白", "热休克蛋白响应"),
            ("热损伤定律", "高温损伤", "高温损伤"),
            ("冷损伤定律", "低温损伤", "低温损伤"),
            ("热致死定律", "致死温度", "致死温度阈值"),
            ("耐热定律", "耐热能力", "耐热能力差异"),
        ]
    }

    /// 组织温度定律
    pub fn tissue_temperature(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("核心温度定律", "体内温度", "核心体温"),
            ("体表温度定律", "皮肤温度", "皮肤温度较低"),
            ("温度梯度定律", "温度差异", "体内温度梯度"),
            ("局部温度定律", "局部差异", "不同部位温度不同"),
            ("器官温度定律", "器官温度", "不同器官温度"),
            ("血流温度定律", "血流调节", "血流调节温度"),
            ("温度测量定律", "温度测定", "测量体温方法"),
        ]
    }

    /// 热传递定律
    pub fn heat_transfer(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("血流传递定律", "血流传热", "血流传递热量"),
            ("组织传递定律", "组织传导", "组织传导热量"),
            ("热传导率定律", "传导率", "组织热传导率"),
            ("血管调节定律", "血管调节", "血管调节传热"),
            ("热传递系数定律", "传递系数", "热传递系数"),
            ("热平衡定律", "热平衡", "热量平衡状态"),
            ("热响应定律", "热响应时间", "温度响应时间"),
        ]
    }

    /// 热感受定律
    pub fn thermal_sensation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热感受器定律", "温度感受", "感受温度变化"),
            ("热敏神经定律", "热敏神经", "热敏神经响应"),
            ("冷敏神经定律", "冷敏神经", "冷敏神经响应"),
            ("温度阈值定律", "感受阈值", "温度感受阈值"),
            ("温度编码定律", "温度编码", "温度信号编码"),
            ("温度感知定律", "温度感知", "感知温度"),
            ("舒适温度定律", "舒适范围", "舒适温度范围"),
        ]
    }

    /// 生物热力学应用定律
    pub fn biothermodynamics_applications(
        &self,
    ) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("体温监测定律", "体温监测", "监测体温"),
            ("体温调节定律", "调节体温", "调节体温技术"),
            ("热疗定律", "热治疗", "热疗应用"),
            ("冷疗定律", "冷治疗", "冷疗应用"),
            ("保温定律", "保温技术", "保温应用"),
            ("散热定律", "散热技术", "散热应用"),
            ("热成像定律", "热成像", "热成像技术"),
        ]
    }
}

impl Default for BiothermodynamicsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BiothermodynamicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("biothermodynamics")
    }

    fn explain(&self) -> String {
        format!(
            "【生物热力学规则】\n\n\
            生物热力学研究生物体的热力学现象，是生理学和医学的基础。\n\n\
            体温调节:\n{}\n\n\
            热产生:\n{}\n\n\
            热散失:\n{}\n\n\
            能量代谢:\n{}\n\n\
            温度适应:\n{}\n\n\
            热应激:\n{}\n\n\
            组织温度:\n{}\n\n\
            热传递:\n{}\n\n\
            热感受:\n{}\n\n\
            生物热力学应用:\n{}",
            self.temperature_regulation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.heat_production()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.heat_loss()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.energy_metabolism()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.temperature_adaptation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.thermal_stress()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tissue_temperature()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.heat_transfer()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.thermal_sensation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.biothermodynamics_applications()
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
    fn test_biothermodynamics_rules() {
        let rules = BiothermodynamicsRules::new();
        assert_eq!(rules.temperature_regulation().len(), 7);
        assert_eq!(rules.heat_production().len(), 7);
        assert_eq!(rules.heat_loss().len(), 7);
        assert_eq!(rules.energy_metabolism().len(), 7);
        assert_eq!(rules.temperature_adaptation().len(), 7);
        assert_eq!(rules.thermal_stress().len(), 7);
        assert_eq!(rules.tissue_temperature().len(), 7);
        assert_eq!(rules.heat_transfer().len(), 7);
        assert_eq!(rules.thermal_sensation().len(), 7);
        assert_eq!(rules.biothermodynamics_applications().len(), 7);
    }

    #[test]
    fn test_biothermodynamics_metadata() {
        let rules = BiothermodynamicsRules::new();
        assert_eq!(rules.metadata().name, "生物热力学规则");
    }
}
