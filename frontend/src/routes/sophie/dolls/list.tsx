import Grid from "@/components/grid";
import { SophieContext } from "@/data/sophie_data";
import { createColumnHelper } from "@tanstack/react-table";
import { useContext } from "react";
import { ItemLink } from "../utility_components/links";

export default function DollListPage() {
  return (
    <>
      <h1>Doll making</h1>
      <DollList />

      <DollMaterialListSection />
    </>
  );
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
    columnHelper.accessor("cute_min", {
      header: "Cute Min",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("cute_max", {
      header: "Cute Max",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("wise_min", {
      header: "Wise Min",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("wise_max", {
      header: "Wise Max",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("brave_min", {
      header: "Brave Min",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("brave_max", {
      header: "Brave Max",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("fool_min", {
      header: "Fool Min",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("fool_max", {
      header: "Fool Max",
      cell: (i) => <NonZeroNumber value={i.getValue()} />,
    }),
    columnHelper.accessor("doll_hp", { header: "HP" }),
    columnHelper.accessor("doll_mp", { header: "MP" }),
    columnHelper.accessor("doll_lp", { header: "LP" }),
    columnHelper.accessor("doll_atk", { header: "Atk." }),
    columnHelper.accessor("doll_def", { header: "Def." }),
    columnHelper.accessor("doll_spd", { header: "Spd." }),
    columnHelper.accessor("doll_dmg_min", { header: "Dmg Min" }),
    columnHelper.accessor("doll_dmg_max", { header: "Dmg Max" }),
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

function NonZeroNumber({ value }: { value: number }) {
  if (value != 0) {
    return <>{value}</>;
  } else {
    return <></>;
  }
}
