import enemies from "@/data/ryza3/enemies.json";
import enemies_texture from "@/data/ryza3/texture-atlasses/enemies.json";
import { EnemyLink, TextureAtlasImage } from "../utility_components/links";
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import React from "react";
import { enemyDisplayName } from "../ryza3_data_util";

export default function EnemyList() {
  const [data, _] = React.useState(() => [...enemies]);
  let columnHelper = createColumnHelper<(typeof enemies)[0]>();
  let columns = [
    columnHelper.accessor("img_no", {
      header: "Image",
      cell: (i) => (
        <EnemyLink enemy={i.row.original}>
          <TextureAtlasImage
            texture_atlas={enemies_texture}
            texture_atlas_name="enemies"
            name={String(i.getValue())}
          />
        </EnemyLink>
      ),
    }),
    columnHelper.accessor((x) => enemyDisplayName(x), {
      header: "Name",
      cell: (i) => <EnemyLink enemy={i.row.original} />,
    }),
    columnHelper.accessor("race_tag", {
      header: "Race",
      cell: (i) => <code>{i.getValue()}</code>,
    }),
    columnHelper.accessor("monster_tag", {
      header: "Tag",
      cell: (i) => <code>{i.getValue()}</code>,
    }),
    columnHelper.accessor("size", {
      header: "Size",
      cell: (i) => <code>{i.getValue()}</code>,
    }),
    columnHelper.accessor((x) => x.statusses.length, {
      header: "Instance count",
      cell: (i) => (
        <>
          {i.getValue()} {i.getValue() == 1 ? "instance" : "instances"}
        </>
      ),
    }),
    columnHelper.accessor((x) => Math.min(...x.statusses.map((x) => x.lv)), {
      header: "Min lvl",
    }),
    columnHelper.accessor((x) => Math.max(...x.statusses.map((x) => x.lv)), {
      header: "Max lvl",
    }),
    columnHelper.accessor("dlc", {
      header: "DLC",
      // NOTE: Ryza3 does not contain enemies that require multiple DLC
      cell: (i) => <code>{i.getValue() && i.getValue()[0]}</code>,
    }),
  ];

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <>
      <h1>Ryza 3 enemy list</h1>
      <div>
        {enemies.length} enemies found.
        <table>
          <thead>
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <th key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext(),
                        )}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody>
            {table.getRowModel().rows.map((row) => (
              <tr key={row.id}>
                {row.getVisibleCells().map((cell) => (
                  <td key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </>
  );
}
