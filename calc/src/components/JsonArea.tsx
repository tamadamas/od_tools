import { useStore } from '@nanostores/react';
import { $stats } from '../store/stats';
import { useState } from 'react';
import { parseJSON } from '../utils'

function JsonArea() {
  const stats = useStore($stats);
  const name: string = stats?.status?.name || 'dominion name'
  const [inputVisible, setVisibility] = useState(!stats?.status);

  const setStats = (json: string) => {
    if (!json) {
      $stats.set(null);
      setVisibility(true);
    }

    const parsedData = parseJSON(json);
    if (parsedData) {
      setVisibility(false);
      $stats.set(parsedData);
    }
  }

  return (
    <div className="grid grid-cols-2 gap-2">
      <div className="rounded-md bg-gray-800 p-2 text-center
        align-middle font-bold capitalize text-indigo-400 shadow-md">
        {name}
      </div>
      {inputVisible && <textarea className="h-10 rounded-md bg-gray-800 p-2 text-white"
        placeholder="Put ops here" spellCheck="false"
        onChange={(e) => setStats(e.target.value)}
      />}

      {!inputVisible && <button className="h-10 rounded-md bg-sky-900 text-white
          shadow hover:bg-sky-700" onClick={() => setStats('')}
      >
        Clear
      </button>}
    </div>
  )
}

export default JsonArea;
