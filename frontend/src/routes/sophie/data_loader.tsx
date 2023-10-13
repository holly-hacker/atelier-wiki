import { SophieContext, SophieData, getSophieData } from "@/data/sophie_data";
import { useEffect, useState } from "react";
import { Outlet } from "react-router-dom";

export default function DataLoader() {
  const [sophieData, setSophieData] = useState<SophieData | null>(null);

  useEffect(() => {
    // TODO: this happens twice, why?
    getSophieData().then((data) => setSophieData(data));
  }, []);

  if (sophieData == null) {
    return <div>Loading game data...</div>;
  }

  return (
    <SophieContext.Provider value={sophieData}>
      <Outlet />
    </SophieContext.Provider>
  );
}
