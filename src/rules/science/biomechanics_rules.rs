//! 生物力学规则
//!
//! 生物体力学行为的生物学原理，包括骨骼力学、肌肉力学、
//! 血液动力学、运动力学等核心概念。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 生物力学规则集合
pub struct BiomechanicsRules {
    metadata: RuleMetadata,
}

impl BiomechanicsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("生物力学规则", "生物体力学行为的生物学原理")
                .with_origin("生物力学")
                .with_tags(vec!["科学".into(), "生命科学".into(), "力学".into()]),
        }
    }

    /// 骨骼力学定律
    pub fn bone_mechanics(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("沃尔夫定律", "适应载荷", "骨骼根据载荷重塑"),
            ("应力定律", "刺激骨生长", "应力刺激骨形成"),
            ("弹性定律", "弹性变形", "骨骼弹性范围内变形"),
            ("塑性定律", "塑性变形", "超过弹性发生塑性变形"),
            ("骨折定律", "断裂阈值", "超过阈值骨骼断裂"),
            ("密度定律", "强度相关", "骨密度与强度相关"),
            ("结构定律", "优化结构", "骨骼结构优化承载"),
        ]
    }

    /// 肌肉力学定律
    pub fn muscle_mechanics(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("肌肉收缩定律", "张力产生", "肌肉收缩产生张力"),
            ("长度张力定律", "最优长度", "最优长度产生最大张力"),
            ("力量速度定律", "反向关系", "力量与速度反向关系"),
            ("肌丝滑行定律", "收缩机制", "肌丝滑行产生收缩"),
            ("肌肉疲劳定律", "疲劳机制", "疲劳降低收缩力"),
            ("弹性成分定律", "储能释放", "肌肉弹性储能释放"),
            ("协同收缩定律", "稳定关节", "拮抗肌协同收缩"),
        ]
    }

    /// 关节力学定律
    pub fn joint_mechanics(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("关节稳定定律", "结构稳定", "关节结构维持稳定"),
            ("运动范围定律", "活动范围", "关节活动范围"),
            ("润滑定律", "关节润滑", "关节滑液减少摩擦"),
            ("载荷分布定律", "压力分散", "关节分散压力"),
            ("韧带定律", "限制运动", "韧带限制过度运动"),
            ("软骨定律", "缓冲作用", "软骨缓冲关节冲击"),
            ("关节力定律", "合力计算", "关节承受力可计算"),
        ]
    }

    /// 血液动力学定律
    pub fn hemodynamics(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("泊肃叶定律", "流量公式", "流量与压力差半径四次方相关"),
            ("血管阻力定律", "阻力公式", "血管阻力与长度半径相关"),
            ("血压定律", "压力梯度", "血压沿血管逐渐降低"),
            ("血流定律", "流速变化", "血管越细流速越快"),
            ("脉搏定律", "脉动传播", "脉搏沿血管传播"),
            ("层流定律", "正常流动", "正常血流为层流"),
            ("湍流定律", "异常流动", "狭窄处产生湍流"),
        ]
    }

    /// 心脏力学定律
    pub fn cardiac_mechanics(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("心输出量定律", "HR×SV", "心输出量等于心率乘每搏量"),
            ("压力容积定律", "心脏做功", "心脏压力容积关系"),
            ("收缩定律", "主动收缩", "心肌主动收缩产生压力"),
            ("舒张定律", "被动充盈", "心脏舒张被动充盈"),
            ("射血定律", "射血分数", "射血分数衡量收缩功能"),
            ("前负荷定律", "初始负荷", "前负荷影响心输出量"),
            ("后负荷定律", "射血阻力", "后负荷影响射血阻力"),
        ]
    }

    /// 呼吸力学定律
    pub fn respiratory_mechanics(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("肺通气定律", "进出气体", "肺通气进出气体"),
            ("肺容量定律", "容量变化", "肺容量周期变化"),
            ("肺顺应性定律", "弹性特性", "肺顺应性衡量弹性"),
            ("气道阻力定律", "阻力公式", "气道阻力影响气流"),
            ("呼吸功定律", "做功计算", "呼吸克服阻力做功"),
            ("压力梯度定律", "驱动气流", "压力差驱动气流"),
            ("表面张力定律", "肺泡张力", "肺泡表面张力"),
        ]
    }

    /// 运动力学定律
    pub fn locomotion_mechanics(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("步态定律", "行走模式", "步态周期性模式"),
            ("支撑定律", "支撑阶段", "支撑阶段承受载荷"),
            ("摆动定律", "摆动阶段", "摆动阶段腿摆动"),
            ("重心定律", "重心轨迹", "重心周期性轨迹"),
            ("能量定律", "能量效率", "运动能量效率"),
            ("协调定律", "协调运动", "肢体协调运动"),
            ("稳定性定律", "平衡维持", "运动中维持平衡"),
        ]
    }

    /// 细胞力学定律
    pub fn cell_mechanics(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("细胞膜力学定律", "膜弹性", "细胞膜弹性变形"),
            ("细胞骨架定律", "力学支撑", "细胞骨架提供支撑"),
            ("细胞变形定律", "形态变化", "细胞可变形"),
            ("细胞迁移定律", "力学驱动", "力学驱动细胞迁移"),
            ("力学感受定律", "感知应力", "细胞感知力学信号"),
            ("力学响应定律", "应力响应", "细胞响应力学信号"),
            ("力学传导定律", "信号传导", "力学信号转化为化学信号"),
        ]
    }

    /// 组织力学定律
    pub fn tissue_mechanics(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("弹性定律", "弹性模量", "组织弹性模量"),
            ("粘弹性定律", "粘弹特性", "组织粘弹性特性"),
            ("非线性定律", "非线性响应", "组织非线性力学响应"),
            ("各向异性定律", "方向差异", "组织各向异性"),
            ("预应力定律", "预应力状态", "组织存在预应力"),
            ("应力松弛定律", "松弛特性", "组织应力松弛"),
            ("蠕变定律", "蠕变特性", "组织蠕变变形"),
        ]
    }

    /// 生物力学应用定律
    pub fn biomechanics_applications(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("临床诊断定律", "疾病诊断", "力学参数用于诊断"),
            ("运动训练定律", "优化训练", "力学分析优化训练"),
            ("康复定律", "康复评估", "力学评估康复进展"),
            ("植入物定律", "植入设计", "植入物力学设计"),
            ("假肢定律", "假肢设计", "假肢力学设计"),
            ("运动损伤定律", "损伤预防", "力学分析预防损伤"),
            ("姿势分析定律", "姿势评估", "力学分析评估姿势"),
        ]
    }
}

impl Default for BiomechanicsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BiomechanicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("biomechanics")
    }

    fn explain(&self) -> String {
        format!(
            "【生物力学规则】\n\n\
            生物力学研究生物体的力学行为，是医学和运动科学的基础。\n\n\
            骨骼力学:\n{}\n\n\
            肌肉力学:\n{}\n\n\
            关节力学:\n{}\n\n\
            血液动力学:\n{}\n\n\
            心脏力学:\n{}\n\n\
            呼吸力学:\n{}\n\n\
            运动力学:\n{}\n\n\
            细胞力学:\n{}\n\n\
            组织力学:\n{}\n\n\
            生物力学应用:\n{}",
            self.bone_mechanics()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.muscle_mechanics()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.joint_mechanics()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hemodynamics()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cardiac_mechanics()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.respiratory_mechanics()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.locomotion_mechanics()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cell_mechanics()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tissue_mechanics()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.biomechanics_applications()
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
    fn test_biomechanics_rules() {
        let rules = BiomechanicsRules::new();
        assert_eq!(rules.bone_mechanics().len(), 7);
        assert_eq!(rules.muscle_mechanics().len(), 7);
        assert_eq!(rules.joint_mechanics().len(), 7);
        assert_eq!(rules.hemodynamics().len(), 7);
        assert_eq!(rules.cardiac_mechanics().len(), 7);
        assert_eq!(rules.respiratory_mechanics().len(), 7);
        assert_eq!(rules.locomotion_mechanics().len(), 7);
        assert_eq!(rules.cell_mechanics().len(), 7);
        assert_eq!(rules.tissue_mechanics().len(), 7);
        assert_eq!(rules.biomechanics_applications().len(), 7);
    }

    #[test]
    fn test_biomechanics_metadata() {
        let rules = BiomechanicsRules::new();
        assert_eq!(rules.metadata().name, "生物力学规则");
    }
}
