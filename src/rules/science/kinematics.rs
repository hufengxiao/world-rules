//! 运动学规则
//!
//! 运动学研究物体运动的几何性质，不涉及力。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: KinematicsRules,
    name: "运动学规则",
    desc: "运动学基本概念与运动描述方法",
    origin: "力学",
    tags: ["科学", "物理", "力学", "运动学"]
}

impl KinematicsRules {
    /// 基本概念
    pub fn basic_concepts(&self) -> Vec<&'static str> {
        vec![
            "参考系: 描述物体运动的参照物和坐标系",
            "坐标系: 描述位置的数学工具",
            "位置: 物体在空间中的坐标",
            "位移: 位置的变化，矢量",
            "路程: 实际运动轨迹的长度",
            "速度: 位移与时间之比，v = Δr/Δt",
            "速率: 路程与时间之比",
            "加速度: 速度变化率，a = Δv/Δt",
        ]
    }

    /// 直线运动
    pub fn linear_motion(&self) -> Vec<&'static str> {
        vec![
            "匀速直线运动: v = const，s = vt",
            "匀加速直线运动: v = v₀ + at",
            "位移公式: s = v₀t + ½at²",
            "速度位移关系: v² - v₀² = 2as",
            "平均速度公式: v̄ = s/t = (v₀ + v)/2",
            "自由落体: v₀ = 0，a = g，h = ½gt²",
            "竖直上抛: v = v₀ - gt，h = v₀t - ½gt²",
            "竖直下抛: v = v₀ + gt，h = v₀t + ½gt²",
        ]
    }

    /// 曲线运动
    pub fn curvilinear_motion(&self) -> Vec<&'static str> {
        vec![
            "位置矢量: r(t) 描述物体位置随时间变化",
            "速度矢量: v = dr/dt",
            "加速度矢量: a = dv/dt = d²r/dt²",
            "切向加速度: aₜ = dv/dt",
            "法向加速度: aₙ = v²/r",
            "抛体运动: 合成两个方向的直线运动",
            "平抛运动: x = v₀t，y = ½gt²",
            "斜抛运动: x = v₀cosθ·t，y = v₀sinθ·t - ½gt²",
        ]
    }

    /// 圆周运动
    pub fn circular_motion(&self) -> Vec<&'static str> {
        vec![
            "角位置: θ，描述物体在圆周上的位置",
            "角位移: Δθ = θ₂ - θ₁",
            "角速度: ω = dθ/dt",
            "角加速度: α = dω/dt",
            "匀速圆周运动: ω = const",
            "匀加速圆周运动: ω = ω₀ + αt",
            "线角关系: v = ωr，aₜ = αr",
            "向心加速度: aₙ = v²/r = ω²r",
        ]
    }

    /// 相对运动
    pub fn relative_motion(&self) -> Vec<&'static str> {
        vec![
            "相对位移: Δr' = Δr - Δr₀",
            "相对速度: v' = v - v₀",
            "相对加速度: a' = a - a₀",
            "伽利略变换: x' = x - v₀t",
            "速度合成: v绝对 = v相对 + v牵连",
            "加速度合成: a绝对 = a相对 + a牵连",
            "相对运动分解: 沿牵连速度方向和垂直方向",
            "参考系变换: 不同参考系下的运动描述",
        ]
    }

    /// 运动图像
    pub fn motion_graphs(&self) -> Vec<&'static str> {
        vec![
            "位移-时间图: 描述位移随时间变化",
            "速度-时间图: 描述速度随时间变化",
            "加速度-时间图: 描述加速度随时间变化",
            "图像斜率: v-t 图斜率为加速度",
            "图像面积: v-t 图面积为位移",
            "匀速运动: s-t 图为直线",
            "匀加速运动: v-t 图为直线",
            "运动分析: 通过图像分析运动状态",
        ]
    }

    /// 运动约束
    pub fn motion_constraints(&self) -> Vec<&'static str> {
        vec![
            "几何约束: 物体运动受几何条件限制",
            "速度约束: 关联物体的速度关系",
            "绳索约束: 绳索两端速度大小相等",
            "杆件约束: 杆件两端速度沿杆方向相等",
            "接触约束: 接触点沿切线方向速度相同",
            "约束方程: 描述约束条件的数学方程",
            "自由度: 物体独立运动参数的数目",
            "约束反力: 约束对物体的反作用力",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "车辆运动: 汽车加速、制动分析",
            "体育运动: 投掷、跳跃运动分析",
            "弹道计算: 抛体运动的轨迹",
            "天体运动: 卫星、行星的运动",
            "机械运动: 齿轮、连杆的运动",
            "流体运动: 流体的运动轨迹",
            "生物运动: 动物奔跑、飞行",
            "机器人运动: 机械臂的运动规划",
        ]
    }
}

impl Rule for KinematicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("kinematics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "运动学规则",
            &[
                ("基本概念", &self.basic_concepts()),
                ("直线运动", &self.linear_motion()),
                ("曲线运动", &self.curvilinear_motion()),
                ("圆周运动", &self.circular_motion()),
                ("相对运动", &self.relative_motion()),
                ("运动图像", &self.motion_graphs()),
                ("运动约束", &self.motion_constraints()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinematics_rules() {
        let rules = KinematicsRules::new();
        assert_eq!(rules.metadata().name, "运动学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.basic_concepts().is_empty());
        assert!(!rules.linear_motion().is_empty());
        assert!(!rules.circular_motion().is_empty());
    }
}
