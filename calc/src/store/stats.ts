import { computed, deepMap, onMount } from 'nanostores';
import { persistentAtom } from '@nanostores/persistent';
import { parseJSON } from '../utils';

type Stats = {
  status: {
    [key:string]: string|number
  },
  castle: {
    [key:string]: {
      points: number,
      rating: number,
      incoming: number,
    },
    total: number,
  },
  barracks: {
    units: {
      home: {
        [unitName: string]: number,
      }, 
      training: {
        [unitName: string]: {
          [hour: string]: number,
        } 
      }
    }
  },
  survey: {
    constructed: {
      [buildingName: string]: number
    },
    constructing: {
      [buildingName: string]: {
        [hour: string]: number
      },
      barren_land: number,
      total_land: number,
    }
  },
  land: {
    totalLand: number,
    totalBarrenLand: number,
    explored: {
      [landType: string]: {
        amount: number;
        barren: number
      }
    },
    incoming: {
      [landType: string]: {
        [hour: string]: number
      }
    },
  },
}

export const $jsonStats = persistentAtom<string>('stats', '');
export const $stats = deepMap<Stats>({});

onMount($stats, () => {
  const data = parseJSON($jsonStats.get());

  if (data) {
    $stats.set(data);
  }

  $stats.subscribe(() => {
    const json = JSON.stringify($stats.get());
    $jsonStats.set(json);
  });
});
