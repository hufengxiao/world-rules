//! 天线理论规则
//!
//! 天线理论研究天线的辐射特性、设计和应用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: AntennaTheoryRules,
    name: "天线理论规则",
    desc: "天线设计与应用方法",
    origin: "电磁学",
    tags: ["科学", "物理", "电磁", "天线"]
}

impl AntennaTheoryRules {
    /// 天线基础
    pub fn antenna_basics(&self) -> Vec<&'static str> {
        vec![
            "天线定义: 发射和接收电磁波的装置",
            "辐射原理: 电流在天线中振荡产生电磁波",
            "天线功能: 无线通信的关键器件",
            "天线类型: 线天线、面天线、阵列天线",
            "天线极化: 辐射电磁波的极化方式",
            "天线带宽: 工作频率范围",
            "天线效率: 有效辐射比例",
            "天线阻抗: 输入端阻抗",
        ]
    }

    /// 天线参数
    pub fn antenna_parameters(&self) -> Vec<&'static str> {
        vec![
            "增益: G = 4πAₑ/λ²，辐射集中程度",
            "方向图: 辐射强度的空间分布",
            "主瓣: 最大辐射方向",
            "旁瓣: 主瓣以外的辐射",
            "波束宽度: 主瓣宽度",
            "前后比: 主瓣与背瓣功率比",
            "效率: η = P辐射/P输入",
            "阻抗: 输入端特性阻抗",
        ]
    }

    /// 基本天线类型
    pub fn basic_antenna_types(&self) -> Vec<&'static str> {
        vec![
            "偶极子天线: 半波长偶极子",
            "单极子天线: 四分之一波长单极子",
            "环形天线: 环形导体天线",
            "螺旋天线: 螺旋形天线",
            "喇叭天线: 波导开口天线",
            "抛物面天线: 反射聚焦天线",
            "微带天线: PCB平面天线",
            "缝隙天线: 矩形缝隙天线",
        ]
    }

    /// 天线阵列
    pub fn antenna_arrays(&self) -> Vec<&'static str> {
        vec![
            "阵列天线: 多个天线单元组合",
            "直线阵列: 单元沿直线排列",
            "平面阵列: 单元在平面排列",
            "相控阵: 相位控制波束方向",
            "阵列增益: 比单个单元增益大",
            "波束扫描: 相控阵扫描波束",
            "阵列因子: 阵列方向图函数",
            "阵列应用: 雷达、卫星通信",
        ]
    }

    /// 天线馈电
    pub fn antenna_feeding(&self) -> Vec<&'static str> {
        vec![
            "馈电方式: 天线与传输线连接",
            "同轴馈电: 同轴线连接天线",
            "平衡馈电: 平衡传输线馈电",
            "不平衡转换: 巴伦转换",
            "阻抗匹配: 消除反射",
            "馈电位置: 影响阻抗和方向图",
            "馈电网络: 阵列天线馈电网络",
            "馈线损耗: 馈线功率损耗",
        ]
    }

    /// 天线辐射
    pub fn antenna_radiation(&self) -> Vec<&'static str> {
        vec![
            "辐射机制: 天线中电流振荡",
            "近场区: 近天线处的场",
            "远场区: 远离天线的辐射场",
            "辐射强度: 单位立体角功率",
            "辐射功率: 总辐射功率",
            "辐射电阻: Rₛ = 2P/I²",
            "辐射效率: η = Rₛ/(Rₛ + Rₗ)",
            "辐射方向图: 辐射强度空间分布",
        ]
    }

    /// 天线测量
    pub fn antenna_measurements(&self) -> Vec<&'static str> {
        vec![
            "方向图测量: 测量辐射方向图",
            "增益测量: 与标准天线比较",
            "阻抗测量: 测量输入阻抗",
            "效率测量: 测量辐射效率",
            "带宽测量: 测量工作带宽",
            "极化测量: 测量极化特性",
            "近场测量: 近场扫描测量",
            "远场测量: 远场测试场测量",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "无线通信: 手机、基站天线",
            "卫星通信: 卫星天线、地面站天线",
            "雷达: 雷达天线",
            "广播电视: 发射和接收天线",
            "导航: GPS接收天线",
            "遥感: 遥感接收天线",
            "医学: 医学成像天线",
            "物联网: 物联网天线",
        ]
    }
}

impl Rule for AntennaTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("antenna_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "天线理论规则",
            &[
                ("天线基础", &self.antenna_basics()),
                ("天线参数", &self.antenna_parameters()),
                ("基本天线类型", &self.basic_antenna_types()),
                ("天线阵列", &self.antenna_arrays()),
                ("天线馈电", &self.antenna_feeding()),
                ("天线辐射", &self.antenna_radiation()),
                ("天线测量", &self.antenna_measurements()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antenna_theory_rules() {
        let rules = AntennaTheoryRules::new();
        assert_eq!(rules.metadata().name, "天线理论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.antenna_basics().is_empty());
        assert!(!rules.antenna_parameters().is_empty());
    }
}
