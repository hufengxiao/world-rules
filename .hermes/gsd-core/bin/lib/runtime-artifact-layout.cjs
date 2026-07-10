'use strict';
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
/**
 * Runtime artifact layout module — resolves the artifact directory shapes
 * (commands, agents, skills) for each supported runtime.
 *
 * grok is intentionally absent: it is in runtime-homes.cjs but not wired
 * here. The TypeError on unknown runtime is the loud-fail signal that a
 * runtime was added to the homes list without a layout entry.
 *
 * ADR-457 build-at-publish: the hand-written bin/lib/runtime-artifact-layout.cjs
 * collapsed to a TypeScript source of truth. Behaviour is preserved byte-for-behaviour
 * from the prior hand-written .cjs; only types are added.
 */
const node_path_1 = __importDefault(require("node:path"));
const node_fs_1 = __importDefault(require("node:fs"));
// eslint-disable-next-line @typescript-eslint/no-require-imports
const installProfiles = require("./install-profiles.cjs");
const { stageSkillsForProfile, stageAgentsForProfile, stageSkillsForRuntimeAsSkills, stageCommandsForRuntimeFlat, } = installProfiles;
// In .cts (CommonJS output) files, `require` is available as a global.
const _require = require;
/**
 * Load bin/install.js exports in a test-safe way.
 * Sets GSD_TEST_MODE only for the duration of the require() call and only if
 * it was not already set, restoring the original value in a finally block so
 * the module-level environment is never permanently mutated.
 */
function loadInstallExports() {
    const savedTestMode = process.env['GSD_TEST_MODE'];
    if (savedTestMode === undefined)
        process.env['GSD_TEST_MODE'] = '1';
    try {
        return _require('../../../bin/install.js');
    }
    finally {
        if (savedTestMode === undefined)
            delete process.env['GSD_TEST_MODE'];
        else
            process.env['GSD_TEST_MODE'] = savedTestMode;
    }
}
/** Cache after first successful load. */
let _installExports = null;
function getInstallExports() {
    if (!_installExports)
        _installExports = loadInstallExports();
    return _installExports;
}
// ---------------------------------------------------------------------------
// Source root finders
// ---------------------------------------------------------------------------
/**
 * Locate the GSD commands/gsd source directory.
 *
 * Resolution order:
 * 1. If runtimeConfigDir provided, check <runtimeConfigDir>/.gsd-source marker.
 * 2. Walk up from __dirname using path.dirname (no literal .. segments).
 * 3. Throw a descriptive error if neither succeeds.
 */
function findInstallSourceRoot(runtimeConfigDir) {
    // Step 1: marker check
    if (runtimeConfigDir) {
        const markerPath = node_path_1.default.join(runtimeConfigDir, '.gsd-source');
        if (node_fs_1.default.existsSync(markerPath)) {
            try {
                const src = node_fs_1.default.readFileSync(markerPath, 'utf8').trim();
                if (src && node_fs_1.default.existsSync(src))
                    return src;
            }
            catch { /* fall through */ }
        }
    }
    // Step 2: walk up from __dirname
    let dir = __dirname;
    for (let i = 0; i < 6; i++) {
        const candidate = node_path_1.default.join(dir, 'commands', 'gsd');
        if (node_fs_1.default.existsSync(candidate))
            return candidate;
        const parent = node_path_1.default.dirname(dir);
        if (parent === dir)
            break;
        dir = parent;
    }
    throw new Error(`findInstallSourceRoot: could not locate commands/gsd from ${__dirname}`);
}
/**
 * Locate the GSD agents source directory.
 *
 * Resolution order:
 * 1. If runtimeConfigDir provided, check <runtimeConfigDir>/.gsd-source marker.
 * 2. Walk up from __dirname using path.dirname (no literal .. segments).
 * 3. Throw a descriptive error if neither succeeds.
 */
function findAgentsSourceRoot(runtimeConfigDir) {
    // Step 1: marker check
    if (runtimeConfigDir) {
        const markerPath = node_path_1.default.join(runtimeConfigDir, '.gsd-source');
        if (node_fs_1.default.existsSync(markerPath)) {
            try {
                const src = node_fs_1.default.readFileSync(markerPath, 'utf8').trim();
                if (src && node_fs_1.default.existsSync(src)) {
                    // Marker points to commands/gsd; agents/ is a sibling of commands/
                    const agentsCandidate = node_path_1.default.resolve(node_path_1.default.dirname(src), '..', 'agents');
                    if (node_fs_1.default.existsSync(agentsCandidate))
                        return agentsCandidate;
                }
            }
            catch { /* fall through */ }
        }
    }
    // Step 2: walk up from __dirname
    let dir = __dirname;
    for (let i = 0; i < 6; i++) {
        const candidate = node_path_1.default.join(dir, 'agents');
        if (node_fs_1.default.existsSync(candidate))
            return candidate;
        const parent = node_path_1.default.dirname(dir);
        if (parent === dir)
            break;
        dir = parent;
    }
    throw new Error(`findAgentsSourceRoot: could not locate agents/ from ${__dirname}`);
}
// ---------------------------------------------------------------------------
// Allowlisted runtimes
// ---------------------------------------------------------------------------
const ALLOWED_RUNTIMES = new Set([
    'claude', 'cursor', 'gemini', 'codex', 'copilot', 'antigravity',
    'windsurf', 'augment', 'trae', 'qwen', 'hermes', 'codebuddy',
    'cline', 'opencode', 'kilo',
]);
// ---------------------------------------------------------------------------
// Layout table builders
// ---------------------------------------------------------------------------
function commandsKind(destSubpath, prefix, configDir) {
    return {
        kind: 'commands',
        destSubpath,
        prefix,
        stage: (resolved) => stageSkillsForProfile(findInstallSourceRoot(configDir), resolved),
    };
}
function agentsKind(destSubpath, prefix, configDir) {
    return {
        kind: 'agents',
        destSubpath,
        prefix,
        stage: (resolved) => stageAgentsForProfile(findAgentsSourceRoot(configDir), resolved),
    };
}
/**
 * Build a skills kind descriptor.
 *
 * @param destSubpath
 * @param prefix
 * @param converterName  name of converter function in bin/install.js exports
 * @param runtime        canonical runtime ID (gates Hermes/Qwen branding in converter)
 * @param configDir      runtime config dir (for .gsd-source marker resolution)
 */
function skillsKind(destSubpath, prefix, converterName, runtime, configDir) {
    return {
        kind: 'skills',
        destSubpath,
        prefix,
        stage: (resolved) => {
            const installExports = getInstallExports();
            const realConverter = installExports[converterName];
            // Compute cmdNames once per stage call for performance (#3583).
            // Extra args are ignored by converters that don't need runtime/cmdNames.
            const cmdNames = installExports.readGsdCommandNames();
            const wrappedConverter = (content, skillName) => realConverter(content, skillName, runtime, cmdNames);
            return stageSkillsForRuntimeAsSkills(findInstallSourceRoot(configDir), resolved, wrappedConverter, prefix);
        },
    };
}
/**
 * Build a converted-commands kind descriptor for runtimes that use a flat
 * commands directory with per-file conversion (e.g. Cursor 1.6 slash commands).
 *
 * Unlike `commandsKind` (which passes raw source files through), this kind
 * applies `converterName` from bin/install.js exports to each file during
 * staging, writing flat `${prefix}${stem}.md` files to the staged directory.
 *
 * The staged files are then written by `_copyStaged` (commands branch) which
 * handles prefix logic via the existing layout machinery.
 *
 * @param destSubpath   destination subpath within configDir (e.g. 'commands')
 * @param prefix        filename prefix, e.g. 'gsd-'
 * @param converterName name of converter function in bin/install.js exports
 * @param configDir     runtime config dir (for .gsd-source marker resolution)
 */
function convertedCommandsKind(destSubpath, prefix, converterName, configDir) {
    return {
        kind: 'commands',
        destSubpath,
        prefix,
        stage: (resolved) => {
            const installExports = getInstallExports();
            const converter = installExports[converterName];
            return stageCommandsForRuntimeFlat(findInstallSourceRoot(configDir), resolved, converter, prefix);
        },
    };
}
// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------
/**
 * Resolve the artifact layout for a given runtime and config directory.
 */
function resolveRuntimeArtifactLayout(runtime, configDir, scope = 'global') {
    if (typeof configDir !== 'string' || configDir === '') {
        throw new TypeError('configDir must be a non-empty string');
    }
    if (scope !== 'local' && scope !== 'global') {
        throw new TypeError('scope must be "local" or "global"');
    }
    if (!ALLOWED_RUNTIMES.has(runtime)) {
        throw new TypeError(`Unknown runtime: '${runtime}' — add to runtime-artifact-layout.cjs table`);
    }
    let kinds;
    switch (runtime) {
        case 'claude':
            if (scope === 'local') {
                kinds = [
                    commandsKind('commands/gsd', 'gsd-', configDir),
                    agentsKind('agents', 'gsd-', configDir),
                ];
            }
            else {
                kinds = [skillsKind('skills', 'gsd-', 'convertClaudeCommandToClaudeSkill', 'claude', configDir)];
            }
            break;
        case 'cursor':
            // Cursor 1.6+ supports two artifact surfaces:
            //   1. skills/gsd-<name>/SKILL.md  — rich skills with frontmatter + adapter header
            //   2. commands/gsd-<name>.md      — plain markdown slash commands (no frontmatter)
            //      accessed via '/' in the Agent input (#785)
            kinds = [
                skillsKind('skills', 'gsd-', 'convertClaudeCommandToCursorSkill', 'cursor', configDir),
                convertedCommandsKind('commands', 'gsd-', 'convertClaudeCommandToCursorCommand', configDir),
            ];
            break;
        case 'gemini':
            kinds = [commandsKind('commands/gsd', 'gsd-', configDir)];
            break;
        case 'codex':
            kinds = [skillsKind('skills', 'gsd-', 'convertClaudeCommandToCodexSkill', 'codex', configDir)];
            break;
        case 'copilot':
            kinds = [skillsKind('skills', 'gsd-', 'convertClaudeCommandToCopilotSkill', 'copilot', configDir)];
            break;
        case 'antigravity':
            kinds = [skillsKind('skills', 'gsd-', 'convertClaudeCommandToAntigravitySkill', 'antigravity', configDir)];
            break;
        case 'windsurf':
            kinds = [skillsKind('skills', 'gsd-', 'convertClaudeCommandToWindsurfSkill', 'windsurf', configDir)];
            break;
        case 'augment':
            kinds = [
                commandsKind('commands', 'gsd-', configDir),
                skillsKind('skills', 'gsd-', 'convertClaudeCommandToAugmentSkill', 'augment', configDir),
            ];
            break;
        case 'trae':
            kinds = [skillsKind('skills', 'gsd-', 'convertClaudeCommandToTraeSkill', 'trae', configDir)];
            break;
        case 'qwen':
            kinds = [skillsKind('skills', 'gsd-', 'convertClaudeCommandToClaudeSkill', 'qwen', configDir)];
            break;
        case 'hermes':
            kinds = [skillsKind('skills/gsd', '', 'convertClaudeCommandToClaudeSkill', 'hermes', configDir)];
            break;
        case 'codebuddy':
            // CodeBuddy (Tencent) reads two user-level surfaces (codebuddy.ai/docs/cli):
            //   1. commands/gsd-<name>.md      — slash commands shown in the '/' menu (#789)
            //   2. skills/gsd-<name>/SKILL.md  — model-invocable skills, emitted with
            //      user-invocable:false so they stay OUT of '/' (the commands surface is
            //      the sole '/' entry point) — avoids a duplicated /gsd-* per workflow.
            // Subagents (~/.codebuddy/agents/) are already emitted by the generic agents
            // block in bin/install.js; MCP is excluded (gsd ships no MCP server).
            kinds = [
                convertedCommandsKind('commands', 'gsd-', 'convertClaudeCommandToCodebuddyCommand', configDir),
                skillsKind('skills', 'gsd-', 'convertClaudeCommandToCodebuddySkill', 'codebuddy', configDir),
            ];
            break;
        case 'cline':
            kinds = scope === 'global' ? [skillsKind('skills', 'gsd-', 'convertClaudeCommandToClineSkill', 'cline', configDir)] : [];
            break;
        case 'opencode':
            // OpenCode reads flat slash commands from command/ and on-demand skills
            // from skills/<name>/SKILL.md (https://opencode.ai/docs/skills). Emit both.
            kinds = [
                commandsKind('command', 'gsd-', configDir),
                skillsKind('skills', 'gsd-', 'convertClaudeCommandToOpencodeSkill', 'opencode', configDir),
            ];
            break;
        case 'kilo':
            // Kilo derives from OpenCode and shares the skills/<name>/SKILL.md layout
            // (https://kilo.ai/docs/customize/skills). Emit flat commands + skills.
            kinds = [
                commandsKind('command', 'gsd-', configDir),
                skillsKind('skills', 'gsd-', 'convertClaudeCommandToKiloSkill', 'kilo', configDir),
            ];
            break;
        default:
            throw new TypeError(`Unknown runtime: '${runtime}' — add to runtime-artifact-layout.cjs table`);
    }
    return { runtime, configDir, scope, kinds };
}
module.exports = { resolveRuntimeArtifactLayout, findInstallSourceRoot, getInstallExports };
