import items from "@/data/ryza3/items.json";
import items_texture from "@/data/ryza3/texture-atlasses/items.json";
import { ItemLink, TextureAtlasImage } from "../utility_components/links";
import { ColumnDef, createColumnHelper } from "@tanstack/react-table";
import Grid from "@/components/grid";
import types from "@/data/types/ryza3";
import { useState } from "react";

export default function ItemList() {
  const [showHidden, setShowHidden] = useState(false);

  const filtered_items = items.filter((i) => showHidden || i.name !== null);

  return (
    <>
      <h1>Ryza 3 item list</h1>
      <small>Note: Operations on this page may be fairly slow</small>
      <div>
        <p>
          {filtered_items.length}/{items.length} items shown.
        </p>

        <p>
          <input
            type="checkbox"
            checked={showHidden}
            onChange={() => setShowHidden(!showHidden)}
          />
          <label>Show unimplemented items</label>
        </p>

        <Grid data={filtered_items} columns={getColumnDefs()} />
      </div>
    </>
  );
}

function getColumnDefs(): ColumnDef<types.Item, any>[] {
  const columnHelper = createColumnHelper<(typeof items)[0]>();
  return [
    columnHelper.accessor("img_no", {
      header: "Image",
      cell: (i) => {
        return (
          <ItemLink item={i.row.original}>
            <TextureAtlasImage
              texture_atlas={items_texture}
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
    }),
    columnHelper.accessor("kind_tag", {
      header: "Kind Tag",
      cell: (i) => <code>{i.getValue()}</code>,
    }),
    columnHelper.accessor("dlc", {
      header: "DLC",
      // NOTE: Ryza3 does not contain enemies that require multiple DLC
      cell: (i) => <code>{i.getValue()}</code>,
    }),
  ];
}
