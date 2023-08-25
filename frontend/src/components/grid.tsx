import {
  Column,
  ColumnDef,
  ColumnFiltersState,
  SortingState,
  flexRender,
  getCoreRowModel,
  getFacetedUniqueValues,
  getFilteredRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { useMemo, useState } from "react";

/**
 * Create a standard grid based on `react-table`.
 *
 * Use `react-table`'s `createColumnHelper` to create the `columns` array:
 * ```tsx
 * let columnHelper = createColumnHelper<Person>();
 * let columns = [
 *   columnHelper.accessor("name", {}),
 *   columnHelper.accessor("age", {}),
 *   columnHelper.accessor("image", { cell: i => <img src={i.getValue()} /> }),
 * ];
 * return <Grid colums={columns} data={data} />;
 * ```
 */
export default function Grid<TData>({
  columns,
  data,
}: {
  // use explicit `any` because the second arg seems to be what's printed to the DOM
  // we don't want to limit this, the error messages would suck anyway
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  columns: ColumnDef<TData, any>[];
  data: TData[];
}) {
  const [sorting, setSorting] = useState<SortingState>([]);

  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);

  const table = useReactTable({
    data,
    columns,
    state: {
      sorting,
      columnFilters,
    },
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getFacetedUniqueValues: getFacetedUniqueValues(),
  });

  return (
    <table>
      <thead>
        {table.getHeaderGroups().map((headerGroup) => (
          <tr key={headerGroup.id}>
            {headerGroup.headers.map((header) => (
              <th key={header.id}>
                {header.isPlaceholder ? null : (
                  // header
                  <>
                    <div
                      {...{
                        style: { cursor: "pointer", userSelect: "none" },
                        onClick: header.column.getToggleSortingHandler(),
                      }}
                    >
                      {/* header value (can be arbitrary html) */}
                      {flexRender(
                        header.column.columnDef.header,
                        header.getContext(),
                      )}

                      {/* sorting indicator */}
                      {{
                        asc: " ↑",
                        desc: " ↓",
                      }[header.column.getIsSorted() as string] ?? null}
                    </div>

                    {header.column.getCanFilter() &&
                    header.column.columnDef.filterFn == "equalsString" ? (
                      <div>
                        <EqualsStringFilter column={header.column} />
                      </div>
                    ) : null}
                  </>
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
  );
}

function EqualsStringFilter<TData>({
  column,
}: {
  column: Column<TData, unknown>;
}) {
  const facetedUniqueValues = column.getFacetedUniqueValues();
  const uniqueValues = useMemo(() => {
    return Array.from(facetedUniqueValues.keys()).sort();
  }, [facetedUniqueValues]);

  // TODO: consider react-select to allow selecting multiple values
  return (
    <select
      style={{ width: "90%" }}
      onChange={(e) => column.setFilterValue(e.target.value || undefined)}
    >
      <option value="">All</option>
      {uniqueValues
        .filter((v) => v !== "") // empty string is used for "no filter"
        .map((v) => (
          <option key={v} value={v}>
            {v}
          </option>
        ))}
    </select>
  );
}
