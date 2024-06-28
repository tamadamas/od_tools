interface Stats {
  status: {
    peasants: number; // jobs
    resilience: number;
    spy_mastery: number;
    wizard_mastery: number;
    morale: number; // dp
    wpa: number;
    race_name: string;
    created_at: string;
    realm: number;
    name: string;
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
  units: {
    spies: { amount: number; incoming: number };
    assassins: { amount: number; incoming: number };
    wizards: { amount: number; incoming: number };
    archmages: { amount: number; incoming: number };
    draftees: { amount: number; incoming: number };
    unit1: { amount: number; incoming: number };
    unit2: { amount: number; incoming: number };
    unit3: { amount: number; incoming: number };
    unit4: { amount: number; incoming: number };
  };
  buildings: {
    [name: string]: {
      amount: number,
      incoming: number,
    }
  };
  land: {
    totalLand: number,
    totalBarrenLand: number,
    totalConstructedLand: number,
    [name: string]: {
      amount: number,
      incoming: number,
    }
  };
  techs: unknown[]; 
  hero: {
    class: string,
    level: number,
    experience: number,
    next_level_xp: number,
    bonus: number,
  };
}
