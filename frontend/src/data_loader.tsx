import { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import { Ryza3Context, Ryza3Data, getRyza3Data } from "./data/ryza3_data";

const Loading = function () {
  return <div>Loading game data...</div>;
};

export default function DataLoader({
  children,
}: {
  children: React.ReactNode;
}) {
  const location = useLocation();
  const path = location.pathname;

  const [ryza3Data, setRyza3Data] = useState<Ryza3Data | null>(null);

  useEffect(() => {
    getRyza3Data().then((data) => {
      setRyza3Data(data);
    });
  }, []);

  if (path.startsWith("/ryza3")) {
    if (ryza3Data == null) {
      return <Loading />;
    }

    return (
      <Ryza3Context.Provider value={ryza3Data}>
        {children}
      </Ryza3Context.Provider>
    );
  }

  return children;
}
