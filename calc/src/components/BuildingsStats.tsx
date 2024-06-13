import { useStore } from '@nanostores/react';
import { $stats } from '../store/stats';

const BuildingsStats = () => {
  const stats = useStore($stats);
  if (!stats?.survey) { return null }

  const buildings = stats.survey;

  return (
    <div className="mb-4 rounded bg-gray-800 px-1 pb-1 pt-1 text-white shadow-md">
      <h3 className="mb-2">Buildings</h3>
      <div className="flex flex-col">
        <div className="flex px-1 py-2 text-left text-xs font-bold uppercase">
          <div className="w-1/3">Building Type</div>
          <div className="w-1/3 text-center">Constructed</div>
          <div className="w-1/3 text-center">With Incoming</div>
        </div>
        {buildins.map(buildingName => {
          const constructed = buildingsData[buildingName];
          const percentage = ((constructed / totalConstructed) * 100).toFixed(2);

          return (
            <div key={buildingName} className="flex border-b px-1 py-1 text-left text-sm">
              <div className="w-1/3">{buildingName}</div>
              <div className="w-1/3 text-center">{constructed} ({percentage}%)</div>
              <div className="w-1/3 text-center">{constructed} ({percentage}%)</div>
            </div>
          );
        })}

        <div className="flex bg-gray-900 px-1 py-1 text-left text-sm font-bold">
          <div className="w-1/3">Total</div>
          <div className="w-1/3 text-center">{totalConstructed} (100.00%)</div>
          <div className="w-1/3 text-center">{totalConstructed} (100.00%)</div>
        </div>
      </div>
    </div>
  );
};

export default BuildingsStats;
