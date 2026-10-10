//! 数回（Slitherlink）
//!
//! 数回是在点阵网格上画一条单一闭合回路，使带数字的格子的四条边中恰好有对应数量的边被回路经过，回路不得交叉或分叉。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SlitherlinkLoopRules,
    name: "数回（Slitherlink）",
    desc: "数回是在点阵网格上画一条单一闭合回路，使带数字的格子的四条边中恰好有对应数量的边被回路经过，回路不得交叉或分叉。",
    origin: "由日本 Nikoli 创造，又称「数回」「Fences」",
    tags: ["解谜", "逻辑", "回路", "桌面游戏"]
}

impl SlitherlinkLoopRules {
    /// 盘面构成
    pub fn board(&self) -> Vec<&'static str> {
        vec![
            "网格由点阵与格子组成",
            "部分格子内标有数字0到3",
            "数字表示该格被经过的边数",
            "空白格不限制边数",
        ]
    }

    /// 画线规则
    pub fn drawing(&self) -> Vec<&'static str> {
        vec![
            "沿格子边连接相邻的点",
            "所有线须构成单一闭合回路",
            "回路不得交叉或分叉",
            "每个点相连的线数为零或二",
        ]
    }

    /// 数字约束
    pub fn constraint(&self) -> Vec<&'static str> {
        vec![
            "带数字格四周经过的边数须相等",
            "数字为零则该格四边全无线",
            "数字为三则四边中有三边有线",
            "数字约束同时作用于相邻格",
        ]
    }

    /// 解题技巧
    pub fn solving(&self) -> Vec<&'static str> {
        vec![
            "先处理数字0与3的强约束",
            "利用点最多连两条线推理",
            "排除会造成分叉的画法",
            "从局部确定线扩展到全盘",
        ]
    }
}

impl Rule for SlitherlinkLoopRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("slitherlink")
    }

    fn explain(&self) -> String {
        format!(
            "【数回（Slitherlink）】\n{}",
            [
                format!(
                    "盘面构成：\\n{}",
                    self.board()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "画线规则：\\n{}",
                    self.drawing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "数字约束：\\n{}",
                    self.constraint()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "解题技巧：\\n{}",
                    self.solving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_slitherlinklooprules_basic() {
        let rules = SlitherlinkLoopRules::new();
        assert_eq!(rules.metadata().name, "数回（Slitherlink）");
        assert!(!rules.board().is_empty());
        assert!(!rules.drawing().is_empty());
        assert!(!rules.constraint().is_empty());
        assert!(!rules.solving().is_empty());
    }

    #[test]
    fn test_slitherlinklooprules_validation() {
        let rules = SlitherlinkLoopRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("slitherlink"));
    }

    #[test]
    fn test_slitherlinklooprules_explain() {
        let rules = SlitherlinkLoopRules::new();
        let e = rules.explain();
        assert!(e.contains("盘面构成"));
        assert!(e.contains("画线规则"));
        assert!(e.contains("数字约束"));
    }
}
