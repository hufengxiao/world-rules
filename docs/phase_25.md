# Phase 25: E8 残疾人运动扩充

## 概述

Phase 25 为 world-rules 项目添加了 25 种残疾人运动规则，涵盖残奥会的主要项目。

## 完成内容

### 25-01: 10种残奥规则 ✅

新增的残疾人运动规则：

1. **残疾人自行车规则 (ParaCyclingRules)** - `para_cycling.rs`
   - 运动分级：C级（截肢）、H级（手自行车）、T级（三轮车）、B级（视力残疾）
   - 比赛项目：场地赛、公路赛、手自行车、三轮车、双人自行车
   - 装备要求：自行车、手自行车、三轮车、双人自行车、头盔
   - 适应性规则：车辆改装、假肢固定、领骑员

2. **残疾人射箭规则 (ParaArcheryRules)** - `para_archery.rs`
   - 运动分级：W1级（严重肢体残疾）、W2级、ST级、VI级（视力残疾）
   - 比赛项目：反曲弓70米、复合弓50米、W1复合弓
   - 计分规则：10环制，72箭排名赛
   - 适应性规则：身体支撑、触觉瞄准辅助、轮椅固定

3. **残疾人射击规则 (ParaShootingRules)** - `para_shooting.rs`
   - 运动分级：SH1级（上肢功能正常）、SH2级（需支架）
   - 比赛项目：10米气手枪、气步枪、50米步枪三姿
   - 安全规则：枪口指向、扳机安全、装弹检查
   - 适应性规则：射击支架、特殊座椅、视觉辅助

4. **残疾人赛艇规则 (ParaRowingRules)** - `para_rowing.rs`
   - 运动分级：PR1级（躯干功能丧失）、PR2级、PR3级
   - 比赛项目：PR1单人双桨、PR2双人双桨、PR3混合四人双桨
   - 装备要求：赛艇、固定座椅、滑座
   - 适应性规则：固定座椅+背部支撑、手绑装置、足部固定

5. **残疾人马术规则 (ParaEquestrianRules)** - `para_equestrian.rs`
   - 运动分级：Grade I-V（残疾程度递减）
   - 比赛项目：个人盛装舞步、团体赛、自由式舞步
   - 评分规则：裁判评分0-10分
   - 适应性规则：适应性马鞍、辅助扶手、固定装置

6. **残疾人乒乓球规则 (ParaTableTennisRules)** - `para_table_tennis.rs`
   - 运动分级：TT1-TT5（轮椅）、TT6-TT10（站立）、TT11（智力残疾）
   - 比赛项目：单打、团体、混合团体
   - 发球规则：轮椅级可从后方发球、抛球困难可不抛球
   - 适应性规则：球台高度适配、轮椅固定、假肢辅助

7. **残疾人力量举规则 (ParaPowerliftingRules)** - `para_powerlifting.rs`
   - 运动分级：体重分级（男女各10级）
   - 比赛项目：卧推单项、残奥会男女20个小项
   - 技术规则：起始信号→下放→暂停→推起→结束信号
   - 适应性规则：卧推台改装、假肢固定、绑带辅助

8. **轮椅橄榄球规则 (WheelchairRugbyRules)** - `wheelchair_rugby.rs`
   - 运动分级：0.5-3.5分分级系统，总分限制8.0分
   - 比赛项目：残奥会单项、世界锦标赛
   - 比赛规则：场地室内篮球场，得分持球进入目标区
   - 装备要求：攻击轮椅、防守轮椅、标准排球

9. **轮椅击剑规则 (WheelchairFencingRules)** - `wheelchair_fencing.rs`
   - 运动分级：A级、B级、C级（功能递减）
   - 比赛项目：花剑、重剑、佩剑各级别
   - 计分规则：花剑躯干有效、重剑全身有效、佩剑上半身有效
   - 装备要求：FIE认证剑、固定轮椅框架、击剑服

10. **残疾人冰球规则 (ParaIceHockeyRules)** - `para_ice_hockey.rs`
    - 运动分级：最低下肢残疾要求
    - 比赛项目：残奥会混合团体
    - 比赛规则：雪橇移动、双端球杆挑球和推动
    - 装备要求：双刃雪橇、T形球杆、标准冰球护具

### 25-02: 10种特殊运动规则 ✅

1. **残疾人高山滑雪规则 (ParaAlpineSkiingRules)** - `para_alpine_skiing.rs`
   - 运动分级：视力残疾、站姿、坐姿三类
   - 比赛项目：滑降、超级大回转、大回转、回转
   - 装备要求：滑雪板、坐式滑雪器（坐姿）、头盔
   - 适应性规则：引导员（视力残疾）、假肢固定（站姿）

2. **残疾人越野滑雪规则 (ParaCrossCountrySkiingRules)** - `para_cross_country_skiing.rs`
   - 运动分级：视力残疾、站姿、坐姿
   - 比赛项目：短距离、中距离、长距离、接力
   - 技术规则：传统式、自由式
   - 适应性规则：坐式滑雪器、引导员系统

3. **残疾人盲人柔道规则 (ParaJudoRules)** - `para_judo.rs`
   - 运动分级：J1级（全盲）、J2级（低视力）
   - 比赛项目：残奥会、世界锦标赛各级别
   - 技术规则：一本、握把调整、声音信号
   - 适应性规则：触觉标志、声音引导

4. **坐式排球规则 (SittingVolleyballRules)** - `sitting_volleyball.rs`
   - 运动分级：VS1级、VS2级
   - 场地规则：10×6米场地，网高1.15米（男）1.05米（女）
   - 技术规则：臀部必须接触地面、允许身体任何部位触球
   - 比赛制：五局三胜，每局25分

5. **残疾人冬季两项规则 (ParaBiathlonRules)** - `para_biathlon.rs`
   - 运动分级：视力残疾、站姿、坐姿
   - 比赛项目：短距离、中距离、接力
   - 射击规则：靶标大小适应、罚圈规则
   - 适应性规则：电子瞄准辅助（视力残疾）

6. **残疾人跆拳道规则 (ParaTaekwondoRules)** - `para_taekwondo.rs`
   - 运动分级：K41-K44级（肢体残疾程度递减）
   - 比赛项目：残奥会、世界锦标赛
   - 技术规则：踢击得分、禁止头部攻击
   - 适应性规则：假肢规定、防护装备

7. **残疾人皮划艇规则 (ParaCanoeRules)** - `para_canoe.rs`
   - 运动分级：KL1-KL3（皮艇）、VL1-VL3（划艇）
   - 比赛项目：200米竞速、残奥会
   - 装备要求：皮艇、划艇、救生衣
   - 适应性规则：座椅固定、单臂划桨

8. **残疾人舞蹈运动规则 (ParaDanceSportRules)** - `para_dance_sport.rs`
   - 运动分级：轮椅组、站立组
   - 比赛项目：标准舞、拉丁舞
   - 评分规则：技术、音乐表现、艺术性
   - 适应性规则：轮椅舞蹈技术、组合规定

9. **残疾人羽毛球规则 (ParaBadmintonRules)** - `para_badminton.rs`
   - 运动分级：WH1-WH2（轮椅）、SL3-SL4（站立）、SU5（上肢残疾）
   - 场地规则：轮椅半场缩小、网高调整
   - 技术规则：21分制、发球规则适应
   - 适应性规则：轮椅固定、假肢规定

10. **残疾人帆船规则 (ParaSailingRules)** - `para_sailing.rs`
    - 运动分级：1级-3级（残疾程度递减）
    - 比赛项目：单人帆船、双人帆船、三人帆船
    - 装备要求：帆船、救生衣、通讯设备
    - 适应性规则：座椅改装、操纵系统适应

### 25-03: 5种适应性规则 ✅

1. **适应性游泳规则 (AdaptiveSwimmingRules)** - `adaptive_swimming.rs`
   - 运动分级：S级（自由泳/仰泳/蝶泳）、SB级（蛙泳）、SM级（混合泳）
   - 起跳适应性：水中出发、平台出发、辅助出发、信号适应
   - 转身规则：视力残疾敲击提醒、截肢单手触壁
   - 装备适应性：假肢必须取下、禁止浮力辅助设备

2. **适应性田径规则 (AdaptiveAthleticsRules)** - `adaptive_athletics.rs`
   - 运动分级：T级（径赛）、F级（田赛）
   - 轮椅竞速规则：轮椅规格、轮径限制、禁止电子驱动
   - 义肢赛跑规则：跑步专用义肢、长度限制、碳纤维弹性义肢
   - 投掷适应性：坐姿投掷、假肢固定、引导辅助

3. **适应性划船规则 (AdaptiveRowingRules)** - `adaptive_rowing.rs`
   - 运动分级：PR1级（手臂）、PR2级（躯干）、PR3级（全身）
   - 船艇适应性：固定座椅系统、支撑性座椅、脚踏板改装
   - 划桨适应性：固定座位划桨、单手划桨、手套绑带
   - 安全规则：救生衣必须佩戴、翻船程序演练

4. **适应性雪橇规则 (AdaptiveSledRules)** - `adaptive_sled.rs`
   - 运动分级：雪橇冰球、坐姿滑雪（LW10-LW12）、单板滑雪
   - 雪橇冰球规则：双冰刀设计、双头球杆、移动规则
   - 坐姿滑雪规则：坐式滑雪器、悬吊系统、平衡辅助
   - 视力残疾滑雪：引导员、通讯系统、声音信号

5. **适应性球类规则 (AdaptiveBallGamesRules)** - `adaptive_ball_games.rs`
   - 运动分级：轮椅网球、盲人足球、轮椅篮球、坐式排球、盲人门球
   - 轮椅网球规则：两跳规则、轮椅规格、移动规则
   - 盲人足球规则：发声球、眼罩遮蔽、引导员
   - 盲人门球规则：发声球、眼罩、静音规则

## 测试覆盖

每个规则都包含完整的单元测试：

### 25-01 测试（50个）
- 残疾人自行车：5个测试（基本、分级、项目、装备、适应性）
- 残疾人射箭：5个测试
- 残试射击：5个测试
- 残试赛艇：5个测试
- 残试马术：5个测试
- 残试乒乓球：5个测试
- 残试力量举：5个测试
- 轮椅橄榄球：5个测试
- 轮椅击剑：5个测试
- 残试冰球：5个测试

### 25-02 测试（40个）
- 残试高山滑雪：4个测试
- 残试越野滑雪：4个测试
- 残试盲人柔道：4个测试
- 坐式排球：4个测试
- 残试冬季两项：4个测试
- 残试跆拳道：4个测试
- 残试皮划艇：4个测试
- 残试舞蹈运动：4个测试
- 残试羽毛球：4个测试
- 残试帆船：4个测试

### 25-03 测试（25个）
- 适应性游泳：5个测试
- 适应性田径：5个测试
- 适应性划船：5个测试
- 适应性雪橇：5个测试
- 适应性球类：5个测试

**测试文件**：`tests/phase_25_rules.rs`
**测试总数**：115个测试用例

## 代码统计

- 新增文件：25个规则文件（10+10+5）+ 1个测试文件
- 新增代码：
  - 规则文件：约 15,000行
  - 测试文件：约 1,245行
- 总代码量：约 16,245行

## API 导出

所有规则已正确导出到 `src/rules/sports/mod.rs`：

```rust
// 25-01 导出
pub use para_cycling::ParaCyclingRules;
pub use para_archery::ParaArcheryRules;
pub use para_shooting::ParaShootingRules;
pub use para_rowing::ParaRowingRules;
pub use para_equestrian::ParaEquestrianRules;
pub use para_table_tennis::ParaTableTennisRules;
pub use para_powerlifting::ParaPowerliftingRules;
pub use wheelchair_rugby::WheelchairRugbyRules;
pub use wheelchair_fencing::WheelchairFencingRules;
pub use para_ice_hockey::ParaIceHockeyRules;

// 25-02 导出
pub use para_alpine_skiing::ParaAlpineSkiingRules;
pub use para_cross_country_skiing::ParaCrossCountrySkiingRules;
pub use para_judo::ParaJudoRules;
pub use sitting_volleyball::SittingVolleyballRules;
pub use para_biathlon::ParaBiathlonRules;
pub use para_taekwondo::ParaTaekwondoRules;
pub use para_canoe::ParaCanoeRules;
pub use para_dance_sport::ParaDanceSportRules;
pub use para_badminton::ParaBadmintonRules;
pub use para_sailing::ParaSailingRules;

// 25-03 导出
pub use adaptive_athletics::AdaptiveAthleticsRules;
pub use adaptive_ball_games::AdaptiveBallGamesRules;
pub use adaptive_rowing::AdaptiveRowingRules;
pub use adaptive_sled::AdaptiveSledRules;
pub use adaptive_swimming::AdaptiveSwimmingRules;
```

---

*完成时间：2026-07-11*
*Phase：25 of 55*
*Milestone：v2.2 规则扩充*