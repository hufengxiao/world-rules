//! 数独规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 数独难度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SudokuDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl SudokuDifficulty {
    pub fn name(&self) -> &'static str {
        match self {
            SudokuDifficulty::Easy => "简单",
            SudokuDifficulty::Medium => "中等",
            SudokuDifficulty::Hard => "困难",
            SudokuDifficulty::Expert => "专家",
        }
    }

    /// 提示数字数量范围
    pub fn clue_range(&self) -> (u8, u8) {
        match self {
            SudokuDifficulty::Easy => (36, 45),
            SudokuDifficulty::Medium => (30, 35),
            SudokuDifficulty::Hard => (25, 29),
            SudokuDifficulty::Expert => (17, 24),
        }
    }
}

/// 数独规则
pub struct SudokuRules {
    metadata: RuleMetadata,
}

impl SudokuRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "数独规则",
                "标准数独游戏规则"
            )
            .with_origin("日本")
            .with_tags(vec!["游戏".into(), "益智".into(), "数独".into()]),
        }
    }

    /// 网格大小
    pub fn grid_size(&self) -> u8 {
        9
    }

    /// 宫格大小
    pub fn box_size(&self) -> (u8, u8) {
        (3, 3)
    }

    /// 数字范围
    pub fn number_range(&self) -> std::ops::RangeInclusive<u8> {
        1..=9
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "9×9网格分为9个3×3小宫格",
            "每行必须包含1-9各一个",
            "每列必须包含1-9各一个",
            "每个3×3宫格必须包含1-9各一个",
            "每个格子只能填一个数字",
        ]
    }

    /// 解题技巧
    pub fn solving_techniques(&self) -> Vec<&'static str> {
        vec![
            "唯一候选数: 格子只有一个可能的数字",
            "唯余法: 行/列/宫中某数字只能在一个位置",
            "摒除法: 排除不可能的候选数",
            "数对法: 两个格子只能填两个数字",
            "三链数: 三个格子共享三个候选数",
            "X-Wing: 利用行列约束排除候选数",
        ]
    }

    /// 变体规则
    pub fn variants(&self) -> Vec<&'static str> {
        vec![
            "标准数独: 经典9×9",
            "迷你数独: 6×6或4×4",
            "对角线数独: 对角线也必须1-9",
            "杀手数独: 宫格内数字之和约束",
            "不规则数独: 宫格形状不规则",
        ]
    }

    /// 验证数独是否合法
    pub fn is_valid(&self, grid: &[[Option<u8>; 9]; 9]) -> bool {
        // 检查行
        for row in grid.iter() {
            if !self.is_valid_line(row) {
                return false;
            }
        }

        // 检查列
        for col in 0..9 {
            let column: Vec<Option<u8>> = (0..9).map(|r| grid[r][col]).collect();
            if !self.is_valid_line(&column) {
                return false;
            }
        }

        // 检查宫格
        for box_row in 0..3 {
            for box_col in 0..3 {
                let mut box_values = Vec::new();
                for r in 0..3 {
                    for c in 0..3 {
                        box_values.push(grid[box_row * 3 + r][box_col * 3 + c]);
                    }
                }
                if !self.is_valid_line(&box_values) {
                    return false;
                }
            }
        }

        true
    }

    fn is_valid_line(&self, line: &[Option<u8>]) -> bool {
        let mut seen = [false; 10];
        for &cell in line {
            if let Some(n) = cell {
                if n == 0 || n > 9 || seen[n as usize] {
                    return false;
                }
                seen[n as usize] = true;
            }
        }
        true
    }

    /// 检查是否完成
    pub fn is_complete(&self, grid: &[[Option<u8>; 9]; 9]) -> bool {
        for row in grid.iter() {
            for cell in row.iter() {
                if cell.is_none() {
                    return false;
                }
            }
        }
        self.is_valid(grid)
    }
}

impl Default for SudokuRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SudokuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("sudoku")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【数独规则】\n\n\
            网格: {}×{}\n\
            宫格: {}×{}×{}\n\
            数字: 1-9\n\n\
            基本规则:\n{}\n\n\
            解题技巧:\n{}\n\n\
            变体规则:\n{}\n",
            self.grid_size(),
            self.grid_size(),
            3, 3, 9,
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.solving_techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.variants().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_rules() {
        let rules = SudokuRules::new();
        assert_eq!(rules.grid_size(), 9);
    }

    #[test]
    fn test_valid_grid() {
        let rules = SudokuRules::new();
        let empty_grid: [[Option<u8>; 9]; 9] = [[None; 9]; 9];
        assert!(rules.is_valid(&empty_grid));
    }
}