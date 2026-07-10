'use strict';
// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------
const REGISTRY = Object.freeze({
    claude: Object.freeze({ installSurface: 'settings-json', writesSharedSettings: true, finishPermissionWriter: null }),
    gemini: Object.freeze({ installSurface: 'settings-json', writesSharedSettings: true, finishPermissionWriter: null }),
    antigravity: Object.freeze({ installSurface: 'settings-json', writesSharedSettings: true, finishPermissionWriter: null }),
    augment: Object.freeze({ installSurface: 'settings-json', writesSharedSettings: true, finishPermissionWriter: null }),
    qwen: Object.freeze({ installSurface: 'settings-json', writesSharedSettings: true, finishPermissionWriter: null }),
    hermes: Object.freeze({ installSurface: 'settings-json', writesSharedSettings: true, finishPermissionWriter: null }),
    codebuddy: Object.freeze({ installSurface: 'settings-json', writesSharedSettings: true, finishPermissionWriter: null }),
    opencode: Object.freeze({ installSurface: 'settings-json', writesSharedSettings: true, finishPermissionWriter: 'opencode' }),
    kilo: Object.freeze({ installSurface: 'settings-json', writesSharedSettings: false, finishPermissionWriter: 'kilo' }),
    codex: Object.freeze({ installSurface: 'codex-toml', writesSharedSettings: false, finishPermissionWriter: null }),
    copilot: Object.freeze({ installSurface: 'copilot-instructions', writesSharedSettings: false, finishPermissionWriter: null }),
    cline: Object.freeze({ installSurface: 'cline-rules', writesSharedSettings: false, finishPermissionWriter: null }),
    cursor: Object.freeze({ installSurface: 'cursor-hooks-json', writesSharedSettings: false, finishPermissionWriter: null }),
    windsurf: Object.freeze({ installSurface: 'profile-marker-only', writesSharedSettings: false, finishPermissionWriter: null }),
    trae: Object.freeze({ installSurface: 'profile-marker-only', writesSharedSettings: false, finishPermissionWriter: null }),
});
// ---------------------------------------------------------------------------
// Exports
// ---------------------------------------------------------------------------
/** The complete set of 15 supported runtimes for config-adapter dispatch. */
const ALLOWED_CONFIG_RUNTIMES = new Set(Object.keys(REGISTRY));
/** All valid installSurface values. */
const INSTALL_SURFACES = Object.freeze([
    'settings-json',
    'codex-toml',
    'copilot-instructions',
    'cline-rules',
    'cursor-hooks-json',
    'profile-marker-only',
]);
/**
 * Resolve the config adapter intent for a given runtime.
 *
 * Returns a fresh object each call so callers cannot poison the registry by
 * mutating the returned value.
 *
 * @throws {TypeError} if runtime is not a known supported runtime.
 */
function resolveRuntimeConfigIntent(runtime) {
    if (!Object.hasOwn(REGISTRY, runtime)) {
        throw new TypeError(`Unknown runtime for config adapter: ${runtime}`);
    }
    const entry = REGISTRY[runtime];
    return {
        runtime,
        installSurface: entry.installSurface,
        writesSharedSettings: entry.writesSharedSettings,
        finishPermissionWriter: entry.finishPermissionWriter,
    };
}
module.exports = { resolveRuntimeConfigIntent, ALLOWED_CONFIG_RUNTIMES, INSTALL_SURFACES };
