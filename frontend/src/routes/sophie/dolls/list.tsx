import Grid from "@/components/grid";
import { SophieContext } from "@/data/sophie_data";
import { createColumnHelper } from "@tanstack/react-table";
import { useContext } from "react";
import { ItemLink, TextureAtlasImage } from "../utility_components/links";
import type SophieTypes from "@/data/types/sophie.d.ts";
import { findItemByTag } from "../sophie_data_util";

export default function DollListPage() {
  return (
    <>
      <h1>Doll making</h1>
      <DollList />

      <DollMaterialListSection />
    </>
  );
}

function getDollRequirements(doll: SophieTypes.Doll) {
  const requirement_types = [
    ["cute_min", "Cute >="],
    ["cute_max", "Cute <="],
    ["wise_min", "Wise >="],
    ["wise_max", "Wise <="],
    ["brave_min", "Brave >="],
    ["brave_max", "Brave <="],
    ["fool_min", "Fool >="],
    ["fool_max", "Fool <="],
  ];

  const requirements = requirement_types
    .map(([key, name]) => {
      const val = (doll as unknown as Record<string, number>)[key];
      if (val != 0) {
        return `${name} ${val}`;
      } else {
        return null;
      }
    })
    .filter((v) => v != null);

  if (requirements.length == 0) {
    return "";
  } else {
    return requirements.join(", ");
  }
}

function DollList() {
  const sophieData = useContext(SophieContext);
  const dolls = sophieData.dolls.filter(
    (v) => v.chara_base_tag != "CHARA_BASE_NONE",
  );

  const columnHelper = createColumnHelper<(typeof sophieData.dolls)[0]>();
  const columns = [
    columnHelper.accessor("no", {
      header: "Number",
    }),
    columnHelper.accessor("name", { header: "Name" }),
    columnHelper.display({
      header: "Requirements",
      cell: (i) => <code>{getDollRequirements(i.row.original)}</code>,
    }),
    columnHelper.accessor("doll_hp", {
      header: "HP",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("doll_mp", {
      header: "MP",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("doll_lp", {
      header: "LP",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("doll_atk", {
      header: "Atk.",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("doll_def", {
      header: "Def.",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("doll_spd", {
      header: "Spd.",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("doll_dmg_min", {
      header: "Dmg Min",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("doll_dmg_max", {
      header: "Dmg Max",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor(
      (c) =>
        c.doll_hp +
        c.doll_mp +
        c.doll_lp +
        c.doll_atk +
        c.doll_def +
        c.doll_spd +
        c.doll_dmg_min +
        c.doll_dmg_max,
      {
        header: "Sum of stats",
        cell: (i) => <NonZeroNumber value={i.getValue()} />,
      },
    ),
    columnHelper.accessor("dlc_tag", {
      header: "DLC",
      cell: (i) => <code>{i.getValue()?.substring("DLC_".length)}</code>,
      filterFn: "equalsString",
    }),
  ];

  return (
    <>
      <h1>Doll list</h1>
      <Grid data={dolls} columns={columns} />
    </>
  );
}

function NonZeroNumber({ value }: { value: number }) {
  if (value != 0) {
    return <>{value}</>;
  } else {
    return <></>;
  }
}

function DollMaterialListSection() {
  return (
    <>
      <h2>Doll material list</h2>
      <h3>Elixer</h3>
      <DollMaterialList category="ELIXIL" />
      <h3>Thread</h3>
      <DollMaterialList category="THREAD" />
      <h3>Gem</h3>
      <DollMaterialList category="JEWEL" />
      <h3>Secret Power</h3>
      <DollMaterialList category="MYSTIC" />
    </>
  );
}

function DollMaterialList({ category }: { category: string }) {
  const sophieData = useContext(SophieContext);

  const items = sophieData.items.filter(
    (v) => v.category.indexOf(`ITEM_CATEGORY_${category}`) !== -1,
  );

  const columnHelper = createColumnHelper<(typeof items)[0]>();
  const columns = [
    columnHelper.accessor("image_no", {
      header: "Image",
      cell: (i) => {
        return (
          <ItemLink item={i.row.original}>
            <TextureAtlasImage
              texture_atlas={sophieData.items_texture_atlas}
              texture_atlas_name="items"
              name={String(i.row.original.image_no)}
            />
          </ItemLink>
        );
      },
    }),
    columnHelper.accessor("name", {
      header: "Name",
      cell: (i) => <ItemLink item={i.row.original} />,
    }),
    columnHelper.accessor("cost", { header: "Price" }),
    columnHelper.accessor("level", { header: "Level" }),
    columnHelper.accessor("tag", {
      header: "Tag",
      cell: (i) => <code>{i.getValue().substring("ITEM_".length)}</code>,
    }),
    columnHelper.accessor("base", {
      header: "Item Kind",
      cell: (i) => <code>{i.getValue().substring("ITEM_KIND_".length)}</code>,
      filterFn: "equalsString",
    }),
    columnHelper.accessor("doll_tendency_cute", { header: "Cute" }),
    columnHelper.accessor("doll_tendency_wise", { header: "Wise" }),
    columnHelper.accessor("doll_tendency_brave", { header: "Brave" }),
    columnHelper.accessor("doll_tendency_fool", { header: "Fool" }),
  ];

  return <Grid data={items} columns={columns} />;
}
