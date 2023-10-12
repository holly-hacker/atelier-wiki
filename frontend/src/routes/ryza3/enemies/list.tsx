import { EnemyLink, TextureAtlasImage } from "../utility_components/links";
import { createColumnHelper } from "@tanstack/react-table";
import { enemyDisplayName } from "../ryza3_data_util";
import Grid from "@/components/grid";
import { useContext } from "react";
import { Ryza3Context } from "@/data/ryza3_data";

export default function EnemyList() {
  const ryza3Data = useContext(Ryza3Context);
  const columnHelper = createColumnHelper<(typeof ryza3Data.enemies)[0]>();
  const columns = [
    columnHelper.accessor("img_no", {
      header: "Image",
      cell: (i) => (
        <EnemyLink enemy={i.row.original}>
          <TextureAtlasImage
            texture_atlas={ryza3Data.enemies_texture_atlas}
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
      filterFn: "equalsString",
    }),
    columnHelper.accessor("monster_tag", {
      header: "Tag",
      cell: (i) => <code>{i.getValue()}</code>,
    }),
    columnHelper.accessor("size", {
      header: "Size",
      cell: (i) => <code>{i.getValue()}</code>,
      filterFn: "equalsString",
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
      cell: (i) => <code>{i.getValue()}</code>,
      filterFn: "equalsString",
    }),
  ];

  return (
    <>
      <h1>Ryza 3 enemy list</h1>
      <div>{ryza3Data.enemies.length} enemies found.</div>
      <Grid data={ryza3Data.enemies} columns={columns} />
    </>
  );
}
