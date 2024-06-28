interface StatsJSON {
  status: {
    ruler_name: string;
    land: number;
    peasants: number;
    employment: number;
    networth: number;
    prestige: number;
    resilience: number;
    spy_mastery: number;
    wizard_mastery: number;
    resource_platinum: number;
    resource_food: number;
    resource_lumber: number;
    resource_mana: number;
    resource_ore: number;
    resource_gems: number;
    resource_tech: number;
    resource_boats: number;
    morale: number;
    military_draftees: number;
    military_unit1: number;
    military_unit2: number;
    military_unit3: number;
    military_unit4: number;
    military_spies: number;
    military_assassins: number;
    military_wizards: number;
    military_archmages: number;
    recently_invaded_count: number | null;
    clear_sight_accuracy: number | null;
    wpa: number;
    race_name: string;
    created_at: string;
    realm: number;
    name: string;
  };
  revelation: {
    spells: {
      dominion_id: number;
      spell_id: number;
      duration: number;
      cast_by_dominion_id: number | null;
      created_at: string | null;
      updated_at: string | null;
      spell: string;
      cast_by_dominion_name: string | null;
      cast_by_dominion_realm_number: number | null;
    }[];
    created_at: string;
  };
  castle: {
    science: {
      points: number;
      rating: number;
      incoming: number;
    };
    keep: {
      points: number;
      rating: number;
      incoming: number;
    };
    forges: {
      points: number;
      rating: number;
      incoming: number;
    };
    walls: {
      points: number;
      rating: number;
      incoming: number;
    };
    spires: {
      points: number;
      rating: number;
      rating_secondary: number;
      incoming: number;
    };
    harbor: {
      points: number;
      rating: number;
      rating_secondary: number;
      incoming: number;
    };
    total: number;
    created_at: string;
  };
  barracks: {
    units: {
      home: {
        spies: number;
        assassins: number;
        wizards: number;
        archmages: number;
        draftees: number;
        unit1: number;
        unit2: number;
        unit3: number;
        unit4: number;
      };
      returning: unknown[]; 
      training: {
        [key: string]: {
          [key: string]: number;
        };
      };
    };
    created_at: string;
  };
  survey: {
    constructed: {
      [key: string]: number;
    };
    constructing: {
      [key: string]: {
        [key: string]: number;
      };
    };
    barren_land: number;
    total_land: number;
    created_at: string;
  };
  land: {
    totalLand: number;
    totalBarrenLand: number;
    totalConstructedLand: number;
    explored: {
      [key: string]: {
        amount: number;
        percentage: number;
        barren: number;
        constructed: number;
        constructedPercentage: number;
      };
    };
    incoming: {
      [key: string]: {
        [key: string]: number;
      };
    };
    created_at: string;
  };
  vision: {
    techs: unknown[];
    created_at: string;
  };
  disclosure: {
    "0": {
      name: string;
      class: string;
      level: number;
      experience: number;
      next_level_xp: number;
      bonus: number;
    };
    created_at: string;
  };
}
