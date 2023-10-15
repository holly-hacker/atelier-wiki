import Grid from "@/components/grid";
import { SophieContext } from "@/data/sophie_data";
import { createColumnHelper } from "@tanstack/react-table";
import { useContext } from "react";

export default function DollList() {
  const sophieData = useContext(SophieContext);
  const dolls = sophieData.dolls.filter(
    (v) => v.chara_base_tag != "CHARA_BASE_NONE",
  );

  const columnHelper = createColumnHelper<(typeof sophieData.dolls)[0]>();
  const columns = [
    columnHelper.accessor("no", {
      header: "Number",
    }),
    columnHelper.accessor("name", { header: "Name" }),
    columnHelper.accessor("cute_min", {
      header: "Cute Min",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("cute_max", {
      header: "Cute Max",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("wise_min", {
      header: "Wise Min",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("wise_max", {
      header: "Wise Max",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("brave_min", {
      header: "Brave Min",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("brave_max", {
      header: "Brave Max",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("fool_min", {
      header: "Fool Min",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("fool_max", {
      header: "Fool Max",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("doll_hp", { header: "HP" }),
    columnHelper.accessor("doll_mp", { header: "MP" }),
    columnHelper.accessor("doll_lp", { header: "LP" }),
    columnHelper.accessor("doll_atk", { header: "Atk." }),
    columnHelper.accessor("doll_def", { header: "Def." }),
    columnHelper.accessor("doll_spd", { header: "Spd." }),
    columnHelper.accessor("doll_dmg_min", { header: "Dmg Min" }),
    columnHelper.accessor("doll_dmg_max", { header: "Dmg Max" }),
    columnHelper.accessor("dlc_tag", {
      header: "DLC",
      cell: (i) => <code>{i.getValue()?.substring("DLC_".length)}</code>,
      filterFn: "equalsString",
    }),
  ];

  return (
    <>
      <h1>Doll making</h1>
      <Grid data={dolls} columns={columns} />
    </>
  );
}

function NonZeroNumber({ value }: { value: number }) {
  if (value != 0) {
    return <>{value}</>;
  } else {
    return <></>;
  }
}
