//! 动力学规则
//!
//! 动力学研究物体运动与力之间的关系，是经典力学核心内容。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: DynamicsRules,
    name: "动力学规则",
    desc: "动力学基本定律与运动分析方法",
    origin: "力学",
    tags: ["科学", "物理", "力学", "动力学"]
}

impl DynamicsRules {
    /// 牛顿定律
    pub fn newton_laws(&self) -> Vec<&'static str> {
        vec![
            "牛顿第一定律（惯性定律）: 物体不受力或受力平衡时保持静止或匀速直线运动",
            "牛顿第二定律: F = ma，加速度与力成正比、与质量成反比",
            "牛顿第三定律: 作用力与反作用力大小相等、方向相反、作用在不同物体上",
            "惯性: 物体保持原有运动状态的性质",
            "惯性参考系: 牛顿定律适用的参考系",
            "力的独立性: 各方向力独立作用",
            "叠加原理: 合力等于各分力的矢量和",
            "瞬时性: 力与加速度同时存在同时消失",
        ]
    }

    /// 力的分类
    pub fn force_types(&self) -> Vec<&'static str> {
        vec![
            "重力: G = mg，地球对物体的吸引力",
            "弹力: 物体形变产生的力，F = kx（胡克定律）",
            "摩擦力: 阻碍相对运动的力",
            "静摩擦力: 0 ≤ f ≤ μsN",
            "滑动摩擦力: f = μkN",
            "空气阻力: 与速度有关，F = kv²（高速）或 F = kv（低速）",
            "万有引力: F = GMm/r²",
            "电磁力: 电荷间或磁体间的作用力",
        ]
    }

    /// 运动方程
    pub fn motion_equations(&self) -> Vec<&'static str> {
        vec![
            "匀加速直线运动: v = v₀ + at",
            "位移公式: s = v₀t + ½at²",
            "速度位移关系: v² = v₀² + 2as",
            "平均速度: v̄ = (v₀ + v)/2",
            "自由落体: v = gt，h = ½gt²",
            "竖直上抛: v = v₀ - gt，h = v₀t - ½gt²",
            "平抛运动: 水平匀速、竖直自由落体",
            "斜抛运动: 分解为水平和竖直方向",
        ]
    }

    /// 圆周运动
    pub fn circular_motion(&self) -> Vec<&'static str> {
        vec![
            "角速度: ω = 2π/T = 2πf",
            "线速度: v = ωr",
            "向心加速度: a = v²/r = ω²r",
            "向心力: F = mv²/r = mω²r",
            "匀速圆周运动: 角速度恒定",
            "离心现象: 向心力不足时物体远离圆心",
            "周期: T = 2π/ω = 2πr/v",
            "频率: f = 1/T = ω/2π",
        ]
    }

    /// 功和功率
    pub fn work_power(&self) -> Vec<&'static str> {
        vec![
            "功的定义: W = Fs cosθ",
            "正功: 力与位移同方向（0 ≤ θ < 90°）",
            "负功: 力与位移反方向（90° < θ ≤ 180°）",
            "功率: P = W/t = Fv",
            "恒力做功: W = Fs",
            "变力做功: W = ∫F·ds",
            "保守力做功: 与路径无关，只取决于起点终点",
            "非保守力做功: 与路径有关",
        ]
    }

    /// 能量
    pub fn energy(&self) -> Vec<&'static str> {
        vec![
            "动能: Ek = ½mv²",
            "势能: Ep = mgh（重力势能）或 Ep = ½kx²（弹性势能）",
            "机械能: E = Ek + Ep",
            "动能定理: W = ΔEk = Ek₂ - Ek₁",
            "机械能守恒: 只有保守力做功时机械能不变",
            "能量转化: 能量可以从一种形式转化为另一种形式",
            "能量守恒定律: 能量总量不变",
            "功能关系: W非保 = ΔE",
        ]
    }

    /// 动量和冲量
    pub fn momentum_impulse(&self) -> Vec<&'static str> {
        vec![
            "动量: p = mv，描述物体运动状态的矢量",
            "冲量: I = Ft，力对时间的累积效应",
            "动量定理: I = Δp = p₂ - p₁",
            "动量守恒定律: 系统不受外力或外力合力为零时总动量不变",
            "碰撞: 动量守恒，能量可能守恒或不守恒",
            "弹性碰撞: 动量和动能都守恒",
            "非弹性碰撞: 动量守恒、动能不守恒",
            "完全非弹性碰撞: 碰撞后物体粘在一起",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "车辆制动: 摩擦力与制动距离计算",
            "火箭发射: 推力与加速度计算",
            "卫星轨道: 万有引力提供向心力",
            "碰撞分析: 汽车碰撞、体育运动",
            "弹射装置: 弹簧储能与释放",
            "机械传动: 力、功率、效率",
            "流体运动: 流体动力学基础",
            "航空航天: 飞行器运动分析",
        ]
    }
}

impl Rule for DynamicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("dynamics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "动力学规则",
            &[
                ("牛顿定律", &self.newton_laws()),
                ("力的分类", &self.force_types()),
                ("运动方程", &self.motion_equations()),
                ("圆周运动", &self.circular_motion()),
                ("功和功率", &self.work_power()),
                ("能量", &self.energy()),
                ("动量和冲量", &self.momentum_impulse()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamics_rules() {
        let rules = DynamicsRules::new();
        assert_eq!(rules.metadata().name, "动力学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.newton_laws().is_empty());
        assert!(!rules.force_types().is_empty());
        assert!(!rules.motion_equations().is_empty());
    }
}