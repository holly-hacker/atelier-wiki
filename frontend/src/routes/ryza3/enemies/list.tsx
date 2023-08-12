import enemies from "@/data/ryza3/enemies.json";
import enemies_texture from "@/data/ryza3/texture-atlasses/enemies.json";
import { EnemyLink, TextureAtlasImage } from "../utility_components/links";
import { createColumnHelper } from "@tanstack/react-table";
import { enemyDisplayName } from "../ryza3_data_util";
import Grid from "@/components/grid";

export default function EnemyList() {
  const columnHelper = createColumnHelper<(typeof enemies)[0]>();
  const columns = [
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

  return (
    <>
      <h1>Ryza 3 enemy list</h1>
      <div>{enemies.length} enemies found.</div>
      <Grid data={enemies} columns={columns} />
    </>
  );
}
