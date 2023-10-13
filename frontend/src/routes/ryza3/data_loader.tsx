import { Ryza3Context, Ryza3Data, getRyza3Data } from "@/data/ryza3_data";
import { useEffect, useState } from "react";
import { Outlet } from "react-router-dom";

export default function DataLoader() {
  const [ryza3Data, setRyza3Data] = useState<Ryza3Data | null>(null);

  useEffect(() => {
    // TODO: this happens twice, why?
    getRyza3Data().then((data) => setRyza3Data(data));
  }, []);

  if (ryza3Data == null) {
    return <div>Loading game data...</div>;
  }

  return (
    <Ryza3Context.Provider value={ryza3Data}>
      <Outlet />
    </Ryza3Context.Provider>
  );
}
