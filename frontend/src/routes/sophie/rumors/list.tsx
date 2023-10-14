import Grid from "@/components/grid";
import { SophieContext } from "@/data/sophie_data";
import { createColumnHelper } from "@tanstack/react-table";
import { useContext } from "react";
import { ItemLink } from "../utility_components/links";

export default function RumorList() {
  const sophieData = useContext(SophieContext);

  const columnHelper = createColumnHelper<(typeof sophieData.rumors)[0]>();
  const columns = [
    columnHelper.accessor("name", {
      header: "Name",
      filterFn: "includesString",
    }),
    columnHelper.accessor("type", {
      header: "Type",
      cell: (i) => <code>{i.getValue()?.substring("RUMOR_TYPE_".length)}</code>,
      filterFn: "equalsString",
    }),
    columnHelper.accessor("cost", { header: "Cost" }),
    columnHelper.accessor("deadline", { header: "Deadline" }),
    columnHelper.accessor("interval", { header: "Interval" }),
    columnHelper.accessor("redo", {
      header: "Redo",
      cell: (i) => (i.getValue() ? "✅" : "❌"),
    }),
    columnHelper.accessor("priority", { header: "Priority" }),
    columnHelper.accessor("probability", { header: "Probability" }),
    columnHelper.accessor("register", {
      header: "Register",
      cell: (i) => (i.getValue() ? "✅" : "❌"),
    }),
    columnHelper.accessor("count", { header: "Count" }),
    columnHelper.accessor("fieldmap_tag", {
      header: "Fieldmap",
      cell: (i) => <code>{i.getValue()?.substring("FIELDMAP_".length)}</code>,
    }),
    columnHelper.accessor("monster_tag", {
      header: "Monster",
      cell: (i) => <code>{i.getValue()?.substring("MONSTER_".length)}</code>,
    }),
    columnHelper.accessor("item_tag", {
      header: "Item",
      cell: (i) =>
        i.getValue() && (
          <ItemLink
            item={sophieData.items.find((v) => v.tag == i.getValue())!}
          />
        ),
    }),
    columnHelper.accessor("ev_tag", {
      header: "Event",
      cell: (i) => <code>{i.getValue()?.substring("EVENT_".length)}</code>,
    }),
    columnHelper.accessor("ev_begin", {
      header: "Begin Event",
      cell: (i) => <code>{i.getValue()?.substring("EVENT_".length)}</code>,
    }),
    columnHelper.accessor("ev_end", {
      header: "Begin Event",
      cell: (i) => <code>{i.getValue()?.substring("EVENT_".length)}</code>,
    }),
  ];

  return (
    <>
      <h1>Rumors</h1>
      <Grid data={sophieData.rumors} columns={columns} />
    </>
  );
}
