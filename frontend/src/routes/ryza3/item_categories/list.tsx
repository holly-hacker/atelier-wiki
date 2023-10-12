import Grid from "@/components/grid";
import { createColumnHelper } from "@tanstack/react-table";
import { useContext, useState } from "react";
import { CategoryLink } from "../utility_components/links";
import { Ryza3Context } from "@/data/ryza3_data";

export default function ItemCategoriesList() {
  const ryza3Data = useContext(Ryza3Context);
  const [categories] = useState(() =>
    ryza3Data.items.flatMap((item) => item.cat),
  );
  const [uniqueCategories] = useState(() => [...new Set(categories)]);

  const columnHelper = createColumnHelper<string>();
  const columns = [
    columnHelper.accessor((x) => x, {
      header: "Category",
      cell: (i) => <CategoryLink category_tag={i.getValue()} />,
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
