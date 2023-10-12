import Grid from "@/components/grid";
import { Ryza3Context } from "@/data/ryza3_data";
import { createColumnHelper } from "@tanstack/react-table";
import { useContext, useState } from "react";
import { Link } from "react-router-dom";

export default function ItemKindsList() {
  const ryza3Data = useContext(Ryza3Context);
  const [kinds] = useState(() => ryza3Data.items.map((item) => item.kind_tag));
  const [uniqueKinds] = useState(() => [...new Set(kinds)]);

  const columnHelper = createColumnHelper<string>();
  const columns = [
    columnHelper.accessor((x) => x, {
      header: "Item kind",
      cell: (i) => (
        <Link to={`/ryza3/item_kinds/${i.getValue()}`}>{i.getValue()}</Link>
      ),
    }),
    columnHelper.accessor((x) => kinds.filter((c) => c === x).length, {
      header: "Count",
    }),
  ];

  return (
    <>
      <h1>Item kinds</h1>A list of all item kinds.
      <Grid data={uniqueKinds} columns={columns} />
    </>
  );
}
