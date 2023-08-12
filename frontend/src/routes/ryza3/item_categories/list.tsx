import Grid from "@/components/grid";
import items from "@/data/ryza3/items.json";
import { createColumnHelper } from "@tanstack/react-table";
import { useState } from "react";
import { Link } from "react-router-dom";

export default function ItemCategoriesList() {
  const [categories] = useState(() => items.flatMap((item) => item.cat));
  const [uniqueCategories] = useState(() => [...new Set(categories)]);

  const columnHelper = createColumnHelper<string>();
  const columns = [
    columnHelper.accessor((x) => x, {
      header: "Category",
      cell: (i) => (
        <Link to={`/ryza3/item_categories/${i.getValue()}`}>
          {i.getValue()}
        </Link>
      ),
    }),
    columnHelper.accessor((x) => categories.filter((c) => c === x).length, {
      header: "Count",
    }),
  ];

  return (
    <>
      <h1>Item categories</h1>
      A list of all item categories.
      <Grid data={uniqueCategories} columns={columns} />
    </>
  );
}
