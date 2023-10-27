import Grid from "@/components/grid";
import { Ryza3Context } from "@/data/ryza3_data";
import { createColumnHelper } from "@tanstack/react-table";
import { useContext } from "react";
import { ItemLink, TextureAtlasImage } from "../utility_components/links";
import { findItemByTag } from "../ryza3_data_util";

export default function PuniFeeding() {
  return (
    <>
      <h1>Puni feeding</h1>

      <h2>Puni species</h2>
      <PuniSpeciesSection />

      <h2>Unique rewards</h2>
      <p>Unique rewards when raising specific punis.</p>
      <PuniUniqueRewardsSection />
    </>
  );
}

function NumericRange({ range }: { range: [number, number] }) {
  if (range[0] == range[1]) {
    return <code>{range[0]}</code>;
  } else {
    return (
      <code>
        {range[0]}-{range[1]}
      </code>
    );
  }
}

function PuniSpeciesSection() {
  const ryza3Data = useContext(Ryza3Context);
  const species = ryza3Data.puni_feeding.species;
  const columnHelper = createColumnHelper<(typeof species)[0]>();

  function isFullRange(range: [number, number]) {
    return range[0] == 0 && range[1] == 100;
  }

  const columns = [
    columnHelper.accessor("image_no", {
      header: "Name",
      cell: (i) => {
        return (
          <TextureAtlasImage
            texture_atlas={ryza3Data.enemies_texture_atlas}
            texture_atlas_name="enemies"
            name={String(i.getValue())}
          />
        );
      },
    }),
    columnHelper.accessor("name", { header: "Name" }),
    columnHelper.accessor("character_tag", {
      header: "Character tag",
      cell: (i) => <code>{i.getValue()}</code>,
    }),
    columnHelper.group({
      header: "Stats",
      columns: [
        columnHelper.accessor("energy", {
          header: "Health",
          cell: (i) =>
            isFullRange(i.getValue()) ? (
              ""
            ) : (
              <NumericRange range={i.getValue()} />
            ),
        }),
        columnHelper.accessor("color", {
          header: "Luster",
          cell: (i) =>
            isFullRange(i.getValue()) ? (
              ""
            ) : (
              <NumericRange range={i.getValue()} />
            ),
        }),
        columnHelper.accessor("mood", {
          header: "Mood",
          cell: (i) =>
            isFullRange(i.getValue()) ? (
              ""
            ) : (
              <NumericRange range={i.getValue()} />
            ),
        }),
      ],
    }),
    columnHelper.group({
      header: "Rank",
      columns: [
        columnHelper.accessor("rank_e", {
          header: "E",
          cell: (i) => {
            const arr = i.getValue();
            return arr === null ? "" : <NumericRange range={arr} />;
          },
        }),
        columnHelper.accessor("rank_d", {
          header: "D",
          cell: (i) => {
            const arr = i.getValue();
            return arr === null ? "" : <NumericRange range={arr} />;
          },
        }),
        columnHelper.accessor("rank_c", {
          header: "C",
          cell: (i) => {
            const arr = i.getValue();
            return arr === null ? "" : <NumericRange range={arr} />;
          },
        }),
        columnHelper.accessor("rank_b", {
          header: "B",
          cell: (i) => {
            const arr = i.getValue();
            return arr === null ? "" : <NumericRange range={arr} />;
          },
        }),
        columnHelper.accessor("rank_a", {
          header: "A",
          cell: (i) => {
            const arr = i.getValue();
            return arr === null ? "" : <NumericRange range={arr} />;
          },
        }),
        columnHelper.accessor("rank_s", {
          header: "S",
          cell: (i) => {
            const arr = i.getValue();
            return arr === null ? "" : <NumericRange range={arr} />;
          },
        }),
      ],
    }),
  ];

  return (
    <>
      <Grid data={species} columns={columns} />
    </>
  );
}

function PuniUniqueRewardsSection() {
  const ryza3Data = useContext(Ryza3Context);
  const events = ryza3Data.puni_feeding.unique_events;
  const columnHelper = createColumnHelper<(typeof events)[0]>();

  const columns = [
    columnHelper.display({
      header: "Image",
      cell: (i) => {
        const item = findItemByTag(ryza3Data, i.row.original.item_tag);
        if (!item) return null;
        return (
          <ItemLink item={item}>
            <TextureAtlasImage
              texture_atlas={ryza3Data.items_texture_atlas}
              texture_atlas_name="items"
              name={String(item.img_no)}
            />
          </ItemLink>
        );
      },
    }),
    columnHelper.accessor("item_tag", {
      header: "Item",
      cell: (i) => {
        return <ItemLink item={i.getValue()} />;
      },
    }),
    columnHelper.accessor("condition.PuniSpecies", {
      header: "Puni species",
      cell: (i) => {
        return <code>{i.getValue() as string}</code>;
        // const this_species = i.getValue() as string;
        // if (!this_species) return null;
        // const species_num = this_species
        //   .substring("FEEDING_SPECIES_".length)
        //   .padStart(2, "0");
        // const species_obj = species.find(
        //   (s) =>
        //     s.character_tag.substring("CHARA_PUNI_FEEDING_".length) ===
        //     species_num,
        // );
        // return species_obj?.name ?? this_species;
      },
    }),
    columnHelper.accessor("condition.Energy", {
      header: "Health",
      cell: (i) => {
        const arr = i.getValue();
        return arr instanceof Array ? `${arr[0]}-${arr[1]}` : "";
      },
    }),
    columnHelper.accessor("condition.Color", {
      header: "Luster",
      cell: (i) => {
        const arr = i.getValue();
        return arr instanceof Array ? `${arr[0]}-${arr[1]}` : "";
      },
    }),
    columnHelper.accessor("condition.Mood", {
      header: "Mood",
      cell: (i) => {
        const arr = i.getValue();
        return arr instanceof Array ? `${arr[0]}-${arr[1]}` : "";
      },
    }),
  ];

  return (
    <>
      <Grid data={events} columns={columns} />
    </>
  );
}
