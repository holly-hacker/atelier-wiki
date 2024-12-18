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
  const species = ryza3Data.puni_feeding.species;
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
    columnHelper.accessor("condition.species", {
      header: "Puni species",
      cell: (i) => {
        const this_species_tag = i.getValue() as string | null;

        if (!this_species_tag) return null;
        const species_idx = this_species_tag
          .substring("FEEDING_SPECIES_".length)
          .padStart(2, "0");
        const this_species = species[Number(species_idx)];

        return (
          <div
            style={{
              display: "flex",
              flexDirection: "row",
              alignItems: "center",
            }}
          >
            <TextureAtlasImage
              texture_atlas={ryza3Data.enemies_texture_atlas}
              texture_atlas_name="enemies"
              name={String(this_species.image_no)}
            />
            <span style={{}}>{this_species.name}</span>
          </div>
        );
      },
    }),
    columnHelper.accessor("condition.min", {
      header: "Health",
      cell: (i) => {
        if (i.row.original.condition.type != "Energy") {
          return null;
        }

        const min = i.getValue();
        const max = i.row.original.condition.max;
        return `${min}-${max}`;
      },
    }),
    columnHelper.accessor("condition.min", {
      header: "Luster",
      cell: (i) => {
        if (i.row.original.condition.type != "Color") {
          return null;
        }

        const min = i.getValue();
        const max = i.row.original.condition.max;
        return `${min}-${max}`;
      },
    }),
    columnHelper.accessor("condition.min", {
      header: "Mood",
      cell: (i) => {
        if (i.row.original.condition.type != "Mood") {
          return null;
        }

        const min = i.getValue();
        const max = i.row.original.condition.max;
        return `${min}-${max}`;
      },
    }),
  ];

  return (
    <>
      <Grid data={events} columns={columns} />
    </>
  );
}
