import {
  ColumnDef,
  SortingState,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { useState } from "react";

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
  const table = useReactTable({
    data,
    columns,
    state: {
      sorting,
    },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  });

  return (
    <table>
      <thead>
        {table.getHeaderGroups().map((headerGroup) => (
          <tr key={headerGroup.id}>
            {headerGroup.headers.map((header) => (
              <th key={header.id}>
                {header.isPlaceholder ? null : (
                  <div
                    {...{
                      style: { cursor: "pointer", userSelect: "none" },
                      onClick: header.column.getToggleSortingHandler(),
                    }}
                  >
                    {flexRender(
                      header.column.columnDef.header,
                      header.getContext(),
                    )}
                    {{
                      asc: " ↑",
                      desc: " ↓",
                    }[header.column.getIsSorted() as string] ?? null}
                  </div>
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
