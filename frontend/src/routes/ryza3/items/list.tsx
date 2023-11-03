import { ItemLink, TextureAtlasImage } from "../utility_components/links";
import { ColumnDef, createColumnHelper } from "@tanstack/react-table";
import Grid from "@/components/grid";
import { ItemTypes } from "@/data/types/ryza3";
import { useContext, useState } from "react";
import { Ryza3Context, Ryza3Data } from "@/data/ryza3_data";

export default function ItemList() {
  const ryza3Data = useContext(Ryza3Context);
  const [showHidden, setShowHidden] = useState(false);

  const filtered_items = ryza3Data.items.filter(
    (i) => showHidden || i.name !== null,
  );

  return (
    <>
      <h1>Ryza 3 item list</h1>
      <small>Note: Operations on this page may be fairly slow</small>
      <div>
        <p>
          <input
            type="checkbox"
            checked={showHidden}
            onChange={() => setShowHidden(!showHidden)}
          />
          <label>Include unimplemented items</label>
        </p>

        <Grid data={filtered_items} columns={getColumnDefs(ryza3Data)} />
      </div>
    </>
  );
}

function getColumnDefs(
  ryza3Data: Ryza3Data,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
): ColumnDef<ItemTypes.Item, any>[] {
  const columnHelper = createColumnHelper<(typeof ryza3Data.items)[0]>();
  return [
    columnHelper.accessor("img_no", {
      header: "Image",
      cell: (i) => {
        return (
          <ItemLink item={i.row.original}>
            <TextureAtlasImage
              texture_atlas={ryza3Data.items_texture_atlas}
              texture_atlas_name="items"
              name={String(i.getValue())}
            />
          </ItemLink>
        );
      },
    }),
    columnHelper.accessor("name", {
      header: "Name",
      cell: (i) => <ItemLink item={i.row.original} />,
    }),
    columnHelper.accessor("price", { header: "Price" }),
    columnHelper.accessor("lv", { header: "Level" }),
    columnHelper.accessor("hp", { header: "HP" }),
    columnHelper.accessor("atk", { header: "Atk" }),
    columnHelper.accessor("def", { header: "Def" }),
    columnHelper.accessor("spd", { header: "Spd" }),
    columnHelper.accessor("tag", {
      header: "Tag",
      cell: (i) => <code>{i.getValue()}</code>,
    }),
    columnHelper.accessor("use_tag", {
      header: "Use Tag",
      cell: (i) => <code>{i.getValue()}</code>,
      filterFn: "equalsString",
    }),
    columnHelper.accessor("kind_tag", {
      header: "Kind Tag",
      cell: (i) => <code>{i.getValue()}</code>,
      filterFn: "equalsString",
    }),
    columnHelper.accessor("dlc", {
      header: "DLC",
      // NOTE: Ryza3 does not contain enemies that require multiple DLC
      cell: (i) => <code>{i.getValue()}</code>,
      filterFn: "equalsString",
    }),
  ];
}
