//! 振动与波规则
//!
//! 振动与波研究振动现象和波动的传播规律。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: VibrationWaveRules,
    name: "振动与波规则",
    desc: "振动与波动的基本定律与分析方法",
    origin: "力学",
    tags: ["科学", "物理", "力学", "振动", "波动"]
}

impl VibrationWaveRules {
    /// 简谐振动
    pub fn simple_harmonic_motion(&self) -> Vec<&'static str> {
        vec![
            "简谐振动: 位移随时间正弦或余弦变化",
            "运动方程: x = A cos(ωt + φ)",
            "振幅 A: 最大位移",
            "频率 f: 每秒振动次数，Hz",
            "周期 T: 完成一次振动的时间 T = 1/f",
            "角频率 ω: ω = 2πf",
            "相位 φ: 初始相位角",
            "速度: v = -Aω sin(ωt + φ)",
        ]
    }

    /// 振动能量
    pub fn vibration_energy(&self) -> Vec<&'static str> {
        vec![
            "动能: Ek = ½mv² = ½mω²A² sin²(ωt+φ)",
            "势能: Ep = ½kx² = ½kA² cos²(ωt+φ)",
            "总能量: E = Ek + Ep = ½kA²",
            "能量守恒: 无阻尼振动总能量不变",
            "能量转换: 动能与势能交替转换",
            "平衡位置: 能量全部为动能",
            "最大位移: 能量全部为势能",
            "振动频率: ω = √(k/m)",
        ]
    }

    /// 阻尼振动
    pub fn damped_vibration(&self) -> Vec<&'static str> {
        vec![
            "阻尼振动: 有能量损失的振动",
            "阻尼力: Fd = -bv",
            "阻尼系数 b: 与阻尼力大小有关",
            "欠阻尼: 振幅逐渐衰减",
            "过阻尼: 不振动，缓慢回到平衡位置",
            "临界阻尼: 最快回到平衡位置而不振动",
            "衰减规律: A = A₀ e^(-βt)",
            "品质因子 Q: Q = ω₀/(2β)",
        ]
    }

    /// 受迫振动
    pub fn forced_vibration(&self) -> Vec<&'static str> {
        vec![
            "受迫振动: 受周期性外力作用的振动",
            "驱动力: F = F₀ cos(ωt)",
            "共振: 当驱动力频率接近固有频率时振幅最大",
            "共振频率: ω共振 ≈ ω₀",
            "共振振幅: A共振 = F₀/(2mβω₀)",
            "共振应用: 振动筛、共振电路",
            "共振危害: 机械破坏、噪声",
            "避免共振: 改变频率、增加阻尼",
        ]
    }

    /// 波动基础
    pub fn wave_basics(&self) -> Vec<&'static str> {
        vec![
            "波动: 振动在介质中的传播",
            "波的类型: 机械波、电磁波",
            "横波: 振动方向与传播方向垂直",
            "纵波: 振动方向与传播方向平行",
            "波长 λ: 一个周期内传播的距离",
            "波速 v: v = fλ",
            "波速公式: v = √(弹性/密度)",
            "波动方程: y = A sin(ωt - kx)",
        ]
    }

    /// 波的干涉
    pub fn wave_interference(&self) -> Vec<&'static str> {
        vec![
            "波的叠加: 两波相遇时的叠加",
            "干涉: 两波叠加形成稳定图案",
            "相干条件: 频率相同、相位差恒定",
            "相长干涉: 波峰相遇，振幅增大",
            "相消干涉: 波峰与波谷相遇，振幅减小",
            "干涉条纹: 明纹和暗纹交替",
            "驻波: 干涉形成的稳定波形",
            "驻波特点: 有波节和波腹",
        ]
    }

    /// 波的衍射
    pub fn wave_diffraction(&self) -> Vec<&'static str> {
        vec![
            "衍射: 波绕过障碍物的现象",
            "衍射条件: 障碍物尺寸与波长相近",
            "衍射角度: θ ≈ λ/a",
            "单缝衍射: 通过狭缝的衍射",
            "衍射图样: 中心亮纹、两侧暗纹交替",
            "衍射应用: 光栅、衍射天线",
            "全息技术: 利用衍射记录信息",
            "衍射极限: 光学系统分辨率限制",
        ]
    }

    /// 声波
    pub fn sound_waves(&self) -> Vec<&'static str> {
        vec![
            "声波: 空气中的纵波",
            "声速: v ≈ 340 m/s（常温空气）",
            "频率范围: 人耳可听 20Hz ~ 20kHz",
            "声强: 单位面积上的功率",
            "声强级: L = 10 log(I/I₀)，dB",
            "响度: 与声强和频率有关",
            "音调: 与频率有关",
            "音色: 与波形有关",
        ]
    }
}

impl Rule for VibrationWaveRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("vibration_wave")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "振动与波规则",
            &[
                ("简谐振动", &self.simple_harmonic_motion()),
                ("振动能量", &self.vibration_energy()),
                ("阻尼振动", &self.damped_vibration()),
                ("受迫振动", &self.forced_vibration()),
                ("波动基础", &self.wave_basics()),
                ("波的干涉", &self.wave_interference()),
                ("波的衍射", &self.wave_diffraction()),
                ("声波", &self.sound_waves()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibration_wave_rules() {
        let rules = VibrationWaveRules::new();
        assert_eq!(rules.metadata().name, "振动与波规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.simple_harmonic_motion().is_empty());
        assert!(!rules.wave_basics().is_empty());
        assert!(!rules.sound_waves().is_empty());
    }
}