//! 声学详细规则
//!
//! 声学研究声音的产生、传播和接收。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 声学详细规则集合
pub struct AcousticsDetailedRules {
    metadata: RuleMetadata,
}

impl AcousticsDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("声学详细规则", "声学基本概念和定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "声学".into()]),
        }
    }

    /// 声波基础
    pub fn sound_wave_basics(&self) -> Vec<&'static str> {
        vec![
            "声波定义: 机械振动在介质中的传播",
            "声波类型: 纵波(气体液体)和横波(固体)",
            "声速: c = √(E/ρ) E为弹性模量ρ为密度",
            "空气中声速: 约340 m/s (20°C)",
            "水中声速: 约1500 m/s",
            "固体中声速: 约3000-6000 m/s",
            "声波波长: λ = c/f",
            "声波频率范围: 人耳可听20Hz-20kHz",
        ]
    }

    /// 声波传播规则
    pub fn sound_wave_propagation(&self) -> Vec<&'static str> {
        vec![
            "传播方程: 声压满足波动方程",
            "球面波: 从点源向外扩散的波",
            "平面波: 平行传播的波",
            "反射: 声波遇到界面返回",
            "折射: 声波进入新介质改变方向",
            "衍射: 声波绕过障碍物继续传播",
            "散射: 声波被不规则界面分散",
            "衰减: 声波能量随传播距离减弱",
        ]
    }

    /// 多普勒效应规则
    pub fn doppler_effect_rules(&self) -> Vec<&'static str> {
        vec![
            "多普勒效应: 声源和接收者相对运动时频率变化",
            "频率变化: f' = f(c±v_r)/(c±v_s)",
            "接近时频率升高: 声源靠近接收者",
            "远离时频率降低: 声源远离接收者",
            "马赫锥: 声源速度超过声速形成锥形冲击波",
            "超声速: 声源速度超过介质声速",
            "马赫数: M = v/c，声源速度与声速之比",
            "应用: 测速、雷达、医学成像",
        ]
    }

    /// 声学共振规则
    pub fn acoustic_resonance(&self) -> Vec<&'static str> {
        vec![
            "共振定义: 系统在特定频率下振动增强",
            "共振频率: 系统固有频率被激发",
            "驻波: 两反向传播波叠加形成",
            "声腔共振: 管或腔体的共振模式",
            "亥姆霍兹共振: 球形腔体共振",
            "品质因子Q: 共振峰尖锐程度的度量",
            "共振增强: 共振时振动幅度增大",
            "共振抑制: 频率偏离共振时振幅减小",
        ]
    }

    /// 声学阻抗规则
    pub fn acoustic_impedance_rules(&self) -> Vec<&'static str> {
        vec![
            "声阻抗定义: Z = p/v 声压与质点速度之比",
            "特性阻抗: Z₀ = ρc 介质特性阻抗",
            "阻抗匹配: 两介质阻抗相等时透射最大",
            "阻抗反射: 阻抗差异导致反射",
            "匹配层: 减少阻抗差异的中间层",
            "声阻抗测量: 用于材料性质检测",
            "阻抗应用: 超声换能器设计",
            "阻抗谱: 频率与阻抗的关系曲线",
        ]
    }

    /// 声强和声压规则
    pub fn sound_intensity_rules(&self) -> Vec<&'static str> {
        vec![
            "声强定义: 单位面积声功率流",
            "声强公式: I = p²/(ρc)",
            "声压定义: 声波引起的压力变化",
            "声压级: Lp = 20 log(p/p₀) dB",
            "声强级: LI = 10 log(I/I₀) dB",
            "参考值: p₀=20μPa I₀=10⁻¹²W/m²",
            "响度: 人耳感知的声音强度",
            "响度级: 方(phon)为单位",
        ]
    }

    /// 声学材料规则
    pub fn acoustic_materials(&self) -> Vec<&'static str> {
        vec![
            "吸声材料: 吸收声波能量的材料",
            "隔声材料: 阻挡声波传播的材料",
            "吸声系数: 材料吸收声能的比例",
            "隔声量: 材料阻挡声能的度量",
            "多孔吸声: 多孔材料吸收声能",
            "共振吸声: 共振结构吸收声能",
            "质量定律: 隔声量随材料质量增加",
            "复合结构: 多层材料组合优化声学性能",
        ]
    }

    /// 声学应用
    pub fn acoustic_applications(&self) -> Vec<&'static str> {
        vec![
            "超声成像",
            "声学降噪",
            "建筑声学",
            "音乐厅设计",
            "声学检测",
            "声纳探测",
            "语音通信",
            "声学计量",
        ]
    }
}

impl Default for AcousticsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AcousticsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("acoustics_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "声学详细规则",
            &[
                ("声波基础", &self.sound_wave_basics()),
                ("声波传播", &self.sound_wave_propagation()),
                ("多普勒效应", &self.doppler_effect_rules()),
                ("声学共振", &self.acoustic_resonance()),
                ("声学阻抗", &self.acoustic_impedance_rules()),
                ("声强声压", &self.sound_intensity_rules()),
                ("声学材料", &self.acoustic_materials()),
                ("声学应用", &self.acoustic_applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acoustics_detailed_rules() {
        let rules = AcousticsDetailedRules::new();
        assert_eq!(rules.metadata().name, "声学详细规则");
        assert!(!rules.sound_wave_basics().is_empty());
        assert!(!rules.sound_wave_propagation().is_empty());
        assert!(!rules.doppler_effect_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }
}