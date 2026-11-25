/**
 * The engine every hook that runs temper uses: the one the `.temper`
 * workspace installs (`npm -C .temper install`, the fresh-clone step),
 * never a PATH guess. One constant, so the gate hooks and the telemetry
 * taps state it once (0078, 0082, 0088).
 */
export const binary = ".temper/node_modules/.bin/temper";
