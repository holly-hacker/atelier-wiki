import Grid from "@/components/grid";
import items from "@/data/ryza3/items.json";
import { createColumnHelper } from "@tanstack/react-table";
import { useState } from "react";
import { Link } from "react-router-dom";

export default function ItemUseTagsList() {
  const [kinds] = useState(() => items.map((item) => item.use_tag));
  const [uniqueKinds] = useState(() => [...new Set(kinds)]);

  const columnHelper = createColumnHelper<string>();
  const columns = [
    columnHelper.accessor((x) => x, {
      header: "Item use tags",
      cell: (i) => (
        <Link to={`/ryza3/item_use_tags/${i.getValue()}`}>{i.getValue()}</Link>
      ),
    }),
    columnHelper.accessor((x) => kinds.filter((c) => c === x).length, {
      header: "Count",
    }),
  ];

  return (
    <>
      <h1>Item use tags</h1>
      A list of all item use tags.
      <Grid data={uniqueKinds} columns={columns} />
    </>
  );
}
