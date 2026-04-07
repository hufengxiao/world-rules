//! 魔方规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 魔方类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeType {
    /// 二阶魔方
    TwoByTwo,
    /// 三阶魔方 (标准)
    ThreeByThree,
    /// 四阶魔方
    FourByFour,
    /// 五阶魔方
    FiveByFive,
    /// 其他阶数
    Other(u8),
}

impl CubeType {
    pub fn name(&self) -> String {
        match self {
            CubeType::TwoByTwo => "二阶魔方".to_string(),
            CubeType::ThreeByThree => "三阶魔方".to_string(),
            CubeType::FourByFour => "四阶魔方".to_string(),
            CubeType::FiveByFive => "五阶魔方".to_string(),
            CubeType::Other(n) => format!("{}阶魔方", n),
        }
    }

    /// 面数
    pub fn faces(&self) -> u8 {
        6
    }

    /// 每面块数
    pub fn pieces_per_face(&self) -> u8 {
        match self {
            CubeType::TwoByTwo => 4,
            CubeType::ThreeByThree => 9,
            CubeType::FourByFour => 16,
            CubeType::FiveByFive => 25,
            CubeType::Other(n) => n * n,
        }
    }

    /// 总块数
    pub fn total_pieces(&self) -> u8 {
        self.pieces_per_face() * 6
    }
}

/// 魔方规则
pub struct RubiksCubeRules {
    metadata: RuleMetadata,
    cube_type: CubeType,
}

impl RubiksCubeRules {
    pub fn new(cube_type: CubeType) -> Self {
        Self {
            metadata: RuleMetadata::new(
                cube_type.name(),
                "魔方还原规则"
            )
            .with_origin("匈牙利")
            .with_tags(vec!["游戏".into(), "益智".into(), "魔方".into()]),
            cube_type,
        }
    }

    /// 颜色/面
    pub fn standard_colors(&self) -> Vec<&'static str> {
        vec!["白", "黄", "红", "橙", "蓝", "绿"]
    }

    /// 基本操作
    pub fn basic_moves(&self) -> Vec<&'static str> {
        vec![
            "R (Right): 右面顺时针90°",
            "L (Left): 左面顺时针90°",
            "U (Up): 上面顺时针90°",
            "D (Down): 下面顺时针90°",
            "F (Front): 前面顺时针90°",
            "B (Back): 后面顺时针90°",
            "加撇号(')表示逆时针",
            "加2表示旋转180°",
        ]
    }

    /// 还原方法
    pub fn solving_methods(&self) -> Vec<&'static str> {
        vec![
            "层先法: 逐层还原，适合初学者",
            "CFOP: Cross-F2L-OLL-PLL，速拧主流方法",
            "Roux: 桥式方法",
            "ZZ: ZZ方法",
            "盲拧: 记忆后蒙眼还原",
        ]
    }

    /// CFOP步骤详解
    pub fn cfop_steps(&self) -> Vec<&'static str> {
        vec![
            "Cross (十字): 底层棱块归位",
            "F2L (前两层): 同时还原角块和棱块",
            "OLL (顶层定向): 顶层颜色统一",
            "PLL (顶层归位): 顶层块位置正确",
        ]
    }

    /// 世界纪录 (三阶)
    pub fn world_records(&self) -> Vec<&'static str> {
        vec![
            "单次: 3.13秒 (2023年)",
            "平均: 4.09秒 (2023年)",
            "单手: 6.88秒",
            "盲拧: 12.00秒",
        ]
    }

    /// 公式记号
    pub fn notation(&self) -> Vec<&'static str> {
        vec![
            "R: 右面顺时针",
            "R': 右面逆时针",
            "R2: 右面转180°",
            "x: 整体沿R方向转",
            "y: 整体沿U方向转",
            "z: 整体沿F方向转",
        ]
    }

    /// 判断是否还原
    pub fn is_solved(&self, state: &[[[u8; 3]; 3]; 6]) -> bool {
        // 检查每个面是否颜色一致
        for face in state.iter() {
            let center = face[1][1];
            for row in face.iter() {
                for &piece in row.iter() {
                    if piece != center {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Default for RubiksCubeRules {
    fn default() -> Self {
        Self::new(CubeType::ThreeByThree)
    }
}

impl Rule for RubiksCubeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("rubiks_cube")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}规则】\n\n\
            每面块数: {}\n\
            总块数: {}\n\n\
            标准颜色: {}\n\n\
            基本操作:\n{}\n\n\
            还原方法:\n{}\n\n\
            CFOP步骤:\n{}\n\n\
            公式记号:\n{}\n",
            self.cube_type.name(),
            self.cube_type.pieces_per_face(),
            self.cube_type.total_pieces(),
            self.standard_colors().join("/"),
            self.basic_moves().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.solving_methods().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.cfop_steps().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.notation().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_rules() {
        let rules = RubiksCubeRules::new(CubeType::ThreeByThree);
        assert_eq!(rules.cube_type.pieces_per_face(), 9);
    }

    #[test]
    fn test_solved_cube() {
        let rules = RubiksCubeRules::new(CubeType::ThreeByThree);
        // 已还原状态 (每个面颜色一致)
        let solved = [
            [[0u8; 3]; 3], // 面0: 全0
            [[1u8; 3]; 3], // 面1: 全1
            [[2u8; 3]; 3], // 面2: 全2
            [[3u8; 3]; 3], // 面3: 全3
            [[4u8; 3]; 3], // 面4: 全4
            [[5u8; 3]; 3], // 面5: 全5
        ];
        assert!(rules.is_solved(&solved));
    }
}