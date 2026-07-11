//! 生物物理规则
//!
//! 生物物理学研究生物系统的物理原理。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 生物物理规则集合
pub struct BiophysicsRules {
    metadata: RuleMetadata,
}

impl BiophysicsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("生物物理规则", "生物物理学基本概念和定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "生物物理".into()]),
        }
    }

    /// 生物分子物理规则
    pub fn biomolecular_physics(&self) -> Vec<&'static str> {
        vec![
            "蛋白质结构: 一级二级三级四级结构层次",
            "蛋白质折叠: 蛋白质从线性到立体结构的过程",
            "DNA双螺旋: DNA的两条链缠绕形成双螺旋",
            "RNA结构: 单链RNA可形成复杂结构",
            "氢键作用: 维持蛋白质和核酸结构的稳定性",
            "疏水效应: 疏水基团倾向于聚集",
            "静电相互作用: 生物分子间的电荷作用",
            "范德华力: 生物分子间的弱相互作用",
        ]
    }

    /// 细胞物理规则
    pub fn cellular_physics(&self) -> Vec<&'static str> {
        vec![
            "细胞膜结构: 磷脂双分子层构成",
            "膜电位: 细胞膜内外电位差约-70mV",
            "离子通道: 允许离子通过细胞膜",
            "渗透压: 溶质浓度差异引起的压力",
            "细胞骨架: 维持细胞形态的蛋白质网络",
            "细胞运输: 物质进出细胞的物理过程",
            "细胞分裂: 细胞一分为二的物理机制",
            "细胞粘附: 细胞间相互附着的机制",
        ]
    }

    /// 生物力学规则
    pub fn biomechanics(&self) -> Vec<&'static str> {
        vec![
            "骨骼力学: 骨骼承受载荷的力学特性",
            "肌肉力学: 肌肉收缩和力量产生",
            "关节力学: 关节运动和承载",
            "血流动力学: 血液在血管中的流动",
            "呼吸力学: 气体进出肺的物理过程",
            "心脏力学: 心脏泵血的力学原理",
            "运动力学: 人体运动的物理分析",
            "组织力学: 生物组织的力学性质",
        ]
    }

    /// 生物电学规则
    pub fn bioelectricity(&self) -> Vec<&'static str> {
        vec![
            "神经信号: 神经细胞传递的电信号",
            "动作电位: 神经细胞快速电位变化",
            "离子梯度: 细胞内外离子浓度差异",
            "静息电位: 神经细胞静息状态的电位",
            "电突触: 电流直接传递的神经连接",
            "化学突触: 通过化学物质传递信号",
            "心电图: 心脏电活动的记录",
            "脑电图: 大脑电活动的记录",
        ]
    }

    /// 生物光学规则
    pub fn biophotonics(&self) -> Vec<&'static str> {
        vec![
            "视觉机制: 光信号转换为神经信号",
            "视网膜: 眼睛的感光层",
            "光感受器: 视杆细胞和视锥细胞",
            "光合作用: 植物将光能转化为化学能",
            "荧光效应: 生物分子吸收光后发光",
            "生物发光: 生物体产生光",
            "光学成像: 光学技术观察生物样品",
            "光治疗: 用光进行医学治疗",
        ]
    }

    /// 生物热学规则
    pub fn biothermodynamics(&self) -> Vec<&'static str> {
        vec![
            "代谢热: 生物代谢过程产生热量",
            "体温调节: 维持体温稳定的机制",
            "热传导: 生物体内热量传递",
            "热辐射: 生物体向环境辐射热量",
            "热对流: 血流传递热量",
            "蒸发散热: 通过汗液蒸发散热",
            "热平衡: 生物体与环境的能量交换",
            "代谢率: 生物体能量消耗速率",
        ]
    }

    /// 生物声学规则
    pub fn bioacoustics(&self) -> Vec<&'static str> {
        vec![
            "听觉机制: 声波转换为神经信号",
            "耳蜗: 内耳的声音感受器",
            "声音频率感知: 不同频率声音的感知",
            "声波传导: 声波在外耳中耳的传导",
            "生物发声: 生物体产生声音",
            "声波回声定位: 动物用声音定位",
            "声波通信: 动物用声音交流",
            "声波医学应用: 超声成像和治疗",
        ]
    }

    /// 生物辐射规则
    pub fn biological_radiation(&self) -> Vec<&'static str> {
        vec![
            "辐射损伤: 辐射对生物体的损伤",
            "DNA损伤: 辐射导致DNA结构改变",
            "辐射修复: 生物体修复辐射损伤",
            "辐射剂量: 生物体接受的辐射量",
            "辐射敏感性: 不同组织对辐射的敏感性",
            "辐射防护: 减少辐射损伤的措施",
            "辐射治疗: 用辐射治疗疾病",
            "辐射标记: 用放射性标记研究生物过程",
        ]
    }

    /// 应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "医学成像",
            "生物力学研究",
            "生物技术",
            "医学诊断",
            "医学治疗",
            "药物设计",
            "生物传感器",
            "仿生技术",
        ]
    }
}

impl Default for BiophysicsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BiophysicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("biophysics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "生物物理规则",
            &[
                ("生物分子物理", &self.biomolecular_physics()),
                ("细胞物理", &self.cellular_physics()),
                ("生物力学", &self.biomechanics()),
                ("生物电学", &self.bioelectricity()),
                ("生物光学", &self.biophotonics()),
                ("生物热学", &self.biothermodynamics()),
                ("生物声学", &self.bioacoustics()),
                ("生物辐射", &self.biological_radiation()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biophysics_rules() {
        let rules = BiophysicsRules::new();
        assert_eq!(rules.metadata().name, "生物物理规则");
        assert!(!rules.biomolecular_physics().is_empty());
        assert!(!rules.cellular_physics().is_empty());
        assert!(!rules.biomechanics().is_empty());
        assert!(!rules.explain().is_empty());
    }
}
