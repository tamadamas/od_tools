import StatsJSON from "./types/stats-json";
import Stats from "./types/stats";

export function parseJSON(json: string): StatsJSON {
  try {
    return JSON.parse(json);
  } catch (_) {
    return null;
  }
}
const sum = ()


export function transformStats(input: StatsJSON): Stats {
  
  const output: Stats = {
    status: {
      peasants: input.status.peasants,
      resilience: input.status.resilience,
      spy_mastery: input.status.spy_mastery,
      wizard_mastery: input.status.wizard_mastery,
      morale: input.status.morale,
      wpa: parseFloat(input.status.wpa.toFixed(3)),
      race_name: input.status.race_name,
      created_at: input.status.created_at,
      realm: input.status.realm,
      name: input.status.name,
    },
    castle: input.castle,
    units: {
      spies: {
        amount: input.barracks.units.home.spies,
        incoming: Object.values(input.barracks.units.training.spies ?? {}).reduce((a, b) => a + b, 0) + input.barracks.units.home.spies,
      },
      assassins: {
        amount: input.barracks.units.home.assassins,
        incoming: input.barracks.units.home.assassins,
      },
      // ... (similar for other units, summing incoming from training)
    },
    buildings: {
      home: {
        amount: input.survey.constructed.home,
        incoming: Object.values(input.survey.constructing.home ?? {}).reduce((a, b) => a + b, 0) + input.survey.constructed.home,
      },
      // ... (similar for other buildings)
    },
    land: {
      totalLand: input.land.totalLand,
      totalBarrenLand: input.land.totalBarrenLand,
      totalConstructedLand: input.land.totalConstructedLand,
      plain: {
        amount: input.land.explored.plain.amount,
        incoming: Object.values(input.land.incoming.plain ?? {}).reduce((a, b) => a + b, 0) + input.land.explored.plain.amount,
      },
      // ... (similar for other land types)
    },
    techs: input.vision.techs,
    hero: input.disclosure["0"],
  };
  return output;
}
