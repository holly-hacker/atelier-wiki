import Grid from "@/components/grid";
import { SophieContext, SophieData } from "@/data/sophie_data";
import types from "@/data/types/sophie";
import { ColumnDef, createColumnHelper } from "@tanstack/react-table";
import { useContext } from "react";
import { ItemLink } from "../utility_components/links";

export default function ItemList(): JSX.Element {
  const sophieData = useContext(SophieContext);
  return (
    <>
      <h1>Sophie item list</h1>
      <div>
        <Grid
          data={sophieData.items.filter(isImplementedItem)}
          columns={getColumnDefs(sophieData)}
        />
      </div>
    </>
  );
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function getColumnDefs(sophieData: SophieData): ColumnDef<types.Item, any>[] {
  const columnHelper = createColumnHelper<(typeof sophieData.items)[0]>();
  return [
    columnHelper.accessor("name", {
      header: "Name",
      cell: (i) => <ItemLink item={i.row.original} />,
    }),
    columnHelper.accessor("cost", { header: "Price" }),
    columnHelper.accessor("level", { header: "Level" }),
    columnHelper.accessor("tag", {
      header: "Tag",
      cell: (i) => <code>{i.getValue()}</code>,
    }),
    columnHelper.accessor("base", {
      header: "Item Kind",
      cell: (i) => <code>{i.getValue()}</code>,
      filterFn: "equalsString",
    }),
    columnHelper.accessor("use_type", {
      header: "Use Type",
      cell: (i) => <code>{i.getValue()}</code>,
      filterFn: "equalsString",
    }),
    columnHelper.accessor("color", {
      header: "Color",
      cell: (i) => <code>{i.getValue()}</code>,
      filterFn: "equalsString",
    }),
  ];
}

function isImplementedItem(item: types.Item): boolean {
  const japaneseDigits = [
    "０",
    "１",
    "２",
    "３",
    "４",
    "５",
    "６",
    "７",
    "８",
    "９",
  ];

  if (japaneseDigits.find((char) => item.name.includes(char)) !== undefined)
    return false;

  return true;
}
